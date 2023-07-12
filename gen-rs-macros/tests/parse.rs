// use crate::ast;


// #[test]
// fn test_parse_output() {
//     use quote::quote;
//     use syn::{parse2, parse_quote};

//     let model_ast = parse2::<ModelAst>(quote!(
//         mod grass;
//         use ferric::distributions::Bernoulli;

//         let rain : bool ~ Bernoulli::new( 0.2 );

//         let sprinkler : bool ~
//             if rain {
//                 Bernoulli::new( 0.01 )
//             } else {
//                 Bernoulli::new( 0.4 )
//             };

//         let grass_wet : bool ~ Bernoulli::new(
//             if sprinkler && rain { 0.99 }
//             else if sprinkler && !rain { 0.9 }
//             else if !sprinkler && rain { 0.8 }
//             else { 0.0 }
//         );

//         observe grass_wet;
//         query rain;
//         query sprinkler;
//     ))
//     .unwrap();

//     let exp_model_name: Ident = parse_quote!(grass);
//     assert_eq!(model_ast.model_ident, exp_model_name);

//     let exp_use_exprs: &Expr = &parse_quote!(ferric::distributions::Bernoulli);
//     assert_eq!(model_ast.use_exprs[0], *exp_use_exprs);
//     assert_eq!(model_ast.use_exprs.len(), 1);

//     let exp_var_name: Ident = parse_quote!(rain);
//     let exp_type_name: Ident = parse_quote!(bool);
//     let exp_dependency: &Expr = &parse_quote!(Bernoulli::new(0.2));
//     assert_eq!(model_ast.stmts[0].var_ident, exp_var_name);
//     assert_eq!(model_ast.stmts[0].type_ident, exp_type_name);
//     assert_eq!(model_ast.stmts[0].dependency, *exp_dependency);
//     assert_eq!(model_ast.stmts.len(), 3);

//     let exp_queryies_0: Ident = parse_quote!(rain);
//     let exp_queryies_1: Ident = parse_quote!(sprinkler);
//     assert_eq!(model_ast.queries, [exp_queryies_0, exp_queryies_1]);

//     let exp_observes_0: Ident = parse_quote!(grass_wet);
//     assert_eq!(model_ast.observes, [exp_observes_0]);
// }
