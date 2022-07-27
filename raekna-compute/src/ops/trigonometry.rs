use raekna_common::expression::Literal;

use crate::ops::validate_and_wrap;

fn trig<F>(value: Literal, op: F) -> Option<Literal>
where
    F: Fn(f64) -> f64,
{
    use Literal::*;
    let result = match value {
        Integer(i) => i as f64,
        Float(f) => f,
    };
    let result = op(result);
    validate_and_wrap(result)
}

pub fn sin(value: Literal) -> Option<Literal> {
    trig(value, f64::sin)
}

pub fn cos(value: Literal) -> Option<Literal> {
    trig(value, f64::cos)
}

pub fn tan(value: Literal) -> Option<Literal> {
    trig(value, f64::tan)
}

pub fn sinh(value: Literal) -> Option<Literal> {
    trig(value, f64::sinh)
}

pub fn cosh(value: Literal) -> Option<Literal> {
    trig(value, f64::cosh)
}

pub fn tanh(value: Literal) -> Option<Literal> {
    trig(value, f64::tanh)
}

pub fn asin(value: Literal) -> Option<Literal> {
    trig(value, f64::asin)
}

pub fn acos(value: Literal) -> Option<Literal> {
    trig(value, f64::acos)
}

pub fn atan(value: Literal) -> Option<Literal> {
    trig(value, f64::atan)
}

pub fn asinh(value: Literal) -> Option<Literal> {
    trig(value, f64::asinh)
}

pub fn acosh(value: Literal) -> Option<Literal> {
    trig(value, f64::acosh)
}

pub fn atanh(value: Literal) -> Option<Literal> {
    trig(value, f64::atanh)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

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
        F2: Fn(Literal) -> Option<Literal>,
    {
        test_cases.into_iter().for_each(|value| {
            let value = *value;
            let expected = match value {
                Literal::Integer(i) => exp(i as f64),
                Literal::Float(f) => exp(f),
            };
            let expected = Literal::from(expected);

            let actual = act(value).unwrap();

            assert_eq!(actual, expected);
        });
    }

    mod test_sin {
        use super::*;

        #[test]
        fn positive_cases() {
            trig_test(&TEST_CASES, f64::sin, sin);
        }

        proptest! {
            #[test]
            fn proptest_f64(value: f64) {
                let expected_raw = value.sin();
                let expected = Literal::from(expected_raw);

                let value = Literal::Float(value);
                let actual = sin(value);

                if expected_raw.is_normal() || expected_raw == 0.0 {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                } else {
                    assert!(actual.is_none());
                }
            }

            #[test]
            fn proptest_i64(value: i64) {
                let expected_raw = (value as f64).sin();
                let expected = Literal::from(expected_raw);

                let value = Literal::Integer(value);
                let actual = sin(value);

                if expected_raw.is_normal() || expected_raw == 0.0 {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                } else {
                    assert!(actual.is_none());
                }
            }
        }
    }

    mod test_cos {
        use super::*;

        #[test]
        fn positive_cases() {
            trig_test(&TEST_CASES, f64::cos, cos);
        }

        proptest! {
            #[test]
            fn proptest_f64(value: f64) {
                let expected_raw = value.cos();
                let expected = Literal::from(expected_raw);

                let value = Literal::Float(value);
                let actual = cos(value);

                if expected_raw.is_normal() || expected_raw == 0.0 {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                } else {
                    assert!(actual.is_none());
                }
            }

            #[test]
            fn proptest_i64(value: i64) {
                let expected_raw = (value as f64).cos();
                let expected = Literal::from(expected_raw);

                let value = Literal::Integer(value);
                let actual = cos(value);

                if expected_raw.is_normal() || expected_raw == 0.0 {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                } else {
                    assert!(actual.is_none());
                }
            }
        }
    }

    mod test_tan {
        use super::*;

        #[test]
        fn positive_cases() {
            trig_test(&TEST_CASES, f64::tan, tan);
        }

        proptest! {
            #[test]
            fn proptest_f64(value: f64) {
                let expected_raw = value.tan();
                let expected = Literal::from(expected_raw);

                let value = Literal::Float(value);
                let actual = tan(value);

                if expected_raw.is_normal() || expected_raw == 0.0 {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                } else {
                    assert!(actual.is_none());
                }
            }

            #[test]
            fn proptest_i64(value: i64) {
                let expected_raw = (value as f64).tan();
                let expected = Literal::from(expected_raw);

                let value = Literal::Integer(value);
                let actual = tan(value);

                if expected_raw.is_normal() || expected_raw == 0.0 {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                } else {
                    assert!(actual.is_none());
                }
            }
        }
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
