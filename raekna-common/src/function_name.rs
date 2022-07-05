use std::{fmt, str::FromStr};

use crate::errors::{CommonError, CommonResult};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FunctionName {
    // Unary
    SquareRoot,
    Factorial,
    Negate,
    // Binary
    Max,
    Min,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Power,
    // Trigonometric
    Sin,
    Cos,
    Tan,
    SinH,
    CosH,
    TanH,
    ArcSin,
    ArcCos,
    ArcTan,
    ArcSinH,
    ArcCosH,
    ArcTanH,
}

impl FunctionName {
    pub fn num_arguments(&self) -> usize {
        match self {
            Self::SquareRoot
            | Self::Factorial
            | Self::Negate
            | Self::Sin
            | Self::Cos
            | Self::Tan
            | Self::SinH
            | Self::CosH
            | Self::TanH
            | Self::ArcSin
            | Self::ArcCos
            | Self::ArcTan
            | Self::ArcSinH
            | Self::ArcCosH
            | Self::ArcTanH => 1,
            Self::Max
            | Self::Min
            | Self::Add
            | Self::Subtract
            | Self::Multiply
            | Self::Divide
            | Self::Modulus
            | Self::Power => 2,
        }
    }
}

impl FromStr for FunctionName {
    type Err = CommonError;

    fn from_str(arg: &str) -> CommonResult<Self> {
        match arg.to_lowercase().as_str() {
            "sqrt" | "squareroot" | "square_root" => Ok(Self::SquareRoot),
            "fact" | "factorial" => Ok(Self::Factorial),
            "neg" | "negate" => Ok(Self::Negate),
            "min" | "minimum" => Ok(Self::Min),
            "max" | "maximum" => Ok(Self::Max),
            "add" => Ok(Self::Add),
            "sub" | "subtract" => Ok(Self::Subtract),
            "mul" | "multiply" => Ok(Self::Multiply),
            "div" | "divide" => Ok(Self::Divide),
            "mod" | "modulus" => Ok(Self::Modulus),
            "pow" | "power" => Ok(Self::Power),
            "sin" => Ok(Self::Sin),
            "cos" => Ok(Self::Cos),
            "tan" => Ok(Self::Tan),
            "sinh" => Ok(Self::SinH),
            "cosh" => Ok(Self::CosH),
            "tanh" => Ok(Self::TanH),
            "asin" | "arcsin" => Ok(Self::ArcSin),
            "acos" | "arccos" => Ok(Self::ArcCos),
            "atan" | "arctan" => Ok(Self::ArcTan),
            "asinh" | "arcsinh" => Ok(Self::ArcSinH),
            "acosh" | "arccosh" => Ok(Self::ArcCosH),
            "atanh" | "arctanh" => Ok(Self::ArcTanH),
            _ => Err(CommonError::UnknownFunctionName(arg.to_owned())),
        }
    }
}

impl fmt::Display for FunctionName {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::SquareRoot => write!(fmt, "sqrt"),
            Self::Factorial => write!(fmt, "factorial"),
            Self::Negate => write!(fmt, "negate"),
            Self::Max => write!(fmt, "max"),
            Self::Min => write!(fmt, "min"),
            Self::Add => write!(fmt, "add"),
            Self::Subtract => write!(fmt, "sub"),
            Self::Multiply => write!(fmt, "mul"),
            Self::Divide => write!(fmt, "div"),
            Self::Modulus => write!(fmt, "mod"),
            Self::Power => write!(fmt, "pow"),
            Self::Sin => write!(fmt, "sin"),
            Self::Cos => write!(fmt, "cos"),
            Self::Tan => write!(fmt, "tan"),
            Self::SinH => write!(fmt, "sinh"),
            Self::CosH => write!(fmt, "cosh"),
            Self::TanH => write!(fmt, "tanh"),
            Self::ArcSin => write!(fmt, "asin"),
            Self::ArcCos => write!(fmt, "acos"),
            Self::ArcTan => write!(fmt, "atan"),
            Self::ArcSinH => write!(fmt, "asinh"),
            Self::ArcCosH => write!(fmt, "acosh"),
            Self::ArcTanH => write!(fmt, "atanh"),
        }
    }
}
