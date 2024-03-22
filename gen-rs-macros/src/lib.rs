#![crate_name = "gen_rs_macros"]
#![crate_type = "proc-macro"]
#![warn(non_camel_case_types)]

extern crate proc_macro;


use syn::parse_macro_input;
use syn::{Pat,PatType,Ident,ItemFn,FnArg,ReturnType};
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

    // Extracting types and identifiers from the function arguments
    let (arg_idents, arg_tys) = input_fn.sig.inputs.iter().map(|fn_arg| {
        match fn_arg {
            FnArg::Typed(PatType { pat, ty, .. }) => {
                // Extract the identifier
                let ident = match **pat {
                    Pat::Ident(ref pat_ident) => pat_ident.ident.clone(),
                    _ => panic!("Expected function arguments to have identifiers"),
                };

                // Extract the type
                let arg_type = ty.clone();

                (ident, arg_type)
            },
            _ => panic!("Expected typed arguments"),
        }
    }).unzip::<_, _, Vec<_>, Vec<_>>();

    let args_idents_tuple: proc_macro2::TokenStream; 
    let args_ty_tuple: proc_macro2::TokenStream; 
    if arg_tys.len() > 0 && ty_is_weak_trace_ref(&arg_tys[0]) {
        // Prepare a tuple of proposal argument identifiers
        let trace_ident = &arg_idents[0];
        let proposal_arg_idents = arg_idents[1..].iter().collect::<Vec<_>>();
        args_idents_tuple = quote! { (#trace_ident, (#(#proposal_arg_idents),*)) };

        // Prepare a tuple of proposal argument types for DynGenFnHandler
        let trace_ty = &arg_tys[0];
        let proposal_arg_tys = &arg_tys[1..].iter().collect::<Vec<_>>();
        args_ty_tuple = quote! { (#trace_ty, (#(#proposal_arg_tys),*)) };
    } else {
        // Prepare a tuple of argument identifiers
        args_idents_tuple = quote! { (#(#arg_idents),*) };

        // Prepare a tuple of argument types for DynGenFnHandler
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