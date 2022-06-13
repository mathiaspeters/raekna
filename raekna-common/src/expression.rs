use std::fmt;

use crate::function_name::FunctionName;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
}

impl Literal {
    pub fn maybe_truncate(self) -> Self {
        match self {
            Self::Float(value) if value.fract().abs() < f64::EPSILON => Self::Integer(value as i64),
            _ => self,
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let as_str = match self {
            Literal::Integer(value) => value.to_string(),
            Literal::Float(value) => value.to_string(),
        };
        write!(f, "{as_str}")
    }
}

impl From<i64> for Literal {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<f64> for Literal {
    fn from(value: f64) -> Self {
        if value.fract().abs() < f64::EPSILON {
            Self::Integer(value as i64)
        } else {
            Self::Float(value)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Variable(String, Box<Expression>),
    VariableRef(String),
    Function(FunctionName, Vec<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Literal(literal) => write!(f, "{literal}"),
            Expression::Variable(name, expr) => write!(f, "{name}: {expr}"),
            Expression::VariableRef(name) => write!(f, "{name}"),
            Expression::Function(function_name, arguments) => {
                let args = arguments
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{function_name}({args})",)
            }
        }
    }
}
