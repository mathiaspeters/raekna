use raekna_common::{expression::Literal, function_name::FunctionName};

use crate::errors::{ComputeError, ComputeResult};

mod arithmetic;
mod comparisons;

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
        FunctionName::SquareRoot => arithmetic::sqrt(args[0]),
        FunctionName::Factorial => arithmetic::factorial(args[0]),
        FunctionName::Add => Ok(arithmetic::add(args[0], args[1])),
        FunctionName::Subtract => Ok(arithmetic::sub(args[0], args[1])),
        FunctionName::Multiply => Ok(arithmetic::mul(args[0], args[1])),
        FunctionName::Divide => Ok(arithmetic::div(args[0], args[1])),
        FunctionName::Modulus => Ok(arithmetic::mod0(args[0], args[1])),
        FunctionName::Power => arithmetic::pow(args[0], args[1]),

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
