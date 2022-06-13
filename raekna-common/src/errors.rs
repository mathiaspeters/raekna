pub type CommonResult<T> = Result<T, CommonError>;

#[derive(Debug)]
pub enum CommonError {
    UnknownFunctionName(String),
    OutOfBounds(usize),
}
