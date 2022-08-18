use raekna_common::expression::Literal;

use super::token_tree::TokenTree;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Literal(Literal),
    Operator(Operator),
    Function(String, Vec<TokenTree>),
    VariableDefinition(String),
    VariableReference(String),
    Nested(TokenTree),
}
