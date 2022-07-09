use raekna_common::expression::Literal;

pub type ComputeResult<T> = Result<T, ComputeError>;

#[derive(Debug)]
pub enum ComputeError {
    UnknownVariable(String),
    FunctionArgumentCount {
        function_name: String,
        expected_argument_count: usize,
        supplied_argument_count: usize,
    },
    InvalidFactorialArgument(Literal),
    InvalidSquareRoot(Literal),
    InvalidCubeRoot(Literal),
    InvalidPower {
        factor: Literal,
        exponent: Literal,
    },
}
