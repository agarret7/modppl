use proc_macro2::TokenStream;
use syn::parse2;
use quote::quote;

// mod ast;
// use ast::ModelAst;
mod dag;


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