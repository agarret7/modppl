use std::collections::{HashSet,HashMap};
use common_macros::{hash_set,hash_map};

use gen_rs::{modeling::dists::Distribution,Addr};
use syn::{Expr,ExprBlock};
use proc_macro2::{Ident};


// heap-allocated AST

trait StaticIRNode { }

enum TypeIR {         // eg.
    Var(Ident),       // i64
    Expr(Expr),       // some_crate::types_2d::Bounds
    Other(ExprBlock)  // ???
}

impl StaticIRNode for ArgumentNode { }
pub struct ArgumentNode {
    name: Ident,
    typ: TypeIR
}

// basically a wrapper for `syn::ExprCall`
impl StaticIRNode for RustNode { }
pub struct RustNode {
    func: Box<dyn Fn(Vec<Box<dyn StaticIRNode>>) -> dyn StaticIRNode>,
    inputs: Vec<Box<dyn StaticIRNode>>,
    name: Ident,
    typ: TypeIR
}

impl StaticIRNode for RandomChoiceNode { }
pub struct RandomChoiceNode {
    dist: Ident,
    inputs: Vec<Box<dyn StaticIRNode>>,
    addr: Addr,
    name: Ident,  // replace with "Gensym"
    typ: TypeIR
}

pub struct StaticIR {
    nodes: Vec<Box<dyn StaticIRNode>>,
    arg_nodes: Vec<ArgumentNode>,
    choice_nodes: Vec<RandomChoiceNode>,
    rust_nodes: Vec<RustNode>,
    return_node: Box<dyn StaticIRNode>
}

pub struct StaticIRBuilder {
    nodes: Vec<Box<dyn StaticIRNode>>,
    node_set: HashSet<Box<dyn StaticIRNode>>,
    arg_nodes: Vec<ArgumentNode>,
    choice_nodes: Vec<RandomChoiceNode>,
    rust_nodes: Vec<RustNode>,
    return_node: Option<Box<dyn StaticIRNode>>,
    vars: HashSet<Ident>,
    addrs_to_choice_nodes: HashMap<Addr, RandomChoiceNode>
}

impl StaticIRBuilder {

    fn check_unique_var(&self, name: Ident) {
        if self.vars.contains(&name) {
            panic!("Variable name {} is not unique", name);
        }
    }

    fn check_inputs_exist(&self, input_nodes: Vec<Box<dyn StaticIRNode>>) {
        for input_node in input_nodes.into_iter() {
            if !self.node_set.contains(input_node) {
                panic!("Node {} was not previously added to the IR", input_node);
            }
        }
    }

    fn check_addr_unique(&self, addr: Addr) {
        if self.addrs_to_choice_nodes.contains_key(addr) {
            panic!("Address {} was not unique", addr);
        }
    }

    fn add_node(&mut self, node: impl StaticIRNode) -> impl StaticIRNode {
        self.nodes.push(Box::new(node));
        self.node_set.insert(Box::new(node));
        node
    }

    pub fn new() -> Self {
        StaticIRBuilder {
            nodes: vec![],
            node_set: hash_set![],
            arg_nodes: vec![],
            choice_nodes: vec![],
            rust_nodes: vec![],
            return_node: None,
            vars: hash_set![],
            addrs_to_choice_nodes: hash_map![]
        }
    }

    pub fn build(&mut self) -> StaticIR {
        if self.return_node.is_none() {
            // self.return_node = self.add_constant_node(None);
        }
        StaticIR {
            nodes: self.nodes,
            arg_nodes: self.arg_nodes,
            choice_nodes: self.choice_nodes,
            rust_nodes: self.rust_nodes,
            return_node: self.return_node.unwrap()
        }
    }

    pub fn add_argument_node(&mut self, name: Ident, typ: TypeIR) -> ArgumentNode {

    }

    pub fn add_rust_node(&mut self,
        func: Box<dyn Fn(dyn StaticIRNode) -> dyn StaticIRNode>,
        inputs: Vec<Box<dyn StaticIRNode>>,
        name: Ident,
        typ: TypeIR
    ) {

    }

    pub fn add_constant_node(&mut self, name: Ident, typ: TypeIR) {

    }

    pub fn add_addr_node(&mut self,
        dist: Ident,
        inputs: Vec<Box<dyn StaticIRNode>>,
        addr: Addr,
        name: Ident,  // replace with "Gensym"
        typ: TypeIR
    ) {
    }

    pub fn set_return_node(&mut self, node: dyn StaticIRNode) -> dyn StaticIRNode {

    }
}