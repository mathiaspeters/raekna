use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while},
    character::{
        complete::{char, one_of},
        is_alphanumeric,
    },
    combinator::{map, map_res, opt, recognize, verify},
    error::{ErrorKind, ParseError},
    multi::{many0, many1, separated_list0},
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};
use number_parsers::*;
use text_parsers::*;

use crate::lexer::{
    token::{Literal, Operator, Token},
    token_tree::TokenTree,
};

pub fn parse_number(input: &str) -> IResult<&str, Token> {
    let (remaining, literal) = preceded(
        whitespace,
        alt((
            map(scientific_notation, |f| {
                if f.fract().abs() < f64::EPSILON {
                    Literal::Integer(f as i64)
                } else {
                    Literal::Float(f)
                }
            }),
            map(float, |f| {
                if f.fract().abs() < f64::EPSILON {
                    Literal::Integer(f as i64)
                } else {
                    Literal::Float(f)
                }
            }),
            map(integer, Literal::Integer),
        )),
    )(input)?;
    Ok((remaining, Token::Literal(literal)))
}

pub fn operator(input: &str) -> IResult<&str, Token> {
    let (remaining, operator) = preceded(
        whitespace,
        alt((
            map(char('+'), |_| Operator::Add),
            map(char('-'), |_| Operator::Subtract),
            map(char('*'), |_| Operator::Multiply),
            map(char('/'), |_| Operator::Divide),
            map(char('%'), |_| Operator::Modulo),
            map(char('^'), |_| Operator::Power),
        )),
    )(input)?;
    Ok((remaining, Token::Operator(operator)))
}

pub fn nested(input: &str) -> IResult<&str, Token> {
    map_res(preceded(whitespace, parentheses()), |n| {
        TokenTree::parse_input(n).map(|(_, token_tree)| Token::Nested(token_tree))
    })(input)
}

pub fn function(input: &str) -> IResult<&str, Token> {
    map(
        map_res(
            pair(
                preceded(whitespace, identifier),
                preceded(whitespace, parentheses()),
            ),
            |(f_name, e)| function_arguments(e).map(|e| (f_name, e)),
        ),
        |(f_name, (_, args))| Token::Function(f_name.to_owned(), args),
    )(input)
}

pub fn variable_definition(input: &str) -> IResult<&str, Token> {
    map(
        preceded(
            whitespace,
            pair(identifier, preceded(whitespace, char(':'))),
        ),
        |(ident, _)| Token::VariableDefinition(ident.to_owned()),
    )(input)
}

pub fn variable_reference(input: &str) -> IResult<&str, Token> {
    map(preceded(whitespace, identifier), |ident| {
        Token::VariableReference(ident.to_owned())
    })(input)
}

mod number_parsers {
    use super::*;

    pub fn scientific_notation(input: &str) -> IResult<&str, f64> {
        // Factor can be either float or integer
        let factor = alt((float, map(integer, |i| i as f64)));
        // Exponent can only be an integer
        let exponent = integer;
        map_res(recognize(tuple((factor, one_of("eE"), exponent))), to_f64)(input)
    }

    /// Parses floating point numbers
    /// A floating point number must include a period with at least on digit on at least one side
    pub fn float(input: &str) -> IResult<&str, f64> {
        map_res(
            alt((
                recognize(tuple((opt(char('-')), char('.'), decimal))),
                recognize(tuple((opt(char('-')), decimal, char('.'), opt(decimal)))),
            )),
            to_f64,
        )(input)
    }

    pub fn integer(input: &str) -> IResult<&str, i64> {
        map_res(recognize(tuple((opt(char('-')), decimal))), to_i64)(input)
    }

    fn decimal(input: &str) -> IResult<&str, &str> {
        recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
    }

    fn to_i64(input: &str) -> Result<i64, std::num::ParseIntError> {
        input.parse::<i64>()
    }

    fn to_f64(input: &str) -> Result<f64, std::num::ParseFloatError> {
        input.parse::<f64>()
    }
}

mod text_parsers {
    use super::*;

    pub fn identifier(input: &str) -> IResult<&str, &str> {
        verify(
            take_while(|c| c == '_' || is_alphanumeric(c as u8)),
            |s: &str| s.chars().next().map(|c| c.is_alphabetic()).unwrap_or(false),
        )(input)
    }

    pub fn function_arguments(input: &str) -> IResult<&str, Vec<TokenTree>> {
        map_res(
            separated_list0(
                preceded(whitespace, tag(",")),
                preceded(whitespace, is_not(",")),
            ),
            |args| {
                args.into_iter()
                    .map(|s| TokenTree::parse_input(s).map(|(_, a)| a))
                    .collect::<Result<Vec<_>, _>>()
            },
        )(input)
    }

    pub fn parentheses() -> impl Fn(&str) -> IResult<&str, &str> {
        move |input: &str| {
            let mut bracket_counter = 0;
            for (index, c) in input.chars().enumerate() {
                if index == 0 {
                    if c != '(' {
                        return Err(nom::Err::Error(nom::error::Error::from_error_kind(
                            input,
                            ErrorKind::TakeUntil,
                        )));
                    } else {
                        continue;
                    }
                }
                if c == '(' {
                    bracket_counter += 1;
                } else if c == ')' {
                    if bracket_counter == 0 {
                        return Ok((&input[index + 1..], input[1..index].trim()));
                    } else {
                        bracket_counter -= 1;
                    }
                }
            }

            Err(nom::Err::Error(nom::error::Error::from_error_kind(
                input,
                ErrorKind::TakeUntil,
            )))
        }
    }

    pub fn whitespace(input: &str) -> IResult<&str, &str> {
        take_while(|c: char| c.is_whitespace())(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod internal_parsers {
        use super::*;

        #[test]
        fn test_parentheses() {
            let input = "(1 + 2 ) * 3";

            let expected = Token::Nested(TokenTree {
                num_operators: 1,
                tokens: vec![
                    Token::Literal(Literal::Integer(1)),
                    Token::Operator(Operator::Add),
                    Token::Literal(Literal::Integer(2)),
                ],
            });
            let (rem, actual) = nested(input).unwrap();

            assert_eq!(rem, " * 3");
            assert_eq!(actual, expected);
        }
    }

    mod parse_number {
        use super::*;

        #[test]
        #[should_panic]
        fn empty_input() {
            let input = "";
            parse_number(input).unwrap();
        }

        mod integers {
            use super::*;

            #[test]
            fn positive() {
                let input = "55";

                let expected = Token::Literal(Literal::Integer(55));
                let (rem, actual) = parse_number(input).unwrap();

                assert!(rem.is_empty());
                assert_eq!(actual, expected);
            }

            #[test]
            fn negative() {
                let input = "-1234";

                let expected = Token::Literal(Literal::Integer(-1234));
                let (rem, actual) = parse_number(input).unwrap();

                assert!(rem.is_empty());
                assert_eq!(actual, expected);
            }
        }

        mod decimals {
            use super::*;

            #[test]
            fn no_fract_becomes_integer() {
                let input = "11.0";

                let expected = Token::Literal(Literal::Integer(11));
                let (rem, actual) = parse_number(input).unwrap();

                assert!(rem.is_empty());
                assert_eq!(actual, expected);
            }

            #[test]
            fn positive() {
                let input = "11.1";

                let expected = Token::Literal(Literal::Float(11.1));
                let (rem, actual) = parse_number(input).unwrap();

                assert!(rem.is_empty());
                assert_eq!(actual, expected);
            }

            #[test]
            fn negative() {
                let input = "-98.765";

                let expected = Token::Literal(Literal::Float(-98.765));
                let (rem, actual) = parse_number(input).unwrap();

                assert!(rem.is_empty());
                assert_eq!(actual, expected);
            }

            mod scientific_notation {
                use super::*;

                mod integer_factor {
                    use super::*;

                    #[test]
                    fn pos_e_pos() {
                        let input = "5e9";

                        let expected = Token::Literal(Literal::Integer(5000000000));
                        let (rem, actual) = parse_number(input).unwrap();

                        assert!(rem.is_empty());
                        assert_eq!(actual, expected);
                    }

                    #[test]
                    fn pos_e_neg() {
                        let input = "4e-5";

                        let expected = Token::Literal(Literal::Float(0.00004));
                        let (rem, actual) = parse_number(input).unwrap();

                        assert!(rem.is_empty());
                        assert_eq!(actual, expected);
                    }

                    #[test]
                    fn neg_e_pos() {
                        let input = "-6e7";

                        let expected = Token::Literal(Literal::Integer(-60000000));
                        let (rem, actual) = parse_number(input).unwrap();

                        assert!(rem.is_empty());
                        assert_eq!(actual, expected);
                    }

                    #[test]
                    fn neg_e_neg() {
                        let input = "-2e-3";

                        let expected = Token::Literal(Literal::Float(-0.002));
                        let (rem, actual) = parse_number(input).unwrap();

                        assert!(rem.is_empty());
                        assert_eq!(actual, expected);
                    }
                }

                mod decimal_factor {
                    use super::*;

                    #[test]
                    fn pos_e_pos() {
                        let input = "16.234e4";

                        let expected = Token::Literal(Literal::Integer(162340));
                        let (rem, actual) = parse_number(input).unwrap();

                        assert!(rem.is_empty());
                        assert_eq!(actual, expected);
                    }

                    #[test]
                    fn pos_e_neg() {
                        let input = "7.987123e-5";

                        let expected = Token::Literal(Literal::Float(0.00007987123));
                        let (rem, actual) = parse_number(input).unwrap();

                        assert!(rem.is_empty());
                        assert_eq!(actual, expected);
                    }

                    #[test]
                    fn neg_e_pos() {
                        let input = "-2.95e5";

                        let expected = Token::Literal(Literal::Integer(-295000));
                        let (rem, actual) = parse_number(input).unwrap();

                        assert!(rem.is_empty());
                        assert_eq!(actual, expected);
                    }

                    #[test]
                    fn neg_e_neg() {
                        let input = "-123.456e-3";

                        let expected = Token::Literal(Literal::Float(-0.123456));
                        let (rem, actual) = parse_number(input).unwrap();

                        assert!(rem.is_empty());
                        assert_eq!(actual, expected);
                    }
                }
            }
        }

        mod whitespace {
            use super::*;

            #[test]
            #[should_panic]
            fn just_whitespace() {
                let input = "  ";
                parse_number(input).unwrap();
            }

            #[test]
            fn whitespace_before() {
                let input = " 10";

                let expected = Token::Literal(Literal::Integer(10));
                let (rem, actual) = parse_number(input).unwrap();

                assert!(rem.is_empty());
                assert_eq!(actual, expected);
            }
        }
    }

    mod operator {
        use super::*;

        #[test]
        #[should_panic]
        fn empty_input() {
            let input = "";
            operator(input).unwrap();
        }

        #[test]
        fn add() {
            let input = "+";

            let expected = Token::Operator(Operator::Add);
            let (rem, actual) = operator(input).unwrap();

            assert!(rem.is_empty());
            assert_eq!(actual, expected);
        }

        #[test]
        fn subtract() {
            let input = "-";

            let expected = Token::Operator(Operator::Subtract);
            let (rem, actual) = operator(input).unwrap();

            assert!(rem.is_empty());
            assert_eq!(actual, expected);
        }

        #[test]
        fn multiply() {
            let input = "*";

            let expected = Token::Operator(Operator::Multiply);
            let (rem, actual) = operator(input).unwrap();

            assert!(rem.is_empty());
            assert_eq!(actual, expected);
        }

        #[test]
        fn divide() {
            let input = "/";

            let expected = Token::Operator(Operator::Divide);
            let (rem, actual) = operator(input).unwrap();

            assert!(rem.is_empty());
            assert_eq!(actual, expected);
        }

        #[test]
        fn modulo() {
            let input = "%";

            let expected = Token::Operator(Operator::Modulo);
            let (rem, actual) = operator(input).unwrap();

            assert!(rem.is_empty());
            assert_eq!(actual, expected);
        }

        #[test]
        fn power() {
            let input = "^";

            let expected = Token::Operator(Operator::Power);
            let (rem, actual) = operator(input).unwrap();

            assert!(rem.is_empty());
            assert_eq!(actual, expected);
        }

        mod whitespace {
            use super::*;

            #[test]
            #[should_panic]
            fn just_whitespace() {
                let input = "  ";
                operator(input).unwrap();
            }

            #[test]
            fn whitespace_before() {
                let input = " +";

                let expected = Token::Operator(Operator::Add);
                let (rem, actual) = operator(input).unwrap();

                assert!(rem.is_empty());
                assert_eq!(actual, expected);
            }
        }
    }

    mod nested {
        use super::*;

        #[test]
        #[should_panic]
        fn empty_input() {
            let input = "";
            nested(input).unwrap();
        }

        #[test]
        fn empty_parentheses() {
            let input = "()";

            let expected = Token::Nested(TokenTree {
                num_operators: 0,
                tokens: vec![],
            });
            let (rem, actual) = nested(input).unwrap();

            assert!(rem.is_empty());
            assert_eq!(actual, expected);
        }

        #[test]
        fn simple_case() {
            let input = "(5)";

            let expected = Token::Nested(TokenTree {
                num_operators: 0,
                tokens: vec![Token::Literal(Literal::Integer(5))],
            });
            let (rem, actual) = nested(input).unwrap();

            assert!(rem.is_empty());
            assert_eq!(actual, expected);
        }

        #[test]
        fn one_level_of_nesting() {
            let input = "((10 + 2) * 5)";

            let expected = Token::Nested(TokenTree {
                num_operators: 1,
                tokens: vec![
                    Token::Nested(TokenTree {
                        num_operators: 1,
                        tokens: vec![
                            Token::Literal(Literal::Integer(10)),
                            Token::Operator(Operator::Add),
                            Token::Literal(Literal::Integer(2)),
                        ],
                    }),
                    Token::Operator(Operator::Multiply),
                    Token::Literal(Literal::Integer(5)),
                ],
            });
            let (rem, actual) = nested(input).unwrap();

            assert!(rem.is_empty());
            assert_eq!(actual, expected);
        }

        #[test]
        fn two_levels_of_nesting() {
            let input = "((10 + (1 / 1)) * 5)";

            let expected = Token::Nested(TokenTree {
                num_operators: 1,
                tokens: vec![
                    Token::Nested(TokenTree {
                        num_operators: 1,
                        tokens: vec![
                            Token::Literal(Literal::Integer(10)),
                            Token::Operator(Operator::Add),
                            Token::Nested(TokenTree {
                                num_operators: 1,
                                tokens: vec![
                                    Token::Literal(Literal::Integer(1)),
                                    Token::Operator(Operator::Divide),
                                    Token::Literal(Literal::Integer(1)),
                                ],
                            }),
                        ],
                    }),
                    Token::Operator(Operator::Multiply),
                    Token::Literal(Literal::Integer(5)),
                ],
            });
            let (rem, actual) = nested(input).unwrap();

            assert!(rem.is_empty());
            assert_eq!(actual, expected);
        }

        mod whitespace {
            use super::*;

            #[test]
            #[should_panic]
            fn just_whitespace() {
                let input = "  ";
                nested(input).unwrap();
            }

            #[test]
            fn whitespace_before() {
                let input = " (5 + 1)";

                let expected = Token::Nested(TokenTree {
                    num_operators: 1,
                    tokens: vec![
                        Token::Literal(Literal::Integer(5)),
                        Token::Operator(Operator::Add),
                        Token::Literal(Literal::Integer(1)),
                    ],
                });
                let (rem, actual) = nested(input).unwrap();

                assert!(rem.is_empty());
                assert_eq!(actual, expected);
            }

            #[test]
            fn whitespace_after_opening_parenthesis() {
                let input = "( 5 + 1)";

                let expected = Token::Nested(TokenTree {
                    num_operators: 1,
                    tokens: vec![
                        Token::Literal(Literal::Integer(5)),
                        Token::Operator(Operator::Add),
                        Token::Literal(Literal::Integer(1)),
                    ],
                });
                let (rem, actual) = nested(input).unwrap();

                assert!(rem.is_empty());
                assert_eq!(actual, expected);
            }

            #[test]
            fn whitespace_before_closing_parenthesis() {
                let input = "(5 + 1 )";

                let expected = Token::Nested(TokenTree {
                    num_operators: 1,
                    tokens: vec![
                        Token::Literal(Literal::Integer(5)),
                        Token::Operator(Operator::Add),
                        Token::Literal(Literal::Integer(1)),
                    ],
                });
                let (rem, actual) = nested(input).unwrap();

                assert!(rem.is_empty());
                assert_eq!(actual, expected);
            }
        }
    }

    mod function {
        use super::*;

        #[test]
        #[should_panic]
        fn empty_input() {
            let input = "";
            function(input).unwrap();
        }

        #[test]
        fn one_simple_argument() {
            let input = "my_fn(5)";

            let expected = Token::Function(
                "my_fn".to_owned(),
                vec![TokenTree {
                    num_operators: 0,
                    tokens: vec![Token::Literal(Literal::Integer(5))],
                }],
            );
            let (rem, actual) = function(input).unwrap();

            assert!(rem.is_empty());
            assert_eq!(actual, expected);
        }

        #[test]
        fn two_simple_arguments() {
            let input = "my_fn(5, 10)";

            let expected = Token::Function(
                "my_fn".to_owned(),
                vec![
                    TokenTree {
                        num_operators: 0,
                        tokens: vec![Token::Literal(Literal::Integer(5))],
                    },
                    TokenTree {
                        num_operators: 0,
                        tokens: vec![Token::Literal(Literal::Integer(10))],
                    },
                ],
            );
            let (rem, actual) = function(input).unwrap();

            assert!(rem.is_empty());
            assert_eq!(actual, expected);
        }

        #[test]
        fn nested_argument() {
            let input = "my_fn(5 + (2 + 1))";

            let expected = Token::Function(
                "my_fn".to_owned(),
                vec![TokenTree {
                    num_operators: 1,
                    tokens: vec![
                        Token::Literal(Literal::Integer(5)),
                        Token::Operator(Operator::Add),
                        Token::Nested(TokenTree {
                            num_operators: 1,
                            tokens: vec![
                                Token::Literal(Literal::Integer(2)),
                                Token::Operator(Operator::Add),
                                Token::Literal(Literal::Integer(1)),
                            ],
                        }),
                    ],
                }],
            );
            let (rem, actual) = function(input).unwrap();

            assert!(rem.is_empty());
            assert_eq!(actual, expected);
        }

        #[test]
        #[should_panic]
        fn cannot_start_with_number() {
            let input = "1my_fn(6)";
            function(input).unwrap();
        }

        #[test]
        #[should_panic]
        fn cannot_start_with_underscore() {
            let input = "_my_fn(5)";
            function(input).unwrap();
        }

        mod whitespace {
            use super::*;

            #[test]
            #[should_panic]
            fn just_whitespace() {
                let input = "  ";
                function(input).unwrap();
            }

            #[test]
            fn whitespace_before() {
                let input = " my_fn()";

                let expected = Token::Function("my_fn".to_owned(), vec![]);
                let (rem, actual) = function(input).unwrap();

                assert!(rem.is_empty());
                assert_eq!(actual, expected);
            }

            #[test]
            fn whitespace_before_parenthesis() {
                let input = "my_fn ()";

                let expected = Token::Function("my_fn".to_owned(), vec![]);
                let (rem, actual) = function(input).unwrap();

                assert!(rem.is_empty());
                assert_eq!(actual, expected);
            }
        }
    }

    mod variable_definition {
        use super::*;

        #[test]
        #[should_panic]
        fn empty_input() {
            let input = "";
            variable_definition(input).unwrap();
        }

        #[test]
        fn positive_case() {
            let input = "my_var:";

            let expected = Token::VariableDefinition("my_var".to_owned());
            let (rem, actual) = variable_definition(input).unwrap();

            assert!(rem.is_empty());
            assert_eq!(actual, expected);
        }

        #[test]
        #[should_panic]
        fn cannot_start_with_number() {
            let input = "1my_var:";
            variable_definition(input).unwrap();
        }

        #[test]
        #[should_panic]
        fn cannot_start_with_underscore() {
            let input = "_my_var:";
            variable_definition(input).unwrap();
        }

        mod whitespace {
            use super::*;

            #[test]
            fn whitespace_before() {
                let input = " my_var:";

                let expected = Token::VariableDefinition("my_var".to_owned());
                let (rem, actual) = variable_definition(input).unwrap();

                assert!(rem.is_empty());
                assert_eq!(actual, expected);
            }

            #[test]
            fn whitespace_before_colon() {
                let input = "my_var  :";

                let expected = Token::VariableDefinition("my_var".to_owned());
                let (rem, actual) = variable_definition(input).unwrap();

                assert!(rem.is_empty());
                assert_eq!(actual, expected);
            }
        }
    }

    mod variable_reference {
        use super::*;

        #[test]
        #[should_panic]
        fn empty_input() {
            let input = "";
            variable_reference(input).unwrap();
        }

        #[test]
        fn simple_case() {
            let input = "my_var";

            let expected = Token::VariableReference("my_var".to_owned());
            let (rem, actual) = variable_reference(input).unwrap();

            assert!(rem.is_empty());
            assert_eq!(actual, expected);
        }

        #[test]
        #[should_panic]
        fn cannot_start_with_number() {
            let input = "1my_var";
            variable_reference(input).unwrap();
        }

        #[test]
        #[should_panic]
        fn cannot_start_with_underscore() {
            let input = "_my_var";
            variable_reference(input).unwrap();
        }

        mod whitespace {
            use super::*;

            #[test]
            fn whitespace_before() {
                let input = " my_var";

                let expected = Token::VariableReference("my_var".to_owned());
                let (rem, actual) = variable_reference(input).unwrap();

                assert!(rem.is_empty());
                assert_eq!(actual, expected);
            }

            #[test]
            fn whitespace_after() {
                let input = "my_var ";

                let expected = Token::VariableReference("my_var".to_owned());
                let (rem, actual) = variable_reference(input).unwrap();

                assert_eq!(rem, " ");
                assert_eq!(actual, expected);
            }

            #[test]
            fn whitespace_before_and_after() {
                let input = " my_var ";

                let expected = Token::VariableReference("my_var".to_owned());
                let (rem, actual) = variable_reference(input).unwrap();

                assert_eq!(rem, " ");
                assert_eq!(actual, expected);
            }
        }
    }
}
