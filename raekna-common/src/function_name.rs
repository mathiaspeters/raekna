use std::{fmt, str::FromStr};

use crate::errors::{CommonError, CommonResult};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FunctionName {
    SquareRoot,
    Factorial,
    Max,
    Min,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Power,
}

impl FunctionName {
    pub fn num_arguments(&self) -> usize {
        match self {
            FunctionName::SquareRoot | FunctionName::Factorial => 1,
            FunctionName::Max
            | FunctionName::Min
            | FunctionName::Add
            | FunctionName::Subtract
            | FunctionName::Multiply
            | FunctionName::Divide
            | FunctionName::Modulus
            | FunctionName::Power => 2,
        }
    }
}

impl FromStr for FunctionName {
    type Err = CommonError;

    fn from_str(arg: &str) -> CommonResult<Self> {
        match arg.to_lowercase().as_str() {
            "sqrt" | "squareroot" | "square_root" => Ok(Self::SquareRoot),
            "fact" | "factorial" => Ok(Self::Factorial),
            "min" | "minimum" => Ok(Self::Min),
            "max" | "maximum" => Ok(Self::Max),
            "add" => Ok(Self::Add),
            "sub" | "subtract" => Ok(Self::Subtract),
            "mul" | "multiply" => Ok(Self::Multiply),
            "div" | "divide" => Ok(Self::Divide),
            "mod" | "modulus" => Ok(Self::Modulus),
            "pow" | "power" => Ok(Self::Power),
            _ => Err(CommonError::UnknownFunctionName(arg.to_owned())),
        }
    }
}

impl fmt::Display for FunctionName {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            FunctionName::SquareRoot => write!(fmt, "sqrt"),
            FunctionName::Factorial => write!(fmt, "factorial"),
            FunctionName::Max => write!(fmt, "max"),
            FunctionName::Min => write!(fmt, "min"),
            FunctionName::Add => write!(fmt, "add"),
            FunctionName::Subtract => write!(fmt, "sub"),
            FunctionName::Multiply => write!(fmt, "mul"),
            FunctionName::Divide => write!(fmt, "div"),
            FunctionName::Modulus => write!(fmt, "mod"),
            FunctionName::Power => write!(fmt, "pow"),
        }
    }
}
