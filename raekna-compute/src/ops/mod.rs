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
    let result = match fn_name {
        // Arithmetic
        FunctionName::Negate => arithmetic::negate(args[0]),
        FunctionName::Add => arithmetic::add(args[0], args[1]),
        FunctionName::Subtract => arithmetic::sub(args[0], args[1]),
        FunctionName::Multiply => arithmetic::mul(args[0], args[1]),
        FunctionName::Divide => arithmetic::div(args[0], args[1])?,
        FunctionName::Modulus => arithmetic::mod0(args[0], args[1])?,
        FunctionName::Power => arithmetic::pow(args[0], args[1]),

        // Trigonometry
        FunctionName::Sin => trigonometry::sin(args[0]),
        FunctionName::Cos => trigonometry::cos(args[0]),
        FunctionName::Tan => trigonometry::tan(args[0]),
        FunctionName::SinH => trigonometry::sinh(args[0]),
        FunctionName::CosH => trigonometry::cosh(args[0]),
        FunctionName::TanH => trigonometry::tanh(args[0]),
        FunctionName::ArcSin => trigonometry::asin(args[0]),
        FunctionName::ArcCos => trigonometry::acos(args[0]),
        FunctionName::ArcTan => trigonometry::atan(args[0]),
        FunctionName::ArcSinH => trigonometry::asinh(args[0]),
        FunctionName::ArcCosH => trigonometry::acosh(args[0]),
        FunctionName::ArcTanH => trigonometry::atanh(args[0]),

        // Misc math
        FunctionName::SquareRoot => misc_math::sqrt(args[0])?,
        FunctionName::CubeRoot => misc_math::cbrt(args[0]),
        FunctionName::Factorial => misc_math::factorial(args[0])?,
        FunctionName::Log => misc_math::log(args[0], args[1]),
        FunctionName::Log2 => misc_math::log2(args[0]),
        FunctionName::Log10 => misc_math::log10(args[0]),
        FunctionName::Ln => misc_math::ln(args[0]),
        FunctionName::Abs => misc_math::abs(args[0]),

        // Rounding
        FunctionName::Ceil => rounding::ceil(args[0]),
        FunctionName::CeilPrec => rounding::ceilprec(args[0], args[1]),
        FunctionName::Floor => rounding::floor(args[0]),
        FunctionName::FloorPrec => rounding::floorprec(args[0], args[1]),
        FunctionName::Round => rounding::round(args[0]),
        FunctionName::RoundPrec => rounding::roundprec(args[0], args[1]),
        FunctionName::Trunc => rounding::trunc(args[0]),
        FunctionName::TruncPrec => rounding::truncprec(args[0], args[1])?,

        // Comparisons
        FunctionName::Max => comparisons::max(args[0], args[1]),
        FunctionName::Min => comparisons::min(args[0], args[1]),
    };
    match result {
        Some(result) => Ok(result),
        None => Err(ComputeError::ResultTooBig(fn_name, args)),
    }
}

fn validate_and_wrap(value: f64) -> Option<Literal> {
    if value.is_normal() || value == 0.0 {
        Some(Literal::from(value))
    } else {
        None
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
