use raekna_common::expression::Literal;

use super::validate_and_wrap;
use crate::errors::{ComputeError, ComputeResult};

pub fn sqrt(value: Literal) -> ComputeResult<Option<Literal>> {
    let raw = value.as_f64();
    if raw < 0.0 {
        Err(ComputeError::InvalidSquareRoot(value))
    } else {
        let root = raw.sqrt();
        Ok(validate_and_wrap(root))
    }
}

pub fn cbrt(value: Literal) -> Option<Literal> {
    let value = value.as_f64();
    let root = value.cbrt();
    validate_and_wrap(root)
}

pub fn factorial(value: Literal) -> ComputeResult<Option<Literal>> {
    match value {
        Literal::Integer(val) if val < 0 => {
            // TODO: Support factorial with negative numbers at some point
            Err(ComputeError::InvalidFactorialArgument(value))
        }
        Literal::Integer(value) if value <= 20 => {
            let mut result = 1;
            for i in 1..=value {
                result *= i;
            }
            Ok(Some(Literal::Integer(result)))
        }
        _ => Err(ComputeError::InvalidFactorialArgument(value)),
    }
}

pub fn log(value: Literal, base: Literal) -> Option<Literal> {
    let b = base.as_f64();
    if b == 2.0 {
        log2(value)
    } else if b == 10.0 {
        log10(value)
    } else {
        let v = value.as_f64();
        let result = v.log(b);
        validate_and_wrap(result)
    }
}

pub fn log2(value: Literal) -> Option<Literal> {
    let v = value.as_f64();
    let result = v.log2();
    validate_and_wrap(result)
}

pub fn log10(value: Literal) -> Option<Literal> {
    let v = value.as_f64();
    let result = v.log10();
    validate_and_wrap(result)
}

pub fn ln(value: Literal) -> Option<Literal> {
    let v = value.as_f64();
    let result = v.ln();
    validate_and_wrap(result)
}

pub fn abs(value: Literal) -> Option<Literal> {
    let result = match value {
        Literal::Integer(i) => Literal::Integer(i.abs()),
        Literal::Float(f) => Literal::from(f.abs()),
    };
    Some(result)
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::ops::test_utils::{float, int};

    mod test_sqrt {
        use super::*;

        #[test]
        fn positive_cases() {
            let test_cases = [
                // Integers
                (int(0), int(0)),
                (int(2), float(1.4142135623730951)),
                (int(25), int(5)),
                // Floats
                (float(3.5), float(1.8708286933869707)),
            ];
            for (value, expected) in test_cases.into_iter() {
                let actual = sqrt(value).unwrap().unwrap();
                assert_eq!(actual, expected);
            }
        }

        #[test]
        fn negative_cases() {
            let test_cases = [float(f64::NAN), float(f64::INFINITY)];
            for value in test_cases.into_iter() {
                assert!(sqrt(value).unwrap().is_none());
            }

            let test_cases = [int(-2), float(f64::NEG_INFINITY)];
            for value in test_cases.into_iter() {
                let err = sqrt(value).unwrap_err();
                assert!(matches!(err, ComputeError::InvalidSquareRoot(_)));
            }
        }

        proptest! {
            #[test]
            fn proptest_f64(f: f64) {
                let expected_raw = f.sqrt();
                let expected = Literal::from(expected_raw);

                let actual = Literal::Float(f);
                let actual = sqrt(actual);

                if f >= 0.0 {
                    let actual = actual.unwrap().unwrap();
                    prop_assert_eq!(actual, expected);
                } else {
                    let err = actual.unwrap_err();
                    prop_assert!(
                        matches!(
                            err,
                            ComputeError::InvalidSquareRoot(arg) if arg == Literal::Float(f)
                        )
                    );
                }
            }

            #[test]
            fn proptest_i64(i: i64) {
                let expected_raw = (i as f64).sqrt();
                let expected = Literal::from(expected_raw);

                let actual = Literal::Integer(i);
                let actual = sqrt(actual);

                if i >= 0 {
                    let actual = actual.unwrap().unwrap();
                    prop_assert_eq!(actual, expected);
                } else {
                    let err = actual.unwrap_err();
                    prop_assert!(
                        matches!(
                            err,
                            ComputeError::InvalidSquareRoot(arg) if arg == Literal::Integer(i)
                        )
                    );
                }
            }
        }
    }

    mod test_cbrt {
        use super::*;

        #[test]
        fn positive_cases() {
            let test_cases = [
                // Integers
                (int(0), int(0)),
                (int(5), float(1.709975946676697)),
                (int(64), int(4)),
                (int(-125), int(-5)),
                // Floats
                (float(7.8), float(1.9831924826807747)),
                (float(-10.648), float(-2.2)),
            ];
            for (value, expected) in test_cases.into_iter() {
                let actual = cbrt(value).unwrap();
                assert_eq!(actual, expected);
            }
        }

        proptest! {
            #[test]
            fn proptest_f64(f: f64) {
                let expected_raw = f.cbrt();
                let expected = Literal::from(expected_raw);

                let actual = Literal::Float(f);
                let actual = cbrt(actual).unwrap();

                prop_assert_eq!(actual, expected);
            }

            #[test]
            fn proptest_i64(i: i64) {
                let expected_raw = (i as f64).cbrt();
                let expected = Literal::from(expected_raw);

                let actual = Literal::Integer(i);
                let actual = cbrt(actual).unwrap();

                prop_assert_eq!(actual, expected);
            }
        }
    }

    mod test_factorial {
        use super::*;

        #[test]
        fn positive_cases() {
            let test_cases = [
                (int(0), int(1)),
                (int(2), int(2)),
                (int(6), int(720)),
                (int(11), int(39_916_800)),
            ];
            for (value, expected) in test_cases.into_iter() {
                let actual = factorial(value).unwrap().unwrap();
                assert_eq!(actual, expected);
            }
        }

        #[test]
        fn negative_cases() {
            let test_cases = [int(-2), int(21), float(1.1)];
            for value in test_cases.into_iter() {
                factorial(value).unwrap_err();
            }
        }

        proptest! {
            #[test]
            fn proptest_f64(value: f64) {
                let value = Literal::Float(value);
                let err = factorial(value).unwrap_err();
                assert!(matches!(
                    err,
                    ComputeError::InvalidFactorialArgument(v) if v == value
                ))
            }

            #[test]
            fn proptest_i64(value: i64) {
                let raw = value;
                let value = Literal::from(value);
                let actual = factorial(value);
                let expected = match raw {
                     0 => Ok(Some(Literal::Integer(                        0))),
                     1 => Ok(Some(Literal::Integer(                        1))),
                     2 => Ok(Some(Literal::Integer(                        2))),
                     3 => Ok(Some(Literal::Integer(                        6))),
                     4 => Ok(Some(Literal::Integer(                       24))),
                     5 => Ok(Some(Literal::Integer(                      120))),
                     6 => Ok(Some(Literal::Integer(                      720))),
                     7 => Ok(Some(Literal::Integer(                    5_040))),
                     8 => Ok(Some(Literal::Integer(                   40_320))),
                     9 => Ok(Some(Literal::Integer(                  362_880))),
                    10 => Ok(Some(Literal::Integer(                3_628_800))),
                    11 => Ok(Some(Literal::Integer(               39_916_800))),
                    12 => Ok(Some(Literal::Integer(              479_001_600))),
                    13 => Ok(Some(Literal::Integer(            6_227_020_800))),
                    14 => Ok(Some(Literal::Integer(           87_178_291_200))),
                    15 => Ok(Some(Literal::Integer(        1_307_674_368_000))),
                    16 => Ok(Some(Literal::Integer(       20_922_789_888_000))),
                    17 => Ok(Some(Literal::Integer(      355_687_428_096_000))),
                    18 => Ok(Some(Literal::Integer(    6_402_373_705_728_000))),
                    19 => Ok(Some(Literal::Integer(  121_645_100_408_832_000))),
                    20 => Ok(Some(Literal::Integer(2_432_902_008_176_640_000))),
                    _ => Err(ComputeError::InvalidFactorialArgument(Literal::Integer(raw)))
                };
                assert_eq!(actual, expected);
            }
        }
    }

    mod test_log {
        use super::*;

        #[test]
        fn positive_cases() {
            let test_cases = [
                // Integers
                ((int(144), int(12)), int(2)),
                ((int(15), float(5.5)), float(1.588533938493162)),
                ((int(45), int(0)), int(0)),
                ((int(2), float(0.0)), int(0)),
                // Floats
                ((float(25.2), int(5)), float(2.0049509021675314)),
                ((float(30.9), float(1.1)), float(35.995695223187695)),
                ((float(42.24), int(0)), int(0)),
                ((float(1.2), float(0.0)), int(0)),
            ];
            for ((value, base), expected) in test_cases.into_iter() {
                let actual = log(value, base).unwrap();
                assert_eq!(actual, expected);
            }
        }

        #[test]
        fn negative_cases() {
            let test_cases = [
                // Value == 0
                (int(0), int(5)),
                (int(0), float(2.2)),
                (float(0.0), int(6)),
                (float(0.0), float(10.01)),
                // Value < 0
                (int(-1), int(7)),
                (int(-2), float(8.9)),
                (float(-3.4), int(10)),
                (float(-5.6), float(11.12)),
                // Base < 0
                (int(12), int(-6)),
                (int(11), float(-5.4)),
                (float(10.9), int(-3)),
                (float(8.7), float(-2.1)),
            ];
            for (value, base) in test_cases.into_iter() {
                let result = log(value, base);
                assert!(result.is_none());
            }
        }

        proptest! {
            #[test]
            fn proptest_f64_f64(value: f64, base: f64) {
                let expected_raw = value.log(base);
                let expected = Literal::from(expected_raw);

                let actual = log(Literal::Float(value), Literal::Float(base));

                if value <= 0.0 || base < 0.0 {
                    assert!(actual.is_none());
                } else {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                }
            }

            #[test]
            fn proptest_f64_i64(value: f64, base: i64) {
                let expected_raw = value.log(base as f64);
                let expected = Literal::from(expected_raw);

                let actual = log(Literal::Float(value), Literal::Integer(base));

                if value <= 0.0 || base < 0 {
                    assert!(actual.is_none());
                } else {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                }
            }

            #[test]
            fn proptest_i64_f64(value: i64, base: f64) {
                let expected_raw = (value as f64).log(base);
                let expected = Literal::from(expected_raw);

                let actual = log(Literal::Integer(value), Literal::Float(base));

                if value <= 0 || base < 0.0 {
                    assert!(actual.is_none());
                } else {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                }
            }

            #[test]
            fn proptest_i64_i64(value: i64, base: i64) {
                let expected_raw = (value as f64).log(base as f64);
                let expected = Literal::from(expected_raw);

                let actual = log(Literal::Integer(value), Literal::Integer(base));

                if value <= 0 || base < 0 {
                    assert!(actual.is_none());
                } else {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                }
            }
        }
    }

    mod test_log2 {
        use super::*;

        #[test]
        fn positive_cases() {
            let test_cases = [
                // Integers
                (int(16), int(4)),
                // Floats
                (float(5.3), float(2.4059923596758366)),
            ];
            for (value, expected) in test_cases.into_iter() {
                let actual = log2(value).unwrap();
                assert_eq!(actual, expected);
            }
        }

        #[test]
        fn negative_cases() {
            let test_cases = [
                // Integers
                int(-8),
                int(0),
                // Floats
                float(-6.5),
                float(0.0),
            ];
            for value in test_cases.into_iter() {
                let result = log2(value);
                assert!(result.is_none());
            }
        }

        proptest! {
            #[test]
            fn proptest_f64(value: f64) {
                let expected_raw = value.log2();
                let expected = Literal::from(expected_raw);

                let actual = log2(Literal::Float(value));

                if value <= 0.0 {
                    assert!(actual.is_none());
                } else {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                }
            }

            #[test]
            fn proptest_i64(value: i64) {
                let expected_raw = (value as f64).log2();
                let expected = Literal::from(expected_raw);

                let actual = log2(Literal::Integer(value));

                if value <= 0 {
                    assert!(actual.is_none());
                } else {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                }
            }
        }
    }

    mod test_log10 {
        use super::*;

        #[test]
        fn positive_cases() {
            let test_cases = [
                // Integers
                (int(100), int(2)),
                // Floats
                (float(10.5), float(1.021189299069938)),
            ];
            for (value, expected) in test_cases.into_iter() {
                let actual = log10(value).unwrap();
                assert_eq!(actual, expected);
            }
        }

        #[test]
        fn negative_cases() {
            let test_cases = [
                // Integers
                int(-1000),
                int(0),
                // Floats
                float(-100.55),
                float(0.0),
            ];
            for value in test_cases.into_iter() {
                let result = log10(value);
                assert!(result.is_none());
            }
        }

        proptest! {
            #[test]
            fn proptest_f64(value: f64) {
                let expected_raw = value.log10();
                let expected = Literal::from(expected_raw);

                let actual = log10(Literal::Float(value));

                if value <= 0.0 {
                    assert!(actual.is_none());
                } else {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                }
            }

            #[test]
            fn proptest_i64(value: i64) {
                let expected_raw = (value as f64).log10();
                let expected = Literal::from(expected_raw);

                let actual = log10(Literal::Integer(value));

                if value <= 0 {
                    assert!(actual.is_none());
                } else {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                }
            }
        }
    }

    mod test_ln {
        use super::*;

        #[test]
        fn positive_cases() {
            let test_cases = [
                // Integers
                (int(55), float(4.007333185232471)),
                // Floats
                (float(25.2), float(3.2268439945173775)),
            ];
            for (value, expected) in test_cases.into_iter() {
                let actual = ln(value).unwrap();
                assert_eq!(actual, expected);
            }
        }

        #[test]
        fn negative_cases() {
            let test_cases = [
                // Integers
                int(-43),
                int(0),
                // Floats
                float(-22.33),
                float(0.0),
            ];
            for value in test_cases.into_iter() {
                let result = ln(value);
                assert!(result.is_none());
            }
        }

        proptest! {
            #[test]
            fn proptest_f64(value: f64) {
                let expected_raw = value.ln();
                let expected = Literal::from(expected_raw);

                let actual = ln(Literal::Float(value));

                if value <= 0.0 {
                    assert!(actual.is_none());
                } else {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                }
            }

            #[test]
            fn proptest_i64(value: i64) {
                let expected_raw = (value as f64).ln();
                let expected = Literal::from(expected_raw);

                let actual = ln(Literal::Integer(value));

                if value <= 0 {
                    assert!(actual.is_none());
                } else {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                }
            }
        }
    }

    mod test_abs {
        use super::*;

        #[test]
        fn positive_cases() {
            [
                (Literal::Integer(-2), Literal::Integer(2)),
                (Literal::Integer(0), Literal::Integer(0)),
                (Literal::Integer(7), Literal::Integer(7)),
                (Literal::Float(-1.5), Literal::Float(1.5)),
                (Literal::Float(5.6), Literal::Float(5.6)),
            ]
            .into_iter()
            .for_each(|(input, expected)| {
                let actual = abs(input).unwrap();
                assert_eq!(actual, expected);
            });
        }

        proptest! {
            #[test]
            fn proptest_f64(f: f64) {
                let expected_raw = f.abs();
                let expected = Literal::from(expected_raw);

                let actual = Literal::Float(f);
                let actual = abs(actual).unwrap();

                prop_assert_eq!(actual, expected);
            }

            #[test]
            fn proptest_i64(i: i64) {
                let expected = i.abs();
                let expected = Literal::Integer(expected);

                let actual = Literal::Integer(i);
                let actual = abs(actual).unwrap();

                prop_assert_eq!(actual, expected);
            }
        }
    }
}
