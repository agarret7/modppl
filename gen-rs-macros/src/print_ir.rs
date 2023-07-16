use crate::dag::{StaticIR, StaticIRNode, ArgumentNode, RustNode, RandomChoiceNode};
use syn::Ident;
use quote::ToTokens;
use std::fmt::{Display,Formatter,Result};


fn write_in_punc(f: &mut Formatter<'_>, n: usize, N: usize) {
    if n < N - 1 {
        write!(f, ", ");
    } else {
        write!(f, ")");
    }
}

impl Display for StaticIR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "== Static IR ==");
        let args = String::new();
        write!(f, "Arguments: (");
        let arg_nodes = self.arg_nodes();
        for (n, arg) in arg_nodes.iter().enumerate() {
            write!(f, "{}", arg);
            write_in_punc(f, n, arg_nodes.len());
        }
        writeln!(f);
        for node in self.nodes.iter() {
            self.arg_nodes().contains(node) && continue;
            write!(f, "  ");
            write!(f, "{}", node);
            writeln!(f);
        }
        write!(f, "  return {}", self.return_id)
    }
}

impl Display for StaticIRNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            StaticIRNode::Arg(node) => { write!(f, "{}", node) },
            StaticIRNode::Rust(node) => { write!(f, "{}", node) },
            StaticIRNode::RandomChoice(node) => { write!(f, "{}", node) }
        }
    }
}

impl Display for ArgumentNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}: ", self.name);
        write!(f, "{}", self.ty.to_token_stream())
    }
}

impl Display for RustNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}: ", self.name);
        write!(f, "{} = ", self.ty.to_token_stream());
        write!(f, "({})", self.func.to_token_stream());
        write!(f, "(");
        if self.inputs.len() > 0 {
            for (n, inp) in self.inputs.iter().enumerate() {
                write!(f, "{}", inp);
                write_in_punc(f, n, self.inputs.len());
            }
        } else {
            write!(f, ")");
        }
        write!(f, "")
    }
}

impl Display for RandomChoiceNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}: ", self.name);
        write!(f, "{} = ", self.ty.to_token_stream());
        write!(f, "{{\"{}\"}} ~ ", self.addr);
        write!(f, "{}", self.dist.to_token_stream());
        write!(f, "(");
        if self.inputs.len() > 0 {
            for (n, inp) in self.inputs.iter().enumerate() {
                write!(f, "{}", inp);
                write_in_punc(f, n, self.inputs.len());
            }
        } else {
            write!(f, ")");
        }
        write!(f, "")
    }
}