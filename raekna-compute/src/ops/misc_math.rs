use raekna_common::expression::Literal;

use crate::errors::{ComputeError, ComputeResult};

pub fn sqrt(value: Literal) -> ComputeResult<Literal> {
    use Literal::*;
    let root = match value {
        Integer(value) => (value as f64).sqrt(),
        Float(value) => value.sqrt(),
    };
    if root.is_normal() || root == 0.0 {
        Ok(Literal::from(root))
    } else {
        Err(ComputeError::InvalidSquareRoot(value))
    }
}

pub fn cbrt(value: Literal) -> ComputeResult<Literal> {
    use Literal::*;
    let root = match value {
        Integer(value) => (value as f64).cbrt(),
        Float(value) => value.cbrt(),
    };
    if root.is_normal() || root == 0.0 {
        Ok(Literal::from(root))
    } else {
        Err(ComputeError::InvalidCubeRoot(value))
    }
}

pub fn factorial(value: Literal) -> ComputeResult<Literal> {
    use Literal::*;
    match value {
        Integer(val) if val < 0 => {
            // TODO: Support factorial with negative numbers at some point
            Err(ComputeError::InvalidFactorialArgument(value))
        }
        Integer(value) if value <= 20 => {
            let mut result = 1;
            for i in 1..=value {
                result *= i;
            }
            Ok(Integer(result))
        }
        _ => Err(ComputeError::InvalidFactorialArgument(value)),
    }
}

pub fn log(value: Literal, base: Literal) -> Literal {
    let value = match value {
        Literal::Integer(i) => i as f64,
        Literal::Float(f) => f,
    };
    let base = match base {
        Literal::Integer(i) => i as f64,
        Literal::Float(f) => f,
    };
    Literal::from(value.log(base))
}

pub fn log2(value: Literal) -> Literal {
    let value = match value {
        Literal::Integer(i) => i as f64,
        Literal::Float(f) => f,
    };
    Literal::from(value.log2())
}

pub fn log10(value: Literal) -> Literal {
    let value = match value {
        Literal::Integer(i) => i as f64,
        Literal::Float(f) => f,
    };
    Literal::from(value.log10())
}

pub fn ln(value: Literal) -> Literal {
    let value = match value {
        Literal::Integer(i) => i as f64,
        Literal::Float(f) => f,
    };
    Literal::from(value.ln())
}

pub fn abs(value: Literal) -> Literal {
    match value {
        Literal::Integer(i) => Literal::Integer(i.abs()),
        Literal::Float(f) => Literal::Float(f.abs()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::test_utils::{float, int};

    #[test]
    fn square_root() {
        let test_cases = [
            // Integers
            (int(0), int(0)),
            (int(2), float(1.4142135623730951)),
            (int(25), int(5)),
            // Floats
            (float(3.5), float(1.8708286933869707)),
        ];
        for (value, expected) in test_cases.into_iter() {
            let actual = sqrt(value).unwrap();
            assert_eq!(actual, expected);
        }

        let test_cases = [
            int(-2),
            float(f64::NAN),
            float(f64::INFINITY),
            float(f64::NEG_INFINITY),
        ];
        for value in test_cases.into_iter() {
            sqrt(value).unwrap_err();
        }
    }

    #[test]
    fn test_factorial() {
        let test_cases = [
            (int(0), int(1)),
            (int(2), int(2)),
            (int(6), int(720)),
            (int(11), int(39_916_800)),
        ];
        for (value, expected) in test_cases.into_iter() {
            let actual = factorial(value).unwrap();
            assert_eq!(actual, expected);
        }

        let test_cases = [int(-2), int(21), float(1.1)];
        for value in test_cases.into_iter() {
            factorial(value).unwrap_err();
        }
    }
}
