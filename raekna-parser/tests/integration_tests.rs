use raekna_common::{
    expression::{Expression, Literal},
    function_name::FunctionName,
};

fn parse(input: &str) -> Expression {
    raekna_parser::parse(input).unwrap()
}

fn int(value: i64) -> Expression {
    let literal = Literal::Integer(value);
    Expression::Literal(literal)
}

fn float(value: f64) -> Expression {
    let literal = Literal::Float(value);
    Expression::Literal(literal)
}

macro_rules! generate_function_expression {
    ($funname:ident, $exprfunname:ident) => {
        pub fn $funname(args: Vec<Expression>) -> Expression {
            Expression::Function(FunctionName::$exprfunname, args)
        }
    };
}

generate_function_expression!(add_expr, Add);
generate_function_expression!(sub_expr, Subtract);
generate_function_expression!(mul_expr, Multiply);
generate_function_expression!(div_expr, Divide);
generate_function_expression!(mod_expr, Modulus);
generate_function_expression!(pow_expr, Power);
generate_function_expression!(sqrt_expr, SquareRoot);

mod parse_literals {
    use super::*;

    mod integers {
        use super::*;

        #[test]
        fn positive() {
            let input = "55";

            let expected = int(55);
            let actual = parse(input);

            assert_eq!(actual, expected);
        }
    }

    mod decimals {
        use super::*;

        #[test]
        fn positive() {
            let input = "11.1";

            let expected = float(11.1);
            let actual = parse(input);

            assert_eq!(actual, expected);
        }

        mod scientific_notation {
            use super::*;

            mod integer_factor {
                use super::*;

                #[test]
                fn pos_e_pos() {
                    let input = "5e3";

                    let expected = int(5000);
                    let actual = parse(input);

                    assert_eq!(actual, expected);
                }

                #[test]
                fn pos_e_neg() {
                    let input = "4e-5";

                    let expected = float(0.00004);
                    let actual = parse(input);

                    assert_eq!(actual, expected);
                }
            }

            mod decimal_factor {
                use super::*;

                #[test]
                fn pos_e_pos() {
                    let input = "16.234e6";

                    let expected = int(16234000);
                    let actual = parse(input);

                    assert_eq!(actual, expected);
                }

                #[test]
                fn pos_e_neg() {
                    let input = "7.987123e-3";

                    let expected = float(0.007987123);
                    let actual = parse(input);

                    assert_eq!(actual, expected);
                }
            }
        }
    }
}

mod operators {
    use super::*;

    #[test]
    fn add() {
        ["5 + 10", "5 +10", "5+ 10", "5+10"]
            .iter()
            .for_each(|input| {
                let expected = add_expr(vec![int(5), int(10)]);
                let actual = parse(input);

                assert_eq!(actual, expected);
            });
    }

    #[test]
    fn subtract() {
        ["5 - 10", "5 -10", "5- 10", "5-10"]
            .iter()
            .for_each(|input| {
                let expected = sub_expr(vec![int(5), int(10)]);
                let actual = parse(input);

                assert_eq!(actual, expected);
            });
    }

    #[test]
    fn multiply() {
        ["5 * 10", "5 *10", "5* 10", "5*10"]
            .iter()
            .for_each(|input| {
                let expected = mul_expr(vec![int(5), int(10)]);
                let actual = parse(input);

                assert_eq!(actual, expected);
            });
    }

    #[test]
    fn division() {
        ["5 / 10", "5 /10", "5/ 10", "5/10"]
            .iter()
            .for_each(|input| {
                let expected = div_expr(vec![int(5), int(10)]);
                let actual = parse(input);

                assert_eq!(actual, expected);
            });
    }

    #[test]
    fn modulo() {
        ["5 % 10", "5 %10", "5% 10", "5%10"]
            .iter()
            .for_each(|input| {
                let expected = mod_expr(vec![int(5), int(10)]);
                let actual = parse(input);

                assert_eq!(actual, expected);
            });
    }

    #[test]
    fn power() {
        ["5 ^ 10", "5 ^10", "5^ 10", "5^10"]
            .iter()
            .for_each(|input| {
                let expected = pow_expr(vec![int(5), int(10)]);
                let actual = parse(input);

                assert_eq!(actual, expected);
            });
    }

    mod operator_and_sign {
        use super::*;

        #[test]
        fn all_operators_with_positive_sign() {
            [
                ("5++10", add_expr(vec![int(5), int(10)])),
                ("5-+10", sub_expr(vec![int(5), int(10)])),
                ("5*+10", mul_expr(vec![int(5), int(10)])),
                ("5/+10", div_expr(vec![int(5), int(10)])),
                ("5%+10", mod_expr(vec![int(5), int(10)])),
                ("5^+10", pow_expr(vec![int(5), int(10)])),
            ]
            .into_iter()
            .for_each(|(input, expected)| {
                let actual = parse(input);
                assert_eq!(actual, expected);
            });
        }

        #[test]
        fn all_operators_with_negative_sign() {
            [
                ("5+-10", add_expr(vec![int(5), int(-10)])),
                ("5--10", sub_expr(vec![int(5), int(-10)])),
                ("5*-10", mul_expr(vec![int(5), int(-10)])),
                ("5/-10", div_expr(vec![int(5), int(-10)])),
                ("5%-10", mod_expr(vec![int(5), int(-10)])),
                ("5^-10", pow_expr(vec![int(5), int(-10)])),
            ]
            .into_iter()
            .for_each(|(input, expected)| {
                let actual = parse(input);
                assert_eq!(actual, expected);
            });
        }

        #[test]
        #[should_panic]
        fn multiply_as_sign() {
            let input = "5+*10";
            parse(input);
        }

        #[test]
        #[should_panic]
        fn divide_as_sign() {
            let input = "5+/10";
            parse(input);
        }

        #[test]
        #[should_panic]
        fn modulo_as_sign() {
            let input = "5+%10";
            parse(input);
        }

        #[test]
        #[should_panic]
        fn power_as_sign() {
            let input = "5+^10";
            parse(input);
        }
    }
}

mod order_of_operations {
    use super::*;

    #[test]
    fn add_subtract() {
        let input = "1 + 2 - 3";

        let expected = sub_expr(vec![add_expr(vec![int(1), int(2)]), int(3)]);
        let actual = parse(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn subtract_add() {
        let input = "1 - 2 + 3";

        let expected = add_expr(vec![sub_expr(vec![int(1), int(2)]), int(3)]);
        let actual = parse(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn multiply_divide() {
        let input = "1 * 2 / 3";

        let expected = div_expr(vec![mul_expr(vec![int(1), int(2)]), int(3)]);
        let actual = parse(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn divide_multiply() {
        let input = "1 / 2 * 3";

        let expected = mul_expr(vec![div_expr(vec![int(1), int(2)]), int(3)]);
        let actual = parse(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn add_multiply() {
        let input = "1 + 2 * 3";

        let expected = add_expr(vec![int(1), mul_expr(vec![int(2), int(3)])]);
        let actual = parse(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn multiply_add() {
        let input = "1 * 2 + 3";

        let expected = add_expr(vec![mul_expr(vec![int(1), int(2)]), int(3)]);
        let actual = parse(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn subtract_multiply() {
        let input = "1 - 2 * 3";

        let expected = sub_expr(vec![int(1), mul_expr(vec![int(2), int(3)])]);
        let actual = parse(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn multiply_subtract() {
        let input = "1 * 2 - 3";

        let expected = sub_expr(vec![mul_expr(vec![int(1), int(2)]), int(3)]);
        let actual = parse(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn add_divide() {
        let input = "1 + 2 / 3";

        let expected = add_expr(vec![int(1), div_expr(vec![int(2), int(3)])]);
        let actual = parse(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn divide_add() {
        let input = "1 / 2 + 3";

        let expected = add_expr(vec![div_expr(vec![int(1), int(2)]), int(3)]);
        let actual = parse(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn subtract_divide() {
        let input = "1 - 2 / 3";

        let expected = sub_expr(vec![int(1), div_expr(vec![int(2), int(3)])]);
        let actual = parse(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn divide_subtract() {
        let input = "1 / 2 - 3";

        let expected = sub_expr(vec![div_expr(vec![int(1), int(2)]), int(3)]);
        let actual = parse(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn add_power() {
        let input = "1 + 2 ^ 3";

        let expected = add_expr(vec![int(1), pow_expr(vec![int(2), int(3)])]);
        let actual = parse(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn power_add() {
        let input = "1 ^ 2 + 3";

        let expected = add_expr(vec![pow_expr(vec![int(1), int(2)]), int(3)]);
        let actual = parse(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn multiply_power() {
        let input = "1 * 2 ^ 3";

        let expected = mul_expr(vec![int(1), pow_expr(vec![int(2), int(3)])]);
        let actual = parse(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn power_multiply() {
        let input = "1 ^ 2 * 3";

        let expected = mul_expr(vec![pow_expr(vec![int(1), int(2)]), int(3)]);
        let actual = parse(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn parentheses() {
        let input = "1 * (2 + 3)";

        let expected = mul_expr(vec![int(1), add_expr(vec![int(2), int(3)])]);
        let actual = parse(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn function_with_negative_arguments() {
        let input = "add(-1, -2)";

        let expected = add_expr(vec![int(-1), int(-2)]);
        let actual = parse(input);

        assert_eq!(actual, expected);
    }
}

mod functions {
    use super::*;

    #[test]
    fn sqrt() {
        ["sqrt", "squareroot", "square_root"]
            .iter()
            .for_each(|fn_name| {
                let input = format!("{fn_name}(25)");

                let expected = Expression::Function(FunctionName::SquareRoot, vec![int(25)]);
                let actual = parse(&input);

                assert_eq!(actual, expected);
            });
    }

    #[test]
    fn factorial() {
        ["fact", "factorial"].iter().for_each(|fn_name| {
            let input = format!("{fn_name}(9)");

            let expected = Expression::Function(FunctionName::Factorial, vec![int(9)]);
            let actual = parse(&input);

            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn max() {
        ["max", "maximum"].iter().for_each(|fn_name| {
            let input = format!("{fn_name}(9, 6)");

            let expected = Expression::Function(FunctionName::Max, vec![int(9), int(6)]);
            let actual = parse(&input);

            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn min() {
        ["min", "minimum"].iter().for_each(|fn_name| {
            let input = format!("{fn_name}(9, 6)");

            let expected = Expression::Function(FunctionName::Min, vec![int(9), int(6)]);
            let actual = parse(&input);

            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn add() {
        let input = "add(9, 10)";

        let expected = Expression::Function(FunctionName::Add, vec![int(9), int(10)]);
        let actual = parse(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn subtract() {
        ["sub", "subtract"].iter().for_each(|fn_name| {
            let input = format!("{fn_name}(9, 2)");

            let expected = Expression::Function(FunctionName::Subtract, vec![int(9), int(2)]);
            let actual = parse(&input);

            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn multiply() {
        ["mul", "multiply"].iter().for_each(|fn_name| {
            let input = format!("{fn_name}(9, 2)");

            let expected = Expression::Function(FunctionName::Multiply, vec![int(9), int(2)]);
            let actual = parse(&input);

            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn divide() {
        ["div", "divide"].iter().for_each(|fn_name| {
            let input = format!("{fn_name}(9, 9)");

            let expected = Expression::Function(FunctionName::Divide, vec![int(9), int(9)]);
            let actual = parse(&input);

            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn modulus() {
        ["mod", "modulus"].iter().for_each(|fn_name| {
            let input = format!("{fn_name}(9, 3)");

            let expected = Expression::Function(FunctionName::Modulus, vec![int(9), int(3)]);
            let actual = parse(&input);

            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn power() {
        ["pow", "power"].iter().for_each(|fn_name| {
            let input = format!("{fn_name}(9, 2)");

            let expected = Expression::Function(FunctionName::Power, vec![int(9), int(2)]);
            let actual = parse(&input);

            assert_eq!(actual, expected);
        });
    }

    #[test]
    #[should_panic]
    fn invalid_function_name() {
        let input = "invalid_function(25)";
        parse(input);
    }
}

mod variables {
    use super::*;

    #[test]
    fn variable_definition() {
        let input = "my_var: 5";

        let expected = Expression::Variable("my_var".to_owned(), Box::new(int(5)));
        let actual = parse(input);

        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic]
    fn variable_definition_invalid_variable_name() {
        let input = "_my_var: 5";
        parse(input);
    }

    #[test]
    fn variable_reference() {
        let input = "5 + my_var";

        let expected = Expression::Function(
            FunctionName::Add,
            vec![int(5), Expression::VariableRef("my_var".to_owned())],
        );
        let actual = parse(input);

        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic]
    fn variable_reference_invalid_variable_name() {
        let input = "5 + _my_var";
        parse(input);
    }
}

mod combining_rules {
    use super::*;

    #[test]
    fn combine_all() {
        let input = "var_def: pow(var_ref, 5 / 2.0) * (1e2 + 2.2)";

        let expected = Expression::Variable(
            "var_def".to_owned(),
            Box::new(mul_expr(vec![
                pow_expr(vec![
                    Expression::VariableRef("var_ref".to_owned()),
                    div_expr(vec![int(5), int(2)]),
                ]),
                add_expr(vec![int(100), float(2.2)]),
            ])),
        );
        let actual = parse(input);

        assert_eq!(actual, expected);
    }
}
