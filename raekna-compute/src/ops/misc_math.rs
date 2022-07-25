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

pub fn log(value: Literal, base: Literal) -> ComputeResult<Literal> {
    let b = match base {
        Literal::Integer(i) => i as f64,
        Literal::Float(f) => f,
    };
    match b {
        _ if b == 2.0 => log2(value),
        _ if b == 10.0 => log10(value),
        _ => {
            let v = match value {
                Literal::Integer(i) => i as f64,
                Literal::Float(f) => f,
            };
            let result = v.log(b);
            if result.is_normal() || result == 0.0 {
                Ok(Literal::from(result))
            } else {
                Err(ComputeError::InvalidLogarithm { value, base })
            }
        }
    }
}

pub fn log2(value: Literal) -> ComputeResult<Literal> {
    let v = match value {
        Literal::Integer(i) => i as f64,
        Literal::Float(f) => f,
    };
    let result = v.log2();
    if result.is_normal() || result == 0.0 {
        Ok(Literal::from(result))
    } else {
        Err(ComputeError::InvalidLogarithm {
            value,
            base: Literal::Integer(2),
        })
    }
}

pub fn log10(value: Literal) -> ComputeResult<Literal> {
    let v = match value {
        Literal::Integer(i) => i as f64,
        Literal::Float(f) => f,
    };
    let result = v.log10();
    if result.is_normal() || result == 0.0 {
        Ok(Literal::from(result))
    } else {
        Err(ComputeError::InvalidLogarithm {
            value,
            base: Literal::Integer(10),
        })
    }
}

pub fn ln(value: Literal) -> ComputeResult<Literal> {
    let v = match value {
        Literal::Integer(i) => i as f64,
        Literal::Float(f) => f,
    };
    let result = v.ln();
    if result.is_normal() || result == 0.0 {
        Ok(Literal::from(result))
    } else {
        Err(ComputeError::InvalidNaturalLogarithm(value))
    }
}

pub fn abs(value: Literal) -> Literal {
    match value {
        Literal::Integer(i) => Literal::Integer(i.abs()),
        Literal::Float(f) => Literal::from(f.abs()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::test_utils::{float, int};
    use proptest::prelude::*;

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
                let actual = sqrt(value).unwrap();
                assert_eq!(actual, expected);
            }
        }

        #[test]
        fn negative_cases() {
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

        proptest! {
            #[test]
            fn proptest_f64(f: f64) {
                let expected_raw = f.sqrt();
                let expected = Literal::from(expected_raw);

                let actual = Literal::Float(f);
                let actual = sqrt(actual);

                if expected_raw.is_normal() || expected_raw == 0.0 {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                } else {
                    let err = actual.unwrap_err();
                    prop_assert!(
                        matches!(
                            err,
                            ComputeError::InvalidSquareRoot(_)
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

                if expected_raw.is_normal() || expected_raw == 0.0 {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                } else {
                    let err = actual.unwrap_err();
                    prop_assert!(
                        matches!(
                            err,
                            ComputeError::InvalidSquareRoot(_)
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
                (int(27), int(3)),
                (int(-125), int(-5)),
                // Floats
                (float(7.8), float(1.9831924826807747)),
                (float(-11.1), float(-2.2306991044756197)),
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
                let actual = cbrt(actual);

                if expected_raw.is_normal() || expected_raw == 0.0 {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                } else {
                    let err = actual.unwrap_err();
                    prop_assert!(
                        matches!(
                            err,
                            ComputeError::InvalidCubeRoot(_)
                        )
                    );
                }
            }

            #[test]
            fn proptest_i64(i: i64) {
                let expected_raw = (i as f64).cbrt();
                let expected = Literal::from(expected_raw);

                let actual = Literal::Integer(i);
                let actual = cbrt(actual);

                if expected_raw.is_normal() || expected_raw == 0.0 {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                } else {
                    let err = actual.unwrap_err();
                    prop_assert!(
                        matches!(
                            err,
                            ComputeError::InvalidCubeRoot(_)
                        )
                    );
                }
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
                let actual = factorial(value).unwrap();
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
                    0 => Ok(Literal::Integer(0)),
                    1 => Ok(Literal::Integer(1)),
                    2 => Ok(Literal::Integer(2)),
                    3 => Ok(Literal::Integer(6)),
                    4 => Ok(Literal::Integer(24)),
                    5 => Ok(Literal::Integer(120)),
                    6 => Ok(Literal::Integer(720)),
                    7 => Ok(Literal::Integer(5040)),
                    8 => Ok(Literal::Integer(40320)),
                    9 => Ok(Literal::Integer(362880)),
                    10 => Ok(Literal::Integer(3628800)),
                    11 => Ok(Literal::Integer(39916800)),
                    12 => Ok(Literal::Integer(479001600)),
                    13 => Ok(Literal::Integer(6227020800)),
                    14 => Ok(Literal::Integer(87178291200)),
                    15 => Ok(Literal::Integer(1307674368000)),
                    16 => Ok(Literal::Integer(20922789888000)),
                    17 => Ok(Literal::Integer(355687428096000)),
                    18 => Ok(Literal::Integer(6402373705728000)),
                    19 => Ok(Literal::Integer(121645100408832000)),
                    20 => Ok(Literal::Integer(2432902008176640000)),
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
                let err = log(value, base).unwrap_err();
                match err {
                    ComputeError::InvalidLogarithm { .. } => {}
                    _ => {
                        dbg!(err);
                        assert!(false);
                    }
                }
            }
        }

        proptest! {
            #[test]
            fn proptest_f64(value: f64, base: f64) {
                let expected_raw = value.log(base);
                let expected = Literal::from(expected_raw);

                let value = Literal::Float(value);
                let base = Literal::Float(base);
                let actual = log(value, base);

                if expected_raw.is_normal() || expected_raw == 0.0 {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                } else {
                    match actual.unwrap_err() {
                        ComputeError::InvalidLogarithm {..} => {}
                        actual_err @ _ => {
                            dbg!(actual_err);
                            prop_assert!(false);
                        }
                    }
                }
            }

            #[test]
            fn proptest_i64(value: i64, base: i64) {
                let expected_raw = (value as f64).log(base as f64);
                let expected = Literal::from(expected_raw);

                let value = Literal::Integer(value);
                let base = Literal::Integer(base);
                let actual = log(value, base);

                if expected_raw.is_normal() || expected_raw == 0.0 {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                } else {
                    match actual.unwrap_err() {
                        ComputeError::InvalidLogarithm {..} => {}
                        actual_err @ _ => {
                            dbg!(actual_err);
                            prop_assert!(false);
                        }
                    }
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
                let err = log2(value).unwrap_err();
                match err {
                    ComputeError::InvalidLogarithm { value: v, .. } => {
                        assert_eq!(v, value)
                    }
                    _ => {
                        dbg!(err);
                        assert!(false);
                    }
                }
            }
        }

        proptest! {
            #[test]
            fn proptest_f64(value: f64) {
                let expected_raw = value.log2();
                let expected = Literal::from(expected_raw);

                let value = Literal::Float(value);
                let actual = log2(value);

                if expected_raw.is_normal() || expected_raw == 0.0 {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                } else {
                    match actual.unwrap_err() {
                        ComputeError::InvalidLogarithm {..} => {}
                        actual_err @ _ => {
                            dbg!(actual_err);
                            prop_assert!(false);
                        }
                    }
                }
            }

            #[test]
            fn proptest_i64(value: i64) {
                let expected_raw = (value as f64).log2();
                let expected = Literal::from(expected_raw);

                let value = Literal::Integer(value);
                let actual = log2(value);

                if expected_raw.is_normal() || expected_raw == 0.0 {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                } else {
                    match actual.unwrap_err() {
                        ComputeError::InvalidLogarithm {..} => {}
                        actual_err @ _ => {
                            dbg!(actual_err);
                            prop_assert!(false);
                        }
                    }
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
                let err = log10(value).unwrap_err();
                match err {
                    ComputeError::InvalidLogarithm { value: v, .. } => {
                        assert_eq!(v, value)
                    }
                    _ => {
                        dbg!(err);
                        assert!(false);
                    }
                }
            }
        }

        proptest! {
            #[test]
            fn proptest_f64(value: f64) {
                let expected_raw = value.log10();
                let expected = Literal::from(expected_raw);

                let value = Literal::Float(value);
                let actual = log10(value);

                if expected_raw.is_normal() || expected_raw == 0.0 {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                } else {
                    match actual.unwrap_err() {
                        ComputeError::InvalidLogarithm {..} => {}
                        actual_err @ _ => {
                            dbg!(actual_err);
                            prop_assert!(false);
                        }
                    }
                }
            }

            #[test]
            fn proptest_i64(value: i64) {
                let expected_raw = (value as f64).log10();
                let expected = Literal::from(expected_raw);

                let value = Literal::Integer(value);
                let actual = log10(value);

                if expected_raw.is_normal() || expected_raw == 0.0 {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                } else {
                    match actual.unwrap_err() {
                        ComputeError::InvalidLogarithm {..} => {}
                        actual_err @ _ => {
                            dbg!(actual_err);
                            prop_assert!(false);
                        }
                    }
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
                let err = ln(value).unwrap_err();
                assert!(matches!(err, ComputeError::InvalidNaturalLogarithm(lit) if lit == value))
            }
        }

        proptest! {
            #[test]
            fn proptest_f64(value: f64) {
                let expected_raw = value.ln();
                let expected = Literal::from(expected_raw);

                let value = Literal::Float(value);
                let actual = ln(value);

                if expected_raw.is_normal() || expected_raw == 0.0 {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                } else {
                    let err = actual.unwrap_err();
                    prop_assert!(matches!(
                        err,
                        ComputeError::InvalidNaturalLogarithm(_),
                    ))
                }
            }

            #[test]
            fn proptest_i64(value: i64) {
                let expected_raw = (value as f64).ln();
                let expected = Literal::from(expected_raw);

                let value = Literal::Integer(value);
                let actual = ln(value);

                if expected_raw.is_normal() || expected_raw == 0.0 {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                } else {
                    let err = actual.unwrap_err();
                    prop_assert!(matches!(
                        err,
                        ComputeError::InvalidNaturalLogarithm(_),
                    ))
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
                let actual = abs(input);
                assert_eq!(actual, expected);
            });
        }

        proptest! {
            #[test]
            fn proptest_f64(f: f64) {
                let expected_raw = f.abs();
                let expected = Literal::from(expected_raw);

                let actual = Literal::Float(f);
                let actual = abs(actual);

                prop_assert_eq!(actual, expected);
            }

            #[test]
            fn proptest_i64(i: i64) {
                let expected = i.abs();
                let expected = Literal::Integer(expected);

                let actual = Literal::Integer(i);
                let actual = abs(actual);

                prop_assert_eq!(actual, expected);
            }
        }
    }
}
