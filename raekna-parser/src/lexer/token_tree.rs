use nom::IResult;

use super::{
    parsers::{function, nested, operator, parse_number, variable_definition, variable_reference},
    token::Token,
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TokenTree {
    pub num_operators: usize,
    pub tokens: Vec<Token>,
}

impl TokenTree {
    pub fn parse_input(input: &str) -> IResult<(), Self> {
        let mut token_tree = Self::default();

        let mut input = input;
        let parsers = [
            variable_definition,
            parse_number,
            function,
            variable_reference,
            operator,
            nested,
        ];
        'outer: while !input.is_empty() {
            for parser in parsers.iter() {
                if let Ok((rem, token)) = parser(input) {
                    input = rem;
                    if let Token::Operator(_) = token {
                        token_tree.num_operators += 1;
                    }
                    token_tree.tokens.push(token);
                    continue 'outer;
                }
            }
            return Err(nom::Err::Incomplete(nom::Needed::Unknown));
        }

        Ok(((), token_tree))
    }
}

#[cfg(test)]
mod tests {
    use raekna_common::expression::Literal;

    use super::*;
    use crate::lexer::token::Operator;

    #[test]
    fn empty_input() {
        let input = "";

        let expected = TokenTree {
            num_operators: 0,
            tokens: vec![],
        };
        let (_, actual) = TokenTree::parse_input(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn function_with_negative_arguments() {
        let input = "add(-1, -2)";

        let expected = TokenTree {
            num_operators: 0,
            tokens: vec![Token::Function(
                "add".to_owned(),
                vec![
                    TokenTree {
                        num_operators: 1,
                        tokens: vec![
                            Token::Operator(Operator::Subtract),
                            Token::Literal(Literal::Integer(1)),
                        ],
                    },
                    TokenTree {
                        num_operators: 1,
                        tokens: vec![
                            Token::Operator(Operator::Subtract),
                            Token::Literal(Literal::Integer(2)),
                        ],
                    },
                ],
            )],
        };
        let (_, actual) = TokenTree::parse_input(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn scenario1() {
        let input = "var_def: pow(sqrt(var_ref), 5 / 2.0) * (1e2 + 2.2)";

        let expected = TokenTree {
            num_operators: 1,
            tokens: vec![
                Token::VariableDefinition("var_def".to_owned()),
                Token::Function(
                    "pow".to_owned(),
                    vec![
                        TokenTree {
                            num_operators: 0,
                            tokens: vec![Token::Function(
                                "sqrt".to_owned(),
                                vec![TokenTree {
                                    num_operators: 0,
                                    tokens: vec![Token::VariableReference("var_ref".to_owned())],
                                }],
                            )],
                        },
                        TokenTree {
                            num_operators: 1,
                            tokens: vec![
                                Token::Literal(Literal::Integer(5)),
                                Token::Operator(Operator::Divide),
                                Token::Literal(Literal::Integer(2)),
                            ],
                        },
                    ],
                ),
                Token::Operator(Operator::Multiply),
                Token::Nested(TokenTree {
                    num_operators: 1,
                    tokens: vec![
                        Token::Literal(Literal::Integer(100)),
                        Token::Operator(Operator::Add),
                        Token::Literal(Literal::Float(2.2)),
                    ],
                }),
            ],
        };
        let (_, actual) = TokenTree::parse_input(input).unwrap();

        assert_eq!(actual, expected);
    }
}
