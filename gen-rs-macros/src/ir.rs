use syn::{Error, Expr, Ident};
use crate::{
    ChoiceHashMap
};

pub struct VariableIR {
    pub var_ident: Ident,
    pub type_ident: Ident,
    pub dependency: Expr,
    pub is_observed: bool
}

pub struct ModelIR<U> {
    pub model_ident: Ident,
    pub args: U,
    pub choices: ChoiceHashMap
}