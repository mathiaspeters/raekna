use raekna_common::expression::Literal;

use crate::{errors::ComputeResult, ComputeError};

fn process<F>(value: Literal, precision: Option<Literal>, op: F) -> Literal
where
    F: Fn(f64) -> f64,
{
    let value = match value {
        Literal::Integer(i) => i as f64,
        Literal::Float(f) => f,
    };
    let result = match precision {
        Some(Literal::Float(stepping)) => {
            let precision = stepping.to_string().len();
            let precision = 10_u32.pow(precision as u32) as f64;
            let result = value / stepping;
            let result = (result * precision).round() / precision;
            let result = op(result);
            result * stepping
        }
        Some(Literal::Integer(precision)) => {
            let multiplier = 10_i64.pow(precision as u32) as f64;
            let result = value * multiplier;
            let result = op(result);
            result / multiplier
        }
        None => op(value),
    };
    Literal::from(result)
}

pub fn ceil(value: Literal) -> Literal {
    process(value, None, f64::ceil)
}

pub fn ceilprec(value: Literal, precision: Literal) -> Literal {
    process(value, Some(precision), f64::ceil)
}

pub fn floor(value: Literal) -> Literal {
    process(value, None, f64::floor)
}

pub fn floorprec(value: Literal, precision: Literal) -> Literal {
    process(value, Some(precision), f64::floor)
}

pub fn round(value: Literal) -> Literal {
    process(value, None, f64::round)
}

pub fn roundprec(value: Literal, precision: Literal) -> Literal {
    process(value, Some(precision), f64::round)
}

pub fn trunc(value: Literal) -> Literal {
    process(value, None, f64::trunc)
}

pub fn truncprec(value: Literal, precision: Literal) -> ComputeResult<Literal> {
    if let Literal::Float(_) = precision {
        return Err(ComputeError::InvalidTruncatePrecision(precision));
    }
    Ok(process(value, Some(precision), f64::trunc))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ceil() {
        [
            (Literal::Integer(-2), Literal::Integer(-2)),
            (Literal::Integer(0), Literal::Integer(0)),
            (Literal::Integer(3), Literal::Integer(3)),
            (Literal::Float(-2.1), Literal::Integer(-2)),
            (Literal::Float(-2.9), Literal::Integer(-2)),
            (Literal::Float(0.0), Literal::Integer(0)),
            (Literal::Float(7.1), Literal::Integer(8)),
            (Literal::Float(7.9), Literal::Integer(8)),
        ]
        .into_iter()
        .for_each(|(input, expected)| {
            let actual = ceil(input);
            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn test_floor() {
        [
            (Literal::Integer(-2), Literal::Integer(-2)),
            (Literal::Integer(0), Literal::Integer(0)),
            (Literal::Integer(3), Literal::Integer(3)),
            (Literal::Float(-2.1), Literal::Integer(-3)),
            (Literal::Float(-2.9), Literal::Integer(-3)),
            (Literal::Float(0.0), Literal::Integer(0)),
            (Literal::Float(7.1), Literal::Integer(7)),
            (Literal::Float(7.9), Literal::Integer(7)),
        ]
        .into_iter()
        .for_each(|(input, expected)| {
            let actual = floor(input);
            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn test_round() {
        [
            (Literal::Integer(-2), Literal::Integer(-2)),
            (Literal::Integer(0), Literal::Integer(0)),
            (Literal::Integer(3), Literal::Integer(3)),
            (Literal::Float(-2.1), Literal::Integer(-2)),
            (Literal::Float(-2.9), Literal::Integer(-3)),
            (Literal::Float(0.0), Literal::Integer(0)),
            (Literal::Float(7.1), Literal::Integer(7)),
            (Literal::Float(7.9), Literal::Integer(8)),
        ]
        .into_iter()
        .for_each(|(input, expected)| {
            let actual = round(input);
            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn test_trunc() {
        [
            (Literal::Integer(-2), Literal::Integer(-2)),
            (Literal::Integer(0), Literal::Integer(0)),
            (Literal::Integer(3), Literal::Integer(3)),
            (Literal::Float(-5.5), Literal::Integer(-5)),
            (Literal::Float(-5.56789), Literal::Integer(-5)),
            (Literal::Float(0.0), Literal::Integer(0)),
            (Literal::Float(5.5), Literal::Integer(5)),
            (Literal::Float(5.56789), Literal::Integer(5)),
        ]
        .into_iter()
        .for_each(|(input, expected)| {
            let actual = trunc(input);
            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn test_ceil_with_precision() {
        [
            (
                (Literal::Float(-2.159), Literal::Integer(1)),
                Literal::Float(-2.1),
            ),
            (
                (Literal::Float(0.123456789), Literal::Integer(5)),
                Literal::Float(0.12346),
            ),
        ]
        .into_iter()
        .for_each(|(input, expected)| {
            let actual = ceilprec(input.0, input.1);
            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn test_ceil_with_stepping() {
        [
            (
                (Literal::Float(-2.159), Literal::Integer(1)),
                Literal::Float(-2.1),
            ),
            (
                (Literal::Float(0.123456789), Literal::Integer(5)),
                Literal::Float(0.12346),
            ),
        ]
        .into_iter()
        .for_each(|(input, expected)| {
            let actual = ceilprec(input.0, input.1);
            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn test_floor_with_precision() {
        [
            (
                (Literal::Float(-2.159), Literal::Integer(1)),
                Literal::Float(-2.2),
            ),
            (
                (Literal::Float(0.123456789), Literal::Integer(5)),
                Literal::Float(0.12345),
            ),
        ]
        .into_iter()
        .for_each(|(input, expected)| {
            let actual = floorprec(input.0, input.1);
            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn test_floor_with_stepping() {
        [
            (
                (Literal::Float(-2.159), Literal::Integer(1)),
                Literal::Float(-2.2),
            ),
            (
                (Literal::Float(0.123456789), Literal::Integer(5)),
                Literal::Float(0.12345),
            ),
        ]
        .into_iter()
        .for_each(|(input, expected)| {
            let actual = floorprec(input.0, input.1);
            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn test_round_with_precision() {
        [
            (
                (Literal::Float(-2.159), Literal::Integer(1)),
                Literal::Float(-2.2),
            ),
            (
                (Literal::Float(0.123456789), Literal::Integer(5)),
                Literal::Float(0.12346),
            ),
        ]
        .into_iter()
        .for_each(|(input, expected)| {
            let actual = roundprec(input.0, input.1);
            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn test_round_with_stepping() {
        [
            (
                (Literal::Float(-2.159), Literal::Integer(1)),
                Literal::Float(-2.2),
            ),
            (
                (Literal::Float(0.123456789), Literal::Integer(5)),
                Literal::Float(0.12346),
            ),
        ]
        .into_iter()
        .for_each(|(input, expected)| {
            let actual = roundprec(input.0, input.1);
            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn test_trunc_with_precision() {
        [
            (
                (Literal::Float(-2.159), Literal::Integer(1)),
                Literal::Float(-2.1),
            ),
            (
                (Literal::Float(0.123456789), Literal::Integer(5)),
                Literal::Float(0.12345),
            ),
        ]
        .into_iter()
        .for_each(|(input, expected)| {
            let actual = truncprec(input.0, input.1).unwrap();
            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn test_trunc_with_stepping() {
        let float_err = truncprec(Literal::Float(1.2), Literal::Float(0.2)).unwrap_err();
        assert!(matches!(
            float_err,
            ComputeError::InvalidTruncatePrecision(Literal::Float(f)) if f == 0.2
        ));
    }
}
