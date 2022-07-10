use raekna_common::expression::Literal;

fn process<F>(
    value: Literal,
    precision: Option<Literal>,
    _stepping: Option<Literal>,
    op: F,
) -> Literal
where
    F: Fn(f64) -> f64,
{
    match value {
        Literal::Integer(_) => value,
        Literal::Float(f) => {
            let result = match precision {
                Some(Literal::Float(stepping)) => {
                    let precision = stepping.to_string().len();
                    let precision = 10_u32.pow(precision as u32) as f64;
                    let result = f / stepping;
                    let result = (result * precision).round() / precision;
                    let result = op(result);
                    result * stepping
                }
                Some(Literal::Integer(precision)) => {
                    let multiplier = 10_i64.pow(precision as u32) as f64;
                    let result = f * multiplier;
                    let result = op(result);
                    result / multiplier
                }
                None => op(f),
            };
            Literal::from(result)
        }
    }
}

pub fn ceil(value: Literal) -> Literal {
    process(value, None, None, f64::ceil)
}

pub fn ceilprec(value: Literal, precision: Literal) -> Literal {
    process(value, Some(precision), None, f64::ceil)
}

pub fn floor(value: Literal) -> Literal {
    process(value, None, None, f64::floor)
}

pub fn floorprec(value: Literal, precision: Literal) -> Literal {
    process(value, Some(precision), None, f64::floor)
}

pub fn round(value: Literal) -> Literal {
    process(value, None, None, f64::round)
}

pub fn roundprec(value: Literal, precision: Literal) -> Literal {
    process(value, Some(precision), None, f64::round)
}

pub fn trunc(value: Literal) -> Literal {
    process(value, None, None, f64::trunc)
}

pub fn truncprec(value: Literal, precision: Literal) -> Literal {
    process(value, Some(precision), None, f64::trunc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ceil() {
        [
            (Literal::Integer(-2), Literal::Integer(-2)),
            (Literal::Float(-5.5), Literal::Integer(-5)),
            (Literal::Integer(0), Literal::Integer(0)),
            (Literal::Float(0.0), Literal::Integer(0)),
            (Literal::Integer(3), Literal::Integer(3)),
            (Literal::Float(5.5), Literal::Integer(6)),
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
            (Literal::Float(-5.5), Literal::Integer(-6)),
            (Literal::Integer(0), Literal::Integer(0)),
            (Literal::Float(0.0), Literal::Integer(0)),
            (Literal::Integer(3), Literal::Integer(3)),
            (Literal::Float(5.5), Literal::Integer(5)),
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
            (Literal::Float(-5.5), Literal::Integer(-6)),
            (Literal::Float(-5.4), Literal::Integer(-5)),
            (Literal::Integer(0), Literal::Integer(0)),
            (Literal::Float(0.0), Literal::Integer(0)),
            (Literal::Integer(3), Literal::Integer(3)),
            (Literal::Float(5.4), Literal::Integer(5)),
            (Literal::Float(5.5), Literal::Integer(6)),
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
            (Literal::Float(-5.5), Literal::Integer(-5)),
            (Literal::Float(-5.56789), Literal::Integer(-5)),
            (Literal::Integer(0), Literal::Integer(0)),
            (Literal::Float(0.0), Literal::Integer(0)),
            (Literal::Integer(3), Literal::Integer(3)),
            (Literal::Float(5.5), Literal::Integer(5)),
            (Literal::Float(5.56789), Literal::Integer(5)),
        ]
        .into_iter()
        .for_each(|(input, expected)| {
            let actual = trunc(input);
            assert_eq!(actual, expected);
        });
    }
}
