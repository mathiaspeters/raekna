use nom::error::Error;
use raekna_common::expression::Expression;

use crate::lexer::Operator;

pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug)]
pub enum ParserError {
    EmptyExpression,
    InvalidExpression {
        expressions: Vec<Option<Expression>>,
        operators: Vec<Operator>,
    },
    InvalidSign(char),
    UnknownFunctionName(String),
    InvalidVariableDefinition(String),
    NomError(nom::Err<Error<()>>),
}
