use std::collections::HashMap;

use raekna_common::expression::{Expression, Literal};

use crate::{
    errors::{ComputeError, ComputeResult},
    ops::{constants, evaluate_fn},
};

pub fn evaluate(
    expression: Expression,
    variables: &mut HashMap<String, Literal>,
) -> ComputeResult<Literal> {
    match expression {
        Expression::Variable(name, expr) => match constants::evaluate(&name) {
            Some(_) => Err(ComputeError::VariableNameTaken(name)),
            None => {
                let res = evaluate_to_literal(&expr, variables)?;
                variables.insert(name, res);
                Ok(res)
            }
        },
        expr => {
            let res = evaluate_to_literal(&expr, variables)?;
            Ok(res)
        }
    }
}

fn evaluate_to_literal(
    expression: &Expression,
    variables: &HashMap<String, Literal>,
) -> ComputeResult<Literal> {
    match expression {
        Expression::Literal(literal) => Ok(*literal),
        Expression::Variable(_, _) => unreachable!(),
        Expression::VariableRef(var_name) => {
            let value = constants::evaluate(var_name).map(Ok).unwrap_or_else(|| {
                variables
                    .get(var_name.as_str())
                    .copied()
                    .ok_or_else(|| ComputeError::UnknownVariable(var_name.clone()))
            })?;
            Ok(value)
        }
        Expression::Function(fn_name, args) => {
            let args = args
                .iter()
                .map(|a| evaluate_to_literal(a, variables))
                .collect::<ComputeResult<Vec<_>>>()?;
            evaluate_fn(*fn_name, args)
        }
    }
}

#[cfg(test)]
mod tests {
    use raekna_common::function_name::FunctionName;

    use super::*;

    #[test]
    fn test_correct_variable_is_read() {
        let mut variables = HashMap::new();
        variables.insert("var1".to_owned(), Literal::Integer(5));
        variables.insert("var2".to_owned(), Literal::Integer(10));

        let expression = Expression::VariableRef("var2".to_owned());

        let expected = Literal::Integer(10);
        let actual = evaluate(expression, &mut variables).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic]
    fn test_trying_to_read_undefined_variable() {
        let mut variables = HashMap::new();
        variables.insert("var1".to_owned(), Literal::Integer(5));

        let expression = Expression::VariableRef("var2".to_owned());

        evaluate(expression, &mut variables).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_nested_variable_definition() {
        let mut variables = HashMap::new();

        let expression = Expression::Variable(
            "var1".to_owned(),
            Box::new(Expression::Variable(
                "var2".to_owned(),
                Box::new(Expression::Literal(Literal::Integer(5))),
            )),
        );

        evaluate(expression, &mut variables).unwrap();
    }

    #[test]
    fn test_function_arguments_are_evaluated_before_function_itself() {
        let mut variables = HashMap::new();

        let expression = Expression::Function(
            FunctionName::Power,
            vec![
                Expression::Function(
                    FunctionName::Add,
                    vec![
                        Expression::Literal(Literal::Integer(1)),
                        Expression::Literal(Literal::Integer(2)),
                    ],
                ),
                Expression::Function(
                    FunctionName::Subtract,
                    vec![
                        Expression::Literal(Literal::Integer(4)),
                        Expression::Literal(Literal::Integer(3)),
                    ],
                ),
            ],
        );

        let expected = Literal::Integer(3);
        let actual = evaluate(expression, &mut variables).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_literal_is_evaluated_to_self() {
        let mut variables = HashMap::new();

        let literal = Literal::Float(12.345);
        let expression = Expression::Literal(literal);

        let expected = literal;
        let actual = evaluate(expression, &mut variables).unwrap();

        assert_eq!(actual, expected);
    }
}
