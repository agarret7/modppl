#![feature(unboxed_closures)]
#![feature(tuple_trait)]
#![feature(fn_traits)]

use proc_macro2::TokenStream;
use syn::parse_quote;
use syn::Type;
use quote::quote;
use proc_macro2::{Ident,Span};

// mod ast;
// use ast::ModelAst;
mod dag;
mod print_ir;
use dag::StaticIRBuilder;

//gen!{fn bar(y: f64, b: f64) -> f64 {
//     let u = y + b;
//     let v ~ normal(0., 1.);
//     let z = u + v;
//     return z;
// }}

#[test]
fn test_repr() {
    let mut builder = StaticIRBuilder::new();
    let y = builder.add_argument_node(Ident::new("y", Span::call_site()), parse_quote!(f64));
    let b = builder.add_argument_node(Ident::new("b", Span::call_site()), parse_quote!(f64));
    let u = builder.add_rust_node(parse_quote!(|y, b| { y + b }), vec![y, b], Some(Ident::new("u", Span::call_site())), None);
    let zero = builder.add_constant_node(parse_quote!(0.), None, None);
    let one = builder.add_constant_node(parse_quote!(1.), None, None);
    let v = builder.add_addr_node(parse_quote!(normal), vec![zero, one], "v", Some(Ident::new("v", Span::call_site())), None);
    let z = builder.add_rust_node(parse_quote!(|u, v| { u + v }), vec![u, v.clone()], Some(Ident::new("z", Span::call_site())), None);
    builder.set_return_node(z);
    let ir = builder.build();
    println!("{}", ir);
    assert!(true);
}

// #[proc_macro_attribute]
// pub fn Gen(_: proc_macro::TokenStream, _fn: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     make_model_inner(TokenStream::from(_fn)).into()
// }

// fn make_model_inner(input: TokenStream) -> TokenStream {
//     // parse the token stream into an AST (abstract syntax tree)
//     let ast = parse2::<ModelAst>(input);
//     let ast = match ast {
//         Ok(data) => data,
//         Err(err) => {
//             return err.to_compile_error();
//         }
//     };
//     // analyze the AST and produce an IR (intermediate representation)
//     // let ir = analyze(ast);
//     // let ir = match ir {
//     //     Ok(data) => data,
//     //     Err(err) => {
//     //         return err.to_compile_error();
//     //     }
//     // };
//     // codegen(ir)
//     quote!(1)
// }