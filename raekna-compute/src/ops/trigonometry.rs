use raekna_common::expression::Literal;

fn trig<F>(value: Literal, op: F) -> Literal
where
    F: Fn(f64) -> f64,
{
    use Literal::*;
    let value = match value {
        Integer(i) => i as f64,
        Float(f) => f,
    };
    Literal::from(op(value))
}

pub fn sin(value: Literal) -> Literal {
    trig(value, f64::sin)
}

pub fn cos(value: Literal) -> Literal {
    trig(value, f64::cos)
}

pub fn tan(value: Literal) -> Literal {
    trig(value, f64::tan)
}

pub fn sinh(value: Literal) -> Literal {
    trig(value, f64::sinh)
}

pub fn cosh(value: Literal) -> Literal {
    trig(value, f64::cosh)
}

pub fn tanh(value: Literal) -> Literal {
    trig(value, f64::tanh)
}

pub fn asin(value: Literal) -> Literal {
    trig(value, f64::asin)
}

pub fn acos(value: Literal) -> Literal {
    trig(value, f64::acos)
}

pub fn atan(value: Literal) -> Literal {
    trig(value, f64::atan)
}

pub fn asinh(value: Literal) -> Literal {
    trig(value, f64::asinh)
}

pub fn acosh(value: Literal) -> Literal {
    trig(value, f64::acosh)
}

pub fn atanh(value: Literal) -> Literal {
    trig(value, f64::atanh)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASES: [Literal; 6] = [
        Literal::Float(-0.2),
        Literal::Float(0.0),
        Literal::Float(0.95),
        Literal::Integer(-1),
        Literal::Integer(0),
        Literal::Integer(1),
    ];

    fn trig_test<F1, F2>(test_cases: &[Literal], exp: F1, act: F2)
    where
        F1: Fn(f64) -> f64,
        F2: Fn(Literal) -> Literal,
    {
        test_cases.into_iter().for_each(|value| {
            let value = *value;
            let expected = match value {
                Literal::Integer(i) => exp(i as f64),
                Literal::Float(f) => exp(f),
            };
            let expected = Literal::from(expected);

            let actual = act(value);

            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn test_sin() {
        trig_test(&TEST_CASES, f64::sin, sin);
    }

    #[test]
    fn test_cos() {
        trig_test(&TEST_CASES, f64::cos, cos);
    }

    #[test]
    fn test_tan() {
        trig_test(&TEST_CASES, f64::tan, tan);
    }

    #[test]
    fn test_sinh() {
        trig_test(&TEST_CASES, f64::sinh, sinh);
    }

    #[test]
    fn test_cosh() {
        trig_test(&TEST_CASES, f64::cosh, cosh);
    }

    #[test]
    fn test_tanh() {
        trig_test(&TEST_CASES, f64::tanh, tanh);
    }

    #[test]
    fn test_asin() {
        trig_test(&TEST_CASES, f64::asin, asin);
    }

    #[test]
    fn test_acos() {
        trig_test(&TEST_CASES, f64::acos, acos);
    }

    #[test]
    fn test_atan() {
        trig_test(&TEST_CASES, f64::atan, atan);
    }

    #[test]
    fn test_asinh() {
        trig_test(&TEST_CASES, f64::asinh, asinh);
    }

    #[test]
    fn test_acosh() {
        trig_test(
            &[Literal::Float(1.1), Literal::Integer(10)],
            f64::acosh,
            acosh,
        );
    }

    #[test]
    fn test_atanh() {
        trig_test(&TEST_CASES, f64::atanh, atanh);
    }
}
