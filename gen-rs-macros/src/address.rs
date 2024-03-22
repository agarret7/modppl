use syn::visit_mut::{self,VisitMut};
use syn::{Expr,ExprBinary,ExprCall,BinOp,Lit};
use syn::parse_quote;

pub struct ReplaceAddressedCalls;

impl VisitMut for ReplaceAddressedCalls {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        if let Expr::Binary(ExprBinary{left, op, right, ..}) = node {
            match op {
                BinOp::RemAssign(_) => {
                    if let Expr::Call(ExprCall{func: dist, args, ..}) = &**left {
                        *node = parse_quote!(__g.sample_at(&#dist, (#args), #right));
                    }
                }
                BinOp::DivAssign(_) => {
                    if let Expr::Call(ExprCall{func: gen_fn, args, ..}) = &**left {
                        *node = parse_quote!(__g.trace_at(&#gen_fn, (#args), #right));
                    }
                }
                _ => { }
            }
        }
        visit_mut::visit_expr_mut(self, node);
    }
}
