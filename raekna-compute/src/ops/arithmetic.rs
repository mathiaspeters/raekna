use raekna_common::expression::Literal;

use crate::{
    errors::{ComputeError, ComputeResult},
    ops::validate_and_wrap,
};

pub fn negate(value: Literal) -> Option<Literal> {
    let res = match value {
        Literal::Integer(i) => Literal::Integer(-i),
        Literal::Float(f) => Literal::Float(-f),
    };
    Some(res)
}

pub fn add(left: Literal, right: Literal) -> Option<Literal> {
    let left = left.as_f64();
    let right = right.as_f64();
    let sum = left + right;
    validate_and_wrap(sum)
}

pub fn sub(left: Literal, right: Literal) -> Option<Literal> {
    let left = left.as_f64();
    let right = right.as_f64();
    let difference = left - right;
    validate_and_wrap(difference)
}

pub fn mul(left: Literal, right: Literal) -> Option<Literal> {
    let left = left.as_f64();
    let right = right.as_f64();
    let product = left * right;
    validate_and_wrap(product)
}

pub fn div(dividend: Literal, divisor: Literal) -> ComputeResult<Option<Literal>> {
    let dividend = dividend.as_f64();
    let divisor = divisor.as_f64();
    if divisor == 0.0 {
        Err(ComputeError::DivisionByZero)
    } else {
        let quotient = dividend / divisor;
        Ok(validate_and_wrap(quotient))
    }
}

pub fn mod0(dividend: Literal, divisor: Literal) -> ComputeResult<Option<Literal>> {
    let dividend = dividend.as_f64();
    let divisor = divisor.as_f64();
    if divisor == 0.0 {
        Err(ComputeError::DivisionByZero)
    } else {
        let remainder = dividend % divisor;
        Ok(validate_and_wrap(remainder))
    }
}

pub fn pow(base: Literal, exponent: Literal) -> Option<Literal> {
    let base = base.as_f64();
    let exponent = exponent.as_f64();
    let power = base.powf(exponent);
    validate_and_wrap(power)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::test_utils::{float, int};

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
            let actual = add(left, right).unwrap();
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
            let actual = sub(left, right).unwrap();
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
            let actual = mul(left, right).unwrap();
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
            let actual = div(left, right).unwrap().unwrap();
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
            let actual = mod0(left, right).unwrap().unwrap();
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
            assert!(pow(left, right).is_none());
        }
    }
}
