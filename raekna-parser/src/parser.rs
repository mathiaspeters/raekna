use std::str::FromStr;

use raekna_common::{
    expression::{Expression, Literal as LiteralExpr},
    function_name::FunctionName,
};

use crate::{
    errors::ParserResult,
    lexer::{Literal, Operator, Token, TokenTree},
    ParserError,
};

pub fn parse(raw_expr: &'_ str) -> ParserResult<Expression> {
    if raw_expr.is_empty() {
        return Err(ParserError::EmptyExpression);
    }
    TokenTree::parse_input(raw_expr)
        .map_err(ParserError::NomError)
        .and_then(|(_, tt)| convert_token_tree(tt, true))
}

fn convert_token_tree(token_tree: TokenTree, allow_variable_def: bool) -> ParserResult<Expression> {
    let mut variable = None;
    let mut operators = Vec::with_capacity(token_tree.num_operators);
    let mut expressions = vec![];

    for (i, token) in token_tree.tokens.into_iter().enumerate() {
        let expr = match token {
            Token::Literal(literal) => {
                let sn = match literal {
                    Literal::Integer(i) => LiteralExpr::Integer(i),
                    Literal::Float(f) => LiteralExpr::Float(f),
                };
                Some(Expression::Literal(sn))
            }
            Token::Operator(operator) => {
                operators.push(operator);
                None
            }
            Token::Function(name, args) => {
                let args = args
                    .into_iter()
                    .map(|a| convert_token_tree(a, false))
                    .collect::<ParserResult<Vec<_>>>()?;
                let expr = Expression::Function(
                    FunctionName::from_str(&name)
                        .map_err(|_| ParserError::UnknownFunctionName(name))?,
                    args,
                );
                Some(expr)
            }
            Token::VariableDefinition(name) => {
                if i != 0 || !allow_variable_def {
                    return Err(ParserError::InvalidVariableDefinition(name));
                }
                variable = Some(name);
                None
            }
            Token::VariableReference(name) => Some(Expression::VariableRef(name)),
            Token::Nested(nested_tree) => Some(convert_token_tree(nested_tree, false)?),
        };
        if let Some(expr) = expr {
            expressions.push(Some(expr));
        }
    }
    match (expressions.is_empty(), operators.is_empty()) {
        (true, true) => return Err(ParserError::EmptyExpression),
        (true, false) => {
            return Err(ParserError::InvalidExpression {
                expressions,
                operators,
            })
        }
        _ => {}
    }
    let expr = collapse_expressions(&mut expressions, &mut operators)?;
    if let Some(name) = variable {
        Ok(Expression::Variable(name, Box::new(expr)))
    } else {
        Ok(expr)
    }
}

fn collapse_expressions(
    exprs: &mut [Option<Expression>],
    operators: &mut [Operator],
) -> ParserResult<Expression> {
    if exprs.len() != operators.len() + 1 {
        return Err(ParserError::InvalidExpression {
            expressions: exprs.to_owned(),
            operators: operators.to_owned(),
        });
    }
    if exprs.len() == 1 {
        let mut res = None;
        std::mem::swap(&mut exprs[0], &mut res);
        Ok(res.unwrap())
    } else {
        let mut last_operator = (0, operators[0]);
        for (i, o) in operators.iter().enumerate().skip(1) {
            match (last_operator.1, o) {
                (Operator::Power, _)
                | (
                    Operator::Multiply | Operator::Divide | Operator::Modulo,
                    Operator::Multiply
                    | Operator::Divide
                    | Operator::Modulo
                    | Operator::Add
                    | Operator::Subtract,
                )
                | (Operator::Add | Operator::Subtract, Operator::Add | Operator::Subtract) => {
                    last_operator = (i, *o)
                }
                _ => {}
            }
        }
        let left = if last_operator.0 == 0 {
            let mut left = None;
            std::mem::swap(&mut exprs[0], &mut left);
            left.unwrap()
        } else {
            collapse_expressions(
                &mut exprs[..last_operator.0 + 1],
                &mut operators[..last_operator.0],
            )?
        };
        let right = if last_operator.0 == operators.len().saturating_sub(1) {
            let mut left = None;
            std::mem::swap(&mut exprs[operators.len()], &mut left);
            left.unwrap()
        } else {
            collapse_expressions(
                &mut exprs[last_operator.0 + 1..],
                &mut operators[last_operator.0 + 1..],
            )?
        };
        let function_name = match last_operator.1 {
            Operator::Add => FunctionName::Add,
            Operator::Subtract => FunctionName::Subtract,
            Operator::Multiply => FunctionName::Multiply,
            Operator::Divide => FunctionName::Divide,
            Operator::Modulo => FunctionName::Modulus,
            Operator::Power => FunctionName::Power,
        };
        Ok(Expression::Function(function_name, vec![left, right]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{Literal, Operator, Token};

    mod operator_ordering {
        use super::*;

        #[test]
        fn same_operator_repeated() {
            let tt = TokenTree {
                num_operators: 3,
                tokens: vec![
                    Token::Literal(Literal::Integer(1)),
                    Token::Operator(Operator::Add),
                    Token::Literal(Literal::Integer(2)),
                    Token::Operator(Operator::Add),
                    Token::Literal(Literal::Integer(3)),
                    Token::Operator(Operator::Add),
                    Token::Literal(Literal::Integer(4)),
                ],
            };

            let expected = Expression::Function(
                FunctionName::Add,
                vec![
                    Expression::Function(
                        FunctionName::Add,
                        vec![
                            Expression::Function(
                                FunctionName::Add,
                                vec![
                                    Expression::Literal(LiteralExpr::Integer(1)),
                                    Expression::Literal(LiteralExpr::Integer(2)),
                                ],
                            ),
                            Expression::Literal(LiteralExpr::Integer(3)),
                        ],
                    ),
                    Expression::Literal(LiteralExpr::Integer(4)),
                ],
            );
            let actual = convert_token_tree(tt, true).unwrap();

            assert_eq!(actual, expected);
        }

        #[test]
        fn different_operators_mixed() {
            let tt = TokenTree {
                num_operators: 6,
                tokens: vec![
                    Token::Literal(Literal::Integer(1)),
                    Token::Operator(Operator::Add),
                    Token::Literal(Literal::Integer(2)),
                    Token::Operator(Operator::Multiply),
                    Token::Literal(Literal::Integer(3)),
                    Token::Operator(Operator::Power),
                    Token::Literal(Literal::Integer(4)),
                    Token::Operator(Operator::Divide),
                    Token::Literal(Literal::Integer(5)),
                    Token::Operator(Operator::Subtract),
                    Token::Literal(Literal::Integer(6)),
                    Token::Operator(Operator::Add),
                    Token::Literal(Literal::Integer(7)),
                ],
            };

            let expected = Expression::Function(
                FunctionName::Add,
                vec![
                    Expression::Function(
                        FunctionName::Subtract,
                        vec![
                            Expression::Function(
                                FunctionName::Add,
                                vec![
                                    Expression::Literal(LiteralExpr::Integer(1)),
                                    Expression::Function(
                                        FunctionName::Divide,
                                        vec![
                                            Expression::Function(
                                                FunctionName::Multiply,
                                                vec![
                                                    Expression::Literal(LiteralExpr::Integer(2)),
                                                    Expression::Function(
                                                        FunctionName::Power,
                                                        vec![
                                                            Expression::Literal(
                                                                LiteralExpr::Integer(3),
                                                            ),
                                                            Expression::Literal(
                                                                LiteralExpr::Integer(4),
                                                            ),
                                                        ],
                                                    ),
                                                ],
                                            ),
                                            Expression::Literal(LiteralExpr::Integer(5)),
                                        ],
                                    ),
                                ],
                            ),
                            Expression::Literal(LiteralExpr::Integer(6)),
                        ],
                    ),
                    Expression::Literal(LiteralExpr::Integer(7)),
                ],
            );
            let actual = convert_token_tree(tt, true).unwrap();

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn mix_of_expressions() {
        let tt = TokenTree {
            num_operators: 1,
            tokens: vec![
                Token::VariableDefinition("var_def".to_owned()),
                Token::Function(
                    "sqrt".to_owned(),
                    vec![TokenTree {
                        num_operators: 0,
                        tokens: vec![Token::Literal(Literal::Integer(1))],
                    }],
                ),
                Token::Operator(Operator::Multiply),
                Token::Nested(TokenTree {
                    num_operators: 1,
                    tokens: vec![
                        Token::VariableReference("my_var".to_owned()),
                        Token::Operator(Operator::Add),
                        Token::VariableReference("my_second_var".to_owned()),
                    ],
                }),
            ],
        };

        let expected = Expression::Variable(
            "var_def".to_owned(),
            Box::new(Expression::Function(
                FunctionName::Multiply,
                vec![
                    Expression::Function(
                        FunctionName::SquareRoot,
                        vec![Expression::Literal(LiteralExpr::Integer(1))],
                    ),
                    Expression::Function(
                        FunctionName::Add,
                        vec![
                            Expression::VariableRef("my_var".to_owned()),
                            Expression::VariableRef("my_second_var".to_owned()),
                        ],
                    ),
                ],
            )),
        );
        let actual = convert_token_tree(tt, true).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic]
    fn invalid_function_name() {
        let tt = TokenTree {
            num_operators: 0,
            tokens: vec![Token::Function("invalid".to_owned(), vec![])],
        };
        convert_token_tree(tt, true).unwrap();
    }

    #[test]
    #[should_panic]
    fn only_variable_def() {
        let tt = TokenTree {
            num_operators: 0,
            tokens: vec![Token::VariableDefinition("my_var".to_owned())],
        };
        convert_token_tree(tt, true).unwrap();
    }

    #[test]
    #[should_panic]
    fn variable_def_is_not_first_token() {
        let tt = TokenTree {
            num_operators: 0,
            tokens: vec![
                Token::Literal(Literal::Integer(1)),
                Token::VariableDefinition("invalid".to_owned()),
            ],
        };
        convert_token_tree(tt, true).unwrap();
    }

    #[test]
    #[should_panic]
    fn variable_def_in_nested_token_tree() {
        let tt = TokenTree {
            num_operators: 0,
            tokens: vec![
                Token::Nested(TokenTree {
                    num_operators: 0,
                    tokens: vec![
                        Token::VariableDefinition("invalid".to_owned()),
                        Token::Literal(Literal::Integer(1)),
                    ],
                }),
                Token::Literal(Literal::Integer(1)),
            ],
        };
        convert_token_tree(tt, true).unwrap();
    }
}
