#![crate_name = "modppl_macros"]
#![crate_type = "proc-macro"]
#![warn(non_camel_case_types)]

extern crate proc_macro;


use syn::parse_macro_input;
use syn::{Pat,PatType,ItemFn,FnArg,ReturnType};
use syn::visit_mut::VisitMut;
use quote::quote;

mod address;
use address::ReplaceAddressedCalls;

mod proposal;
use proposal::ty_is_weak_trace_ref;


#[proc_macro]
pub fn dyngen(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // embed(TokenStream::from(input)).into()
    let input_fn = parse_macro_input!(input as ItemFn);

    // Extracting types, identifiers, and mutability from the function arguments
    let arg_details: Vec<_> = input_fn.sig.inputs.iter().map(|fn_arg| {
        match fn_arg {
            FnArg::Typed(PatType { pat, ty, .. }) => {
                // Extract the identifier and check for mutability
                let (ident, is_mut) = match **pat {
                    Pat::Ident(ref pat_ident) => {
                        (pat_ident.ident.clone(), pat_ident.mutability.is_some())
                    },
                    _ => panic!("Expected function arguments to have identifiers"),
                };

                // Extract the type
                let arg_type = ty.clone();

                (ident, is_mut, arg_type)
            },
            _ => panic!("Expected typed arguments"),
        }
    }).collect();

    // Unpacking the vectors of identifiers, mutabilities, and types
    let (arg_idents, mutabilities, arg_tys): (Vec<_>, Vec<_>, Vec<_>) = arg_details.into_iter()
        .fold((vec![], vec![], vec![]), |mut acc, (ident, is_mut, ty)| {
            acc.0.push(ident);
            acc.1.push(is_mut);
            acc.2.push(ty);
            acc
        });
    
    let args_idents_tuple: proc_macro2::TokenStream; 
    let args_ty_tuple: proc_macro2::TokenStream; 
    if arg_tys.len() > 0 && ty_is_weak_trace_ref(&arg_tys[0]) {
        let trace_ident = &arg_idents[0];
        let trace_ty = &arg_tys[0];
        let mut trace_ident_token = quote! { #trace_ident };
        if mutabilities[0] {
            trace_ident_token = quote! { mut #trace_ident };
        }
        
        let proposal_arg_details = arg_idents.iter().zip(mutabilities.iter()).skip(1).map(|(ident, &is_mut)| {
            if is_mut {
                quote! { mut #ident }
            } else {
                quote! { #ident }
            }
        }).collect::<Vec<_>>();
    
        args_idents_tuple = quote! { (#trace_ident_token, (#(#proposal_arg_details),*)) };
        let proposal_arg_tys = &arg_tys[1..].iter().collect::<Vec<_>>();
        args_ty_tuple = quote! { (#trace_ty, (#(#proposal_arg_tys),*)) };
    } else {
        let arg_idents_tokens = arg_idents.iter().zip(mutabilities.iter()).map(|(ident, &is_mut)| {
            if is_mut {
                quote! { mut #ident }
            } else {
                quote! { #ident }
            }
        }).collect::<Vec<_>>();
    
        args_idents_tuple = quote! { (#(#arg_idents_tokens),*) };
        args_ty_tuple = quote! { (#(#arg_tys),*) };
    }

    // Retrieve the return type
    let ret_ty = match input_fn.sig.output {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, ref ty) => quote! { #ty },
    };

    // Modify the function name by appending an underscore
    let original_ident = &input_fn.sig.ident;
    let new_ident = syn::Ident::new(&format!("__{}", original_ident), original_ident.span());

    let mut fn_body = input_fn.block;

    let handler_type = quote! { DynGenFnHandler<#args_ty_tuple, #ret_ty> };
    let genfn_type = quote! { DynGenFn<#args_ty_tuple, #ret_ty> };

    ReplaceAddressedCalls.visit_block_mut(&mut fn_body);

    // Reconstruct the function with the new argument and modified name
    quote! {
        fn #new_ident(__g: &mut #handler_type, __args: #args_ty_tuple) -> #ret_ty {
            let #args_idents_tuple: #args_ty_tuple = __args;
            #fn_body
        }
        pub const #original_ident: #genfn_type = DynGenFn { func: #new_ident };
    }.into()
}