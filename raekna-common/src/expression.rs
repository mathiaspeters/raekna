use crate::function_name::FunctionName;

#[derive(Debug, Copy, Clone)]
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

    pub fn as_f64(self) -> f64 {
        match self {
            Self::Integer(i) => i as f64,
            Self::Float(f) => f,
        }
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Float(_), Self::Integer(_)) | (Self::Integer(_), Self::Float(_)) => false,
            (Self::Integer(left), Self::Integer(right)) => *left == *right,
            (Self::Float(left), Self::Float(right)) => (*left - *right).abs() <= f64::EPSILON,
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
        if value > i64::MAX as f64 {
            Self::Float(value)
        } else if value.fract().abs() < f64::EPSILON {
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
