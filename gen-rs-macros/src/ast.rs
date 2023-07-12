use syn::parse::{Parse, ParseStream, Result};
use syn::token::RArrow;
use syn::{Error, Expr, Ident, ItemFn, FnArg, Token, PatType, Type};


pub enum StmtAst {
    Decl { ident: Ident, init: LocalInit, dependencies: Vec<String> },
    TraceAt { address: String, dist_ident: Ident, dist_args: Vec<Expr> }
}

#[derive(Debug)]
pub struct ModelAst {
    pub model_ident: Ident,
    pub model_args: Vec<PatType>,
    pub model_ret_type: Option<Type>,
    // pub stmts: Vec<StmtAst>,
}

impl Parse for ModelAst {
    fn parse(input: ParseStream) -> Result<Self> {
        let fn_decl: ItemFn = input.parse()?;

        let mut model_args = Vec::new();
        // let mut model_arg_types = Vec::new();

        for arg in fn_decl.sig.inputs.into_iter() {
            match arg {
                FnArg::Typed(ptype) => {
                    model_args.push(ptype);
                },
                _ => { return Err(input.error("all model arguments must be typed")) }
            };
        }

        let model_ret_type = match fn_decl.sig.output {
            Default => None,
            // Type(r, ty) => { return Err(input.error("all model arguments must be typed")) }
        };

        Ok(ModelAst {
            model_ident: fn_decl.sig.ident,
            model_args: model_args,
            model_ret_type: model_ret_type
        })
    }
}

#[test]
fn test_parse_output() {
    use quote::quote;
    use syn::{parse2, parse_quote};

    let model_ast = parse2::<ModelAst>(quote!(
        fn biased_coin(p_heads: f64, p_obs: f64) -> bool {
            let heads ~ bernoulli(p_heads);
            let obs = {"observation"} ~ bernoulli(if heads { p_obs } else { 1 - p_obs });
            obs
        }
    )).unwrap();

    dbg!(model_ast.model_ident);
    dbg!(model_ast.model_args);
}