use super::token_tree::TokenTree;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Literal(Literal),
    Operator(Operator),
    Function(String, Vec<TokenTree>),
    VariableDefinition(String),
    VariableReference(String),
    Nested(TokenTree),
}
