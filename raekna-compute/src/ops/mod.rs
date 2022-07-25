use raekna_common::{expression::Literal, function_name::FunctionName};

use crate::errors::{ComputeError, ComputeResult};

mod arithmetic;
mod comparisons;
mod misc_math;
mod rounding;
mod trigonometry;

pub fn evaluate_fn(fn_name: FunctionName, args: Vec<Literal>) -> ComputeResult<Literal> {
    if args.len() != fn_name.num_arguments() {
        return Err(ComputeError::FunctionArgumentCount {
            function_name: fn_name.to_string(),
            expected_argument_count: fn_name.num_arguments(),
            supplied_argument_count: args.len(),
        });
    }
    match fn_name {
        // Arithmetic
        FunctionName::Negate => arithmetic::negate(args[0]),
        FunctionName::Add => Ok(arithmetic::add(args[0], args[1])),
        FunctionName::Subtract => Ok(arithmetic::sub(args[0], args[1])),
        FunctionName::Multiply => Ok(arithmetic::mul(args[0], args[1])),
        FunctionName::Divide => Ok(arithmetic::div(args[0], args[1])),
        FunctionName::Modulus => Ok(arithmetic::mod0(args[0], args[1])),
        FunctionName::Power => arithmetic::pow(args[0], args[1]),

        // Trigonometry
        FunctionName::Sin => Ok(trigonometry::sin(args[0])),
        FunctionName::Cos => Ok(trigonometry::cos(args[0])),
        FunctionName::Tan => Ok(trigonometry::tan(args[0])),
        FunctionName::SinH => Ok(trigonometry::sinh(args[0])),
        FunctionName::CosH => Ok(trigonometry::cosh(args[0])),
        FunctionName::TanH => Ok(trigonometry::tanh(args[0])),
        FunctionName::ArcSin => Ok(trigonometry::asin(args[0])),
        FunctionName::ArcCos => Ok(trigonometry::acos(args[0])),
        FunctionName::ArcTan => Ok(trigonometry::atan(args[0])),
        FunctionName::ArcSinH => Ok(trigonometry::asinh(args[0])),
        FunctionName::ArcCosH => Ok(trigonometry::acosh(args[0])),
        FunctionName::ArcTanH => Ok(trigonometry::atanh(args[0])),

        // Misc math
        FunctionName::SquareRoot => misc_math::sqrt(args[0]),
        FunctionName::CubeRoot => misc_math::cbrt(args[0]),
        FunctionName::Factorial => misc_math::factorial(args[0]),
        FunctionName::Log => misc_math::log(args[0], args[1]),
        FunctionName::Log2 => misc_math::log2(args[0]),
        FunctionName::Log10 => misc_math::log10(args[0]),
        FunctionName::Ln => misc_math::ln(args[0]),
        FunctionName::Abs => Ok(misc_math::abs(args[0])),

        // Rounding
        FunctionName::Ceil => Ok(rounding::ceil(args[0])),
        FunctionName::CeilPrec => Ok(rounding::ceilprec(args[0], args[1])),
        FunctionName::Floor => Ok(rounding::floor(args[0])),
        FunctionName::FloorPrec => Ok(rounding::floorprec(args[0], args[1])),
        FunctionName::Round => Ok(rounding::round(args[0])),
        FunctionName::RoundPrec => Ok(rounding::roundprec(args[0], args[1])),
        FunctionName::Trunc => Ok(rounding::trunc(args[0])),
        FunctionName::TruncPrec => rounding::truncprec(args[0], args[1]),

        // Comparisons
        FunctionName::Max => Ok(comparisons::max(args[0], args[1])),
        FunctionName::Min => Ok(comparisons::min(args[0], args[1])),
    }
}

#[cfg(test)]
mod test_utils {
    use super::*;

    pub fn int(value: i64) -> Literal {
        Literal::Integer(value)
    }

    pub fn float(value: f64) -> Literal {
        Literal::Float(value)
    }
}
