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

pub fn add(left: Literal, right: Literal) -> Literal {
    use Literal::*;
    let sum = match (left, right) {
        (Integer(left), Integer(right)) => match left.checked_add(right) {
            Some(res) => return Integer(res),
            None => (left as f64) + (right as f64),
        },
        (Integer(i), Float(f)) | (Float(f), Integer(i)) => f + (i as f64),
        (Float(left), Float(right)) => left + right,
    };
    Literal::from(sum)
}

pub fn sub(left: Literal, right: Literal) -> Literal {
    use Literal::*;
    let difference = match (left, right) {
        (Integer(left), Integer(right)) => match left.checked_sub(right) {
            Some(res) => return Integer(res),
            None => (left as f64) - (right as f64),
        },
        (Integer(i), Float(f)) => (i as f64) - f,
        (Float(f), Integer(i)) => f - (i as f64),
        (Float(left), Float(right)) => left - right,
    };
    Literal::from(difference)
}

pub fn mul(left: Literal, right: Literal) -> Literal {
    use Literal::*;
    let product = match (left, right) {
        (Integer(left), Integer(right)) => match left.checked_mul(right) {
            Some(res) => return Integer(res),
            None => (left as f64) * (right as f64),
        },
        (Integer(i), Float(f)) | (Float(f), Integer(i)) => f * (i as f64),
        (Float(left), Float(right)) => left * right,
    };
    Literal::from(product)
}

pub fn div(left: Literal, right: Literal) -> Literal {
    use Literal::*;
    let quotient = match (left, right) {
        (Integer(left), Integer(right)) => {
            let res = (left as f64) / (right as f64);
            if res.fract().abs() < f64::EPSILON {
                return Integer(res as i64);
            } else {
                res
            }
        }
        (Integer(i), Float(f)) => (i as f64) / f,
        (Float(f), Integer(i)) => f / (i as f64),
        (Float(left), Float(right)) => left / right,
    };
    Literal::from(quotient)
}

pub fn mod0(left: Literal, right: Literal) -> Literal {
    use Literal::*;
    let remainder = match (left, right) {
        (Integer(left), Integer(right)) => match left.checked_rem(right) {
            Some(res) => return Integer(res),
            None => (left as f64) % (right as f64),
        },
        (Integer(i), Float(f)) => (i as f64) % f,
        (Float(f), Integer(i)) => f % (i as f64),
        (Float(left), Float(right)) => left % right,
    };
    Literal::from(remainder)
}

pub fn pow(left: Literal, right: Literal) -> ComputeResult<Literal> {
    use Literal::*;
    let power = match (left, right) {
        (Integer(left), Integer(right)) => (left as f64).powf(right as f64),
        (Integer(i), Float(f)) => (i as f64).powf(f),
        (Float(f), Integer(i)) => f.powf(i as f64),
        (Float(left), Float(right)) => left.powf(right),
    };
    if power.is_normal() || power == 0.0 {
        Ok(Literal::from(power))
    } else {
        Err(ComputeError::InvalidPower {
            factor: left,
            exponent: right,
        })
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

    #[test]
    fn addition() {
        let test_cases = [
            // Integers on both sides
            (int(5), int(8), int(13)),
            (int(5), int(-8), int(-3)),
            (int(-5), int(8), int(3)),
            (int(-5), int(-8), int(-13)),
            // Mixed integers and floats
            (int(5), float(8.0), int(13)),
            (int(5), float(-8.0), int(-3)),
            (int(-5), float(8.0), int(3)),
            (int(-5), float(-8.3), float(-13.3)),
            (float(5.0), int(8), int(13)),
            (float(5.0), int(-8), int(-3)),
            (float(-5.0), int(8), int(3)),
            (float(-5.2), int(-8), float(-13.2)),
            // Floats on both sides
            (float(5.2), float(8.3), float(13.5)),
            (float(5.2), float(-8.4), float(-3.2)),
            (float(-5.3), float(8.5), float(3.2)),
            (float(-5.4), float(-8.5), float(-13.9)),
        ];
        for (left, right, expected) in test_cases.into_iter() {
            let actual = add(left, right);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn subtraction() {
        let test_cases = [
            // Integers on both sides
            (int(5), int(8), int(-3)),
            (int(5), int(-8), int(13)),
            (int(-5), int(8), int(-13)),
            (int(-5), int(-8), int(3)),
            // Mixed integers and floats
            (int(5), float(8.5), float(-3.5)),
            (int(5), float(-8.0), int(13)),
            (int(-5), float(8.0), int(-13)),
            (int(-5), float(-8.0), int(3)),
            (float(5.2), int(8), float(-2.8)),
            (float(5.0), int(-8), int(13)),
            (float(-5.0), int(8), int(-13)),
            (float(-5.0), int(-8), int(3)),
            // Floats on both sides
            (float(5.0), float(8.0), int(-3)),
            (float(5.0), float(-8.0), int(13)),
            (float(-5.0), float(8.0), int(-13)),
            (float(-5.2), float(-8.4), float(3.2)),
        ];
        for (left, right, expected) in test_cases.into_iter() {
            let actual = sub(left, right);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn multiplication() {
        let test_cases = [
            // Integers on both sides
            (int(5), int(8), int(40)),
            (int(5), int(-8), int(-40)),
            (int(-5), int(8), int(-40)),
            (int(-5), int(-8), int(40)),
            // Mixed integers and floats
            (int(5), float(8.5), float(42.5)),
            (int(5), float(-8.0), int(-40)),
            (int(-5), float(8.0), int(-40)),
            (int(-5), float(-8.0), int(40)),
            (float(5.6), int(8), float(44.8)),
            (float(5.0), int(-8), int(-40)),
            (float(-5.0), int(8), int(-40)),
            (float(-5.0), int(-8), int(40)),
            // Floats on both sides
            (float(5.6), float(8.0), float(44.8)),
            (float(5.0), float(-8.0), int(-40)),
            (float(-5.0), float(8.0), int(-40)),
            (float(-5.0), float(-8.0), int(40)),
        ];
        for (left, right, expected) in test_cases.into_iter() {
            let actual = mul(left, right);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn division() {
        let test_cases = [
            // Integers on both sides
            (int(20), int(5), int(4)),
            (int(20), int(-5), int(-4)),
            (int(-20), int(5), int(-4)),
            (int(-20), int(-5), int(4)),
            (int(8), int(5), float(1.6)),
            // Mixed integers and floats
            (int(20), float(5.0), int(4)),
            (int(20), float(-5.0), int(-4)),
            (int(-20), float(5.0), int(-4)),
            (int(-20), float(-5.0), int(4)),
            (float(22.0), int(5), float(4.4)),
            (float(20.0), int(-5), int(-4)),
            (float(-20.0), int(5), int(-4)),
            (float(-20.0), int(-5), int(4)),
            // Floats on both sides
            (float(22.0), float(5.0), float(4.4)),
            (float(20.0), float(-5.0), int(-4)),
            (float(-20.0), float(5.0), int(-4)),
            (float(-20.0), float(-5.0), int(4)),
        ];
        for (left, right, expected) in test_cases.into_iter() {
            let actual = div(left, right);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn modulo() {
        let test_cases = [
            // Integers on both sides
            (int(8), int(5), int(3)),
            (int(8), int(-5), int(3)),
            (int(-8), int(5), int(-3)),
            (int(-8), int(-5), int(-3)),
            // Mixed integers and floats
            (int(8), float(5.5), float(2.5)),
            (int(8), float(-5.0), int(3)),
            (int(-8), float(5.0), int(-3)),
            (int(-8), float(-5.0), int(-3)),
            (float(8.5), int(5), float(3.5)),
            (float(8.0), int(-5), int(3)),
            (float(-8.0), int(5), int(-3)),
            (float(-8.0), int(-5), int(-3)),
            // Floats on both sides
            (float(8.5), float(5.0), float(3.5)),
            (float(8.0), float(-5.0), int(3)),
            (float(-8.0), float(5.0), int(-3)),
            (float(-8.0), float(-5.0), int(-3)),
        ];
        for (left, right, expected) in test_cases.into_iter() {
            let actual = mod0(left, right);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn power() {
        let test_cases = [
            // Integers on both sides
            (int(4), int(2), int(16)),
            (int(4), int(-2), float(0.0625)),
            (int(-4), int(3), int(-64)),
            (int(-4), int(-3), float(-0.015625)),
            // Mixed integers and floats
            (int(4), float(2.1), float(18.37917367995256)),
            (int(4), float(-2.0), float(0.0625)),
            (int(-4), float(3.0), int(-64)),
            (int(-4), float(-1.0), float(-0.25)),
            (float(4.5), int(2), float(20.25)),
            (float(4.0), int(-2), float(0.0625)),
            //(float(-4.4), int(2), float(19.36)), rounding error
            (float(-4.0), int(-2), float(0.0625)),
            // Floats on both sides
            (float(4.5), float(2.5), float(42.95673695708276)),
            (float(4.0), float(-3.0), float(0.015625)),
            (float(-4.0), float(2.0), int(16)),
            (float(-4.0), float(-2.0), float(0.0625)),
        ];
        for (left, right, expected) in test_cases.into_iter() {
            let actual = pow(left, right).unwrap();
            assert_eq!(actual, expected);
        }

        let test_cases = [(int(-1), float(-0.1))];
        for (left, right) in test_cases.into_iter() {
            pow(left, right).unwrap_err();
        }
    }
}
