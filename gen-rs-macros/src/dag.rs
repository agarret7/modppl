use std::rc::{Rc,Weak};
use std::collections::{HashMap,HashSet};
use std::cell::RefCell;
use common_macros::{hash_set,hash_map};

use gen_rs::{modeling::dists::Distribution,Addr};
use syn::token::Static;
use syn::{Expr,ExprBlock,ExprClosure, parse_quote, Type};
use proc_macro2::{Ident,Span};
use quote::quote;


thread_local!(static GENSYM_COUNTER: RefCell<usize> = RefCell::new(0));
fn gensym() -> Ident {
    GENSYM_COUNTER.with(|count| {
        let sym = format!("tmp{}", count.borrow());
        *count.borrow_mut() += 1;
        Ident::new(&sym, Span::call_site())
    })
}

#[derive(PartialEq,Eq,Clone)]
pub enum StaticIRNode {
    Arg(ArgumentNode),
    Rust(RustNode),
    RandomChoice(RandomChoiceNode)
}

impl StaticIRNode {
    pub fn id(&self) -> Ident {
        match self {
            StaticIRNode::Arg(node) => node.name.clone(),
            StaticIRNode::Rust(node) => node.name.clone(),
            StaticIRNode::RandomChoice(node) => node.name.clone()
        }
    }

    pub fn ty(&self) -> Type {
        match self {
            StaticIRNode::Arg(node) => node.ty.clone(),
            StaticIRNode::Rust(node) => node.ty.clone(),
            StaticIRNode::RandomChoice(node) => node.ty.clone()
        }
    }
}

#[derive(PartialEq,Eq,Clone)]
pub struct ArgumentNode {
    pub name: Ident,
    pub ty: Type
}

// basically a wrapper for `syn::ExprClosure`
// which contains parents as inputs
#[derive(PartialEq,Eq,Clone)]
pub struct RustNode {
    pub func: ExprClosure,
    pub inputs: Vec<Ident>,
    pub name: Ident,
    pub ty: Type
}

#[derive(PartialEq,Eq,Clone)]
pub struct RandomChoiceNode {
    pub dist: Ident,
    pub inputs: Vec<Ident>,
    pub addr: Addr,
    pub name: Ident,
    pub ty: Type
}


pub struct StaticIR {
    pub nodes: Vec<StaticIRNode>,
    pub return_id: Ident
}

impl StaticIR {
    pub fn arg_nodes(&self) -> Vec<StaticIRNode> {
        let mut args = vec![];
        for node in self.nodes.iter() {
            match node {
                StaticIRNode::Arg(_) => args.push(node.clone()),
                _ => { }
            }
        }
        args
    }
}

pub struct StaticIRBuilder {
    nodes: Vec<Rc<StaticIRNode>>,
    return_node: Option<Weak<StaticIRNode>>,
    vars: HashSet<Ident>,
    addrs_to_choice_nodes: HashMap<Addr, Rc<RandomChoiceNode>>
}

impl StaticIRBuilder {

    fn check_unique_id(&self, id: &Ident) {
        if self.vars.contains(id) {
            panic!("Variable id `{}` is not unique", id);
        }
    }

    fn check_unique_addr(&self, addr: Addr) {
        if self.addrs_to_choice_nodes.contains_key(addr) {
            panic!("Address \"{}\" was not unique", addr);
        }
    }

    fn upgrade_inputs(&self, input_nodes: Vec<Weak<StaticIRNode>>) -> Vec<Rc<StaticIRNode>> {
        input_nodes.into_iter().map(|input_node| {
            match input_node.upgrade() {
                None => { panic!("Weak node reference was dropped! Did you reference another builder's variable?"); }
                Some(input_node_up) => {
                    if self.nodes.contains(&input_node_up) {
                        input_node_up
                    } else {
                        panic!("Node {} was not previously added to the IR", input_node_up.id());
                    }
                }
            }
        }).collect::<Vec<Rc<StaticIRNode>>>()
    }

    fn add_node(&mut self, node: StaticIRNode) -> Weak<StaticIRNode> {
        let node_ref = Rc::new(node);
        self.nodes.push(node_ref.clone());
        Rc::downgrade(&node_ref)
    }

    pub fn new() -> Self {
        StaticIRBuilder {
            nodes: vec![],
            return_node: None,
            vars: hash_set![],
            addrs_to_choice_nodes: hash_map![]
        }
    }

    pub fn build(self) -> StaticIR {
        let return_id = match self.return_node {
            None => { panic!("Return node not set") },
            Some(ret) => { ret.upgrade().unwrap().id() }
        };
        StaticIR {
            nodes: self.nodes.into_iter()
                .map(|node| Rc::into_inner(node).unwrap())
                .collect::<Vec<StaticIRNode>>(),
            return_id
        }
    }

    pub fn add_argument_node(&mut self, name: Ident, ty: Type) -> Weak<StaticIRNode> {
        self.check_unique_id(&name);
        let node = ArgumentNode { name, ty };
        self.add_node(StaticIRNode::Arg(node))
    }

    pub fn add_rust_node(&mut self,
        func: ExprClosure,
        input_nodes: Vec<Weak<StaticIRNode>>,
        name: Option<Ident>,
        ty: Option<Type>
    ) -> Weak<StaticIRNode> {
        let name = name.unwrap_or_else(|| gensym());
        self.check_unique_id(&name);
        let ty = ty.unwrap_or(parse_quote!(_));
        let input_ids = self.upgrade_inputs(input_nodes)
            .iter()
            .map(|inp| inp.id())
            .collect::<Vec<Ident>>();
        self.vars.insert(name.clone());
        let node = RustNode { func, inputs: input_ids, name, ty };
        self.add_node(StaticIRNode::Rust(node))
    }

    pub fn add_constant_node(&mut self, val: syn::Lit, name: Option<Ident>, ty: Option<Type>) -> Weak<StaticIRNode> {
        let name = name.unwrap_or_else(|| gensym());
        self.check_unique_id(&name);
        let ty = ty.unwrap_or(parse_quote!(_));
        self.vars.insert(name.clone());
        let node = RustNode { func: parse_quote!(|| { #val }), inputs: vec![], name, ty };
        self.add_node(StaticIRNode::Rust(node))
    }

    pub fn add_addr_node(&mut self,
        dist: Ident,
        input_nodes: Vec<Weak<StaticIRNode>>,
        addr: Addr,
        name: Option<Ident>,
        ty: Option<Type>
    ) -> Weak<StaticIRNode> {
        let name = name.unwrap_or_else(|| gensym());
        self.check_unique_id(&name);
        self.check_unique_addr(addr);
        let ty = ty.unwrap_or(parse_quote!(_));
        let input_ids = self.upgrade_inputs(input_nodes)
            .iter()
            .map(|inp| inp.id())
            .collect::<Vec<Ident>>();
        self.vars.insert(name.clone());
        let node = RandomChoiceNode { dist, inputs: input_ids, addr, name, ty };
        self.add_node(StaticIRNode::RandomChoice(node))
    }

    pub fn set_return_node(&mut self, node: Weak<StaticIRNode>) {
        if self.return_node.is_none() {
            match node.upgrade() {
                None => { panic!("Weak node reference was dropped! Did you reference another builder's variable?") }
                Some(_) => {
                    self.return_node = Some(node);
                }
            }
        } else {
            panic!("Return node already set")
        }
    }

    pub fn set_return_empty(&mut self) {
        let ret = self.add_node(
            StaticIRNode::Rust(RustNode {
                func: parse_quote!(|_| {()}),
                inputs: vec![],
                name: gensym(),
                ty: parse_quote!(())
            })
        );
        self.set_return_node(ret)
    }
}