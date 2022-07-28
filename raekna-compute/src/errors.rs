use raekna_common::{expression::Literal, function_name::FunctionName};

pub type ComputeResult<T> = Result<T, ComputeError>;

#[derive(Debug, PartialEq)]
pub enum ComputeError {
    UnknownVariable(String),
    VariableNameTaken(String),
    FunctionArgumentCount {
        function_name: String,
        expected_argument_count: usize,
        supplied_argument_count: usize,
    },
    ResultTooBig(FunctionName, Vec<Literal>),
    InvalidFactorialArgument(Literal),
    InvalidSquareRoot(Literal),
    DivisionByZero,
    InvalidTruncatePrecision(Literal),
}
