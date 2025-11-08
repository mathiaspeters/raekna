use raekna_common::expression::Literal;

use crate::ops::validate_and_wrap;

fn trig<F>(value: Literal, op: F) -> Option<Literal>
where
    F: Fn(f64) -> f64,
{
    let value = value.as_f64();
    let result = op(value);
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
    use proptest::prelude::*;

    use super::*;

    mod test_sin {
        use super::*;

        #[test]
        fn positive_cases() {
            let test_cases = [
                (Literal::Integer(-25), Literal::Float(0.13235175009777303)),
                (Literal::Integer(0), Literal::Integer(0)),
                (Literal::Integer(10), Literal::Float(-0.5440211108893698)),
                (Literal::Float(-3.5), Literal::Float(0.35078322768961984)),
                (Literal::Float(0.0), Literal::Integer(0)),
                (Literal::Float(12.7), Literal::Float(0.13323204141994222)),
            ];
            test_cases.into_iter().for_each(|(input, expected)| {
                let actual = sin(input).unwrap();
                assert_eq!(actual, expected);
            });
        }

        proptest! {
            #[test]
            fn proptest_f64(value: f64) {
                if value.is_normal() || value == 0.0 {
                    let expected_raw = value.sin();
                    let expected = Literal::from(expected_raw);

                    let value = Literal::Float(value);
                    let actual = sin(value);

                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                }

            }

            #[test]
            fn proptest_i64(value: i64) {
                let expected_raw = (value as f64).sin();
                let expected = Literal::from(expected_raw);

                let value = Literal::Integer(value);
                let actual = sin(value);

                let actual = actual.unwrap();
                prop_assert_eq!(actual, expected);
            }
        }
    }

    mod test_cos {
        use super::*;

        #[test]
        fn positive_cases() {
            let test_cases = [
                (Literal::Integer(-8), Literal::Float(-0.14550003380861354)),
                (Literal::Integer(0), Literal::Integer(1)),
                (Literal::Integer(13), Literal::Float(0.9074467814501962)),
                (
                    Literal::Float(-14.14),
                    Literal::Float(-0.0028330550561391095),
                ),
                (Literal::Float(0.0), Literal::Integer(1)),
                (Literal::Float(5.6), Literal::Float(0.7755658785102496)),
            ];
            test_cases.into_iter().for_each(|(input, expected)| {
                let actual = cos(input).unwrap();
                assert_eq!(actual, expected);
            });
        }

        proptest! {
            #[test]
            fn proptest_f64(value: f64) {
                if value.is_normal() || value == 0.0 {
                    let expected_raw = value.cos();
                    let expected = Literal::from(expected_raw);

                    let value = Literal::Float(value);
                    let actual = cos(value);

                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                }
            }

            #[test]
            fn proptest_i64(value: i64) {
                let expected_raw = (value as f64).cos();
                let expected = Literal::from(expected_raw);

                let value = Literal::Integer(value);
                let actual = cos(value);

                let actual = actual.unwrap();
                prop_assert_eq!(actual, expected);
            }
        }
    }

    mod test_tan {
        use super::*;

        #[test]
        fn positive_cases() {
            let test_cases = [
                (Literal::Integer(-33), Literal::Float(75.31301480008509)),
                (Literal::Integer(0), Literal::Integer(0)),
                (Literal::Integer(15), Literal::Float(-0.8559934009085188)),
                (Literal::Float(-4.5), Literal::Float(-4.637332054551185)),
                (Literal::Float(0.0), Literal::Integer(0)),
                (Literal::Float(8.1), Literal::Float(-3.982398246755992)),
            ];
            test_cases.into_iter().for_each(|(input, expected)| {
                let actual = tan(input).unwrap();
                assert_eq!(actual, expected);
            });
        }

        proptest! {
            #[test]
            fn proptest_f64(value: f64) {
                if value.is_normal() || value == 0.0 {
                    let expected_raw = value.tan();
                    let expected = Literal::from(expected_raw);

                    let value = Literal::Float(value);
                    let actual = tan(value);

                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                }
            }

            #[test]
            fn proptest_i64(value: i64) {
                let expected_raw = (value as f64).tan();
                let expected = Literal::from(expected_raw);

                let value = Literal::Integer(value);
                let actual = tan(value);

                let actual = actual.unwrap();
                prop_assert_eq!(actual, expected);
            }
        }
    }

    mod test_sinh {
        use super::*;

        #[test]
        fn positive_cases() {
            let test_cases = [
                (Literal::Integer(-5), Literal::Float(-74.20321057778875)),
                (Literal::Integer(0), Literal::Integer(0)),
                (Literal::Integer(99), Literal::Float(4.944515159673473e42)),
                (Literal::Float(-2.22), Literal::Float(-4.549360878528647)),
                (Literal::Float(0.0), Literal::Integer(0)),
                (Literal::Float(67.89), Literal::Float(1.5248333457506416e29)),
            ];
            test_cases.into_iter().for_each(|(input, expected)| {
                let actual = sinh(input).unwrap();
                assert_eq!(actual, expected);
            });
        }

        proptest! {
            #[test]
            fn proptest_f64(value: f64) {
                if value.is_normal() || value == 0.0 {
                    let expected_raw = value.sinh();
                    let expected = Literal::from(expected_raw);

                    let value = Literal::Float(value);
                    let actual = sinh(value);

                    if expected_raw.is_normal() || expected_raw == 0.0 {
                        let actual = actual.unwrap();
                        prop_assert_eq!(actual, expected);
                    } else {
                        assert!(actual.is_none());
                    }
                }

            }

            #[test]
            fn proptest_i64(value: i64) {
                let expected_raw = (value as f64).sinh();
                let expected = Literal::from(expected_raw);

                let value = Literal::Integer(value);
                let actual = sinh(value);

                if expected_raw.is_normal() || expected_raw == 0.0 {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                } else {
                    assert!(actual.is_none());
                }
            }
        }
    }

    mod test_cosh {
        use super::*;

        #[test]
        fn positive_cases() {
            let test_cases = [
                (Literal::Integer(-6), Literal::Float(201.7156361224559)),
                (Literal::Integer(0), Literal::Integer(1)),
                (Literal::Integer(3), Literal::Float(10.067661995777765)),
                (Literal::Float(-0.123), Literal::Float(1.007574041754155)),
                (Literal::Float(0.0), Literal::Integer(1)),
                (Literal::Float(1.234), Literal::Float(1.8630338016984225)),
            ];
            test_cases.into_iter().for_each(|(input, expected)| {
                let actual = cosh(input).unwrap();
                assert_eq!(actual, expected);
            });
        }

        proptest! {
            #[test]
            fn proptest_f64(value: f64) {
                if value.is_normal() || value == 0.0 {
                    let expected_raw = value.cosh();
                    let expected = Literal::from(expected_raw);

                    let value = Literal::Float(value);
                    let actual = cosh(value);

                    if expected_raw.is_normal() || expected_raw == 0.0 {
                        let actual = actual.unwrap();
                        prop_assert_eq!(actual, expected);
                    } else {
                        assert!(actual.is_none());
                    }
                }
            }

            #[test]
            fn proptest_i64(value: i64) {
                let expected_raw = (value as f64).cosh();
                let expected = Literal::from(expected_raw);

                let value = Literal::Integer(value);
                let actual = cosh(value);

                if expected_raw.is_normal() || expected_raw == 0.0 {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                } else {
                    assert!(actual.is_none());
                }
            }
        }
    }

    mod test_tanh {
        use super::*;

        #[test]
        fn positive_cases() {
            let test_cases = [
                (Literal::Integer(-2), Literal::Float(-0.9640275800758169)),
                (Literal::Integer(0), Literal::Integer(0)),
                (Literal::Integer(1), Literal::Float(0.7615941559557649)),
                (Literal::Float(-0.876), Literal::Float(-0.704409765786962)),
                (Literal::Float(0.0), Literal::Integer(0)),
                (Literal::Float(0.0123), Literal::Float(0.012299379748535112)),
            ];
            test_cases.into_iter().for_each(|(input, expected)| {
                let actual = tanh(input).unwrap();
                assert_eq!(actual, expected);
            });
        }

        proptest! {
            #[test]
            fn proptest_f64(value: f64) {
                if value.is_normal() || value == 0.0 {
                    let expected_raw = value.tanh();
                    let expected = Literal::from(expected_raw);

                    let value = Literal::Float(value);
                    let actual = tanh(value);

                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                }
            }

            #[test]
            fn proptest_i64(value: i64) {
                let expected_raw = (value as f64).tanh();
                let expected = Literal::from(expected_raw);

                let value = Literal::Integer(value);
                let actual = tanh(value);

                let actual = actual.unwrap();
                prop_assert_eq!(actual, expected);
            }
        }
    }

    mod test_asin {
        use super::*;

        #[test]
        fn positive_cases() {
            let test_cases = [
                (
                    Literal::Integer(-1),
                    Literal::Float(-core::f64::consts::FRAC_PI_2),
                ),
                (Literal::Integer(0), Literal::Integer(0)),
                (
                    Literal::Integer(1),
                    Literal::Float(core::f64::consts::FRAC_PI_2),
                ),
                (
                    Literal::Float(-1.0),
                    Literal::Float(-core::f64::consts::FRAC_PI_2),
                ),
                (Literal::Float(-0.6), Literal::Float(-0.6435011087932844)),
                (Literal::Float(0.0), Literal::Integer(0)),
                (
                    Literal::Float(1.0),
                    Literal::Float(core::f64::consts::FRAC_PI_2),
                ),
                (Literal::Float(0.723), Literal::Float(0.8081349986659526)),
            ];
            test_cases.into_iter().for_each(|(input, expected)| {
                let actual = asin(input).unwrap();
                assert_eq!(actual, expected);
            });
        }

        #[test]
        fn negative_cases() {
            let test_cases = [
                Literal::Integer(-2),
                Literal::Integer(-10),
                Literal::Integer(2),
                Literal::Integer(12),
                Literal::Float(-4.0),
                Literal::Float(-1.000001),
                Literal::Float(1.000001),
                Literal::Float(25.5),
            ];
            test_cases.into_iter().for_each(|input| {
                let actual = asin(input);
                assert!(actual.is_none());
            });
        }

        proptest! {
            #[test]
            fn proptest_f64(value in -1_f64..=1.0) {
                let expected_raw = value.asin();
                let expected = Literal::from(expected_raw);

                let value = Literal::Float(value);
                let actual = asin(value);

                let actual = actual.unwrap();
                prop_assert_eq!(actual, expected);
            }

            #[test]
            fn proptest_i64(value in -1_i64..=1) {
                let expected_raw = (value as f64).asin();
                let expected = Literal::from(expected_raw);

                let value = Literal::Integer(value);
                let actual = asin(value);

                let actual = actual.unwrap();
                prop_assert_eq!(actual, expected);
            }
        }
    }

    mod test_acos {
        use super::*;

        #[test]
        fn positive_cases() {
            let test_cases = [
                (Literal::Integer(-1), Literal::Float(core::f64::consts::PI)),
                (
                    Literal::Integer(0),
                    Literal::Float(core::f64::consts::FRAC_PI_2),
                ),
                (Literal::Integer(1), Literal::Integer(0)),
                (Literal::Float(-1.0), Literal::Float(core::f64::consts::PI)),
                (Literal::Float(-0.9), Literal::Float(2.6905658417935308)),
                (
                    Literal::Float(0.0),
                    Literal::Float(core::f64::consts::FRAC_PI_2),
                ),
                (Literal::Float(1.0), Literal::Integer(0)),
                (Literal::Float(0.876), Literal::Float(0.5032910474523901)),
            ];
            test_cases.into_iter().for_each(|(input, expected)| {
                let actual = acos(input).unwrap();
                assert_eq!(actual, expected);
            });
        }

        #[test]
        fn negative_cases() {
            let test_cases = [
                Literal::Integer(-2),
                Literal::Integer(-10),
                Literal::Integer(2),
                Literal::Integer(12),
                Literal::Float(-4.0),
                Literal::Float(-1.000001),
                Literal::Float(1.000001),
                Literal::Float(25.5),
            ];
            test_cases.into_iter().for_each(|input| {
                let actual = acos(input);
                assert!(actual.is_none());
            });
        }

        proptest! {
            #[test]
            fn proptest_f64(value in -1_f64..=1.0) {
                let expected_raw = value.acos();
                let expected = Literal::from(expected_raw);

                let value = Literal::Float(value);
                let actual = acos(value);

                let actual = actual.unwrap();
                prop_assert_eq!(actual, expected);
            }

            #[test]
            fn proptest_i64(value in -1_i64..=1) {
                let expected_raw = (value as f64).acos();
                let expected = Literal::from(expected_raw);

                let value = Literal::Integer(value);
                let actual = acos(value);

                let actual = actual.unwrap();
                prop_assert_eq!(actual, expected);
            }
        }
    }

    mod test_atan {
        use super::*;

        #[test]
        fn positive_cases() {
            let test_cases = [
                (Literal::Integer(-23), Literal::Float(-1.5273454314033659)),
                (Literal::Integer(0), Literal::Integer(0)),
                (Literal::Integer(2), Literal::Float(1.1071487177940904)),
                (Literal::Float(-3.89), Literal::Float(-1.3191752492839313)),
                (Literal::Float(0.0), Literal::Integer(0)),
                (Literal::Float(9.1), Literal::Float(1.4613453776535332)),
            ];
            test_cases.into_iter().for_each(|(input, expected)| {
                let actual = atan(input).unwrap();
                assert_eq!(actual, expected);
            });
        }

        proptest! {
            #[test]
            fn proptest_f64(value: f64) {
                if value.is_normal() || value == 0.0 {
                    let expected_raw = value.atan();
                    let expected = Literal::from(expected_raw);

                    let value = Literal::Float(value);
                    let actual = atan(value);

                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                }
            }

            #[test]
            fn proptest_i64(value: i64) {
                let expected_raw = (value as f64).atan();
                let expected = Literal::from(expected_raw);

                let value = Literal::Integer(value);
                let actual = atan(value);

                let actual = actual.unwrap();
                prop_assert_eq!(actual, expected);
            }
        }
    }

    mod test_asinh {
        use super::*;

        #[test]
        fn positive_cases() {
            let test_cases = [
                (Literal::Integer(-78), Literal::Float(-5.04989709610426)),
                (Literal::Integer(0), Literal::Integer(0)),
                (Literal::Integer(15), Literal::Float(3.4023066454805946)),
                (Literal::Float(-3.33), Literal::Float(-1.9179382061866592)),
                (Literal::Float(0.0), Literal::Integer(0)),
                (Literal::Float(44.55), Literal::Float(4.48988527414627)),
            ];
            test_cases.into_iter().for_each(|(input, expected)| {
                let actual = asinh(input).unwrap();
                assert_eq!(actual, expected);
            });
        }

        proptest! {
            #[test]
            fn proptest_f64(value: f64) {
                if value.is_normal() || value == 0.0 {
                    let expected_raw = value.asinh();
                    let expected = Literal::from(expected_raw);

                    let value = Literal::Float(value);
                    let actual = asinh(value);

                    if expected_raw.is_normal() || expected_raw == 0.0 {
                        let actual = actual.unwrap();
                        prop_assert_eq!(actual, expected);
                    } else {
                        assert!(actual.is_none());
                    }
                }

            }

            #[test]
            fn proptest_i64(value: i64) {
                let expected_raw = (value as f64).asinh();
                let expected = Literal::from(expected_raw);

                let value = Literal::Integer(value);
                let actual = asinh(value);

                let actual = actual.unwrap();
                prop_assert_eq!(actual, expected);
            }
        }
    }

    mod test_acosh {
        use super::*;

        #[test]
        fn positive_cases() {
            let test_cases = [
                (Literal::Integer(1), Literal::Integer(0)),
                (Literal::Integer(11), Literal::Float(3.0889699048446033)),
                (Literal::Float(1.0), Literal::Integer(0)),
                (Literal::Float(15.6), Literal::Float(3.4393892235401156)),
            ];
            test_cases.into_iter().for_each(|(input, expected)| {
                let actual = acosh(input).unwrap();
                assert_eq!(actual, expected);
            });
        }

        #[test]
        fn negative_cases() {
            let test_cases = [
                Literal::Integer(-10),
                Literal::Integer(-1),
                Literal::Integer(0),
                Literal::Float(-44.0),
                Literal::Float(-2.0),
                Literal::Float(0.0),
                Literal::Float(0.99999),
            ];
            test_cases.into_iter().for_each(|input| {
                let actual = acosh(input);
                assert!(actual.is_none());
            });
        }

        proptest! {
            #[test]
            fn proptest_f64(value: f64) {
                if value.is_normal() || value == 0.0 {
                    let expected_raw = value.acosh();
                    let expected = Literal::from(expected_raw);

                    let value = Literal::Float(value);
                    let actual = acosh(value);

                    if expected_raw.is_normal() || expected_raw == 0.0 {
                        let actual = actual.unwrap();
                        prop_assert_eq!(actual, expected);
                    } else {
                        assert!(actual.is_none());
                    }
                }
            }

            #[test]
            fn proptest_i64(value: i64) {
                let expected_raw = (value as f64).acosh();
                let expected = Literal::from(expected_raw);

                let value = Literal::Integer(value);
                let actual = acosh(value);

                if expected_raw.is_normal() || expected_raw == 0.0 {
                    let actual = actual.unwrap();
                    prop_assert_eq!(actual, expected);
                } else {
                    assert!(actual.is_none());
                }
            }
        }
    }

    mod test_atanh {
        use super::*;

        #[test]
        fn positive_cases() {
            let test_cases = [
                (Literal::Integer(0), Literal::Integer(0)),
                (Literal::Float(-0.99999), Literal::Float(-6.103033822767855)),
                (Literal::Float(-0.2), Literal::Float(-0.20273255405408222)),
                (Literal::Float(0.0), Literal::Integer(0)),
                (Literal::Float(0.5), Literal::Float(0.5493061443340549)),
                (Literal::Float(0.99999), Literal::Float(6.1030338227611125)),
            ];
            test_cases.into_iter().for_each(|(input, expected)| {
                let actual = atanh(input).unwrap();
                assert_eq!(actual, expected);
            });
        }

        #[test]
        fn negative_cases() {
            let test_cases = [
                Literal::Integer(-1),
                Literal::Integer(-10),
                Literal::Integer(1),
                Literal::Integer(12),
                Literal::Float(-4.0),
                Literal::Float(-1.0),
                Literal::Float(1.0),
                Literal::Float(25.5),
            ];
            test_cases.into_iter().for_each(|input| {
                let actual = atanh(input);
                assert!(actual.is_none());
            });
        }

        proptest! {
            #[test]
            fn proptest_f64(value in -1.0_f64..1.0) {
                if value > -1.0 {
                    let expected_raw = value.atanh();
                    let expected = Literal::from(expected_raw);

                    let value = Literal::Float(value);
                    let actual = atanh(value);

                    let actual = actual.unwrap();
                        prop_assert_eq!(actual, expected);
                }
            }
        }
    }
}
