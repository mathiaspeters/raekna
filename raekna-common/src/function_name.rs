use std::{fmt, str::FromStr};

use crate::errors::{CommonError, CommonResult};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FunctionName {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Power,
    Negate,
    // Trigonometry
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
    // Misc math
    SquareRoot,
    CubeRoot,
    Factorial,
    Log,
    Log2,
    Log10,
    Ln,
    Abs,
    // Rounding
    Ceil,
    CeilPrec,
    Floor,
    FloorPrec,
    Round,
    RoundPrec,
    Trunc,
    TruncPrec,
    // Comparisons
    Max,
    Min,
}

impl FunctionName {
    pub fn num_arguments(&self) -> usize {
        match self {
            Self::SquareRoot
            | Self::CubeRoot
            | Self::Log2
            | Self::Log10
            | Self::Ln
            | Self::Abs
            | Self::Ceil
            | Self::Floor
            | Self::Round
            | Self::Trunc
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
            | Self::Power
            | Self::Log
            | Self::CeilPrec
            | Self::FloorPrec
            | Self::RoundPrec
            | Self::TruncPrec => 2,
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
            "cbrt" | "cuberoot" | "cube_root" => Ok(Self::CubeRoot),
            "log" => Ok(Self::Log),
            "log2" => Ok(Self::Log2),
            "log10" => Ok(Self::Log10),
            "ln" => Ok(Self::Ln),
            "abs" => Ok(Self::Abs),
            "ceil" => Ok(Self::Ceil),
            "floor" => Ok(Self::Floor),
            "round" => Ok(Self::Round),
            "trunc" => Ok(Self::Trunc),
            _ => Err(CommonError::UnknownFunctionName(arg.to_owned())),
        }
    }
}

impl fmt::Display for FunctionName {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::SquareRoot => write!(fmt, "sqrt"),
            Self::CubeRoot => write!(fmt, "cbrt"),
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
            Self::Log => write!(fmt, "log"),
            Self::Log2 => write!(fmt, "log2"),
            Self::Log10 => write!(fmt, "log10"),
            Self::Ln => write!(fmt, "ln"),
            Self::Abs => write!(fmt, "abs"),
            Self::Ceil => write!(fmt, "ceil"),
            Self::Floor => write!(fmt, "floor"),
            Self::Round => write!(fmt, "round"),
            Self::CeilPrec => write!(fmt, "ceil with precision"),
            Self::FloorPrec => write!(fmt, "floor with precision"),
            Self::RoundPrec => write!(fmt, "round with precision"),
            Self::Trunc => write!(fmt, "trunc"),
            Self::TruncPrec => write!(fmt, "trunc with precision"),
        }
    }
}
