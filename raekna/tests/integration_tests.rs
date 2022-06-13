use std::collections::HashMap;

use raekna_common::expression::Literal;
use raekna_compute::evaluate;
use raekna_parser::parse;

#[test]
fn test_simple_literal() {
    let mut variables = HashMap::new();

    let input = "123";

    let expected = Literal::Integer(123);
    let actual = {
        let parsed = parse(input).unwrap();
        evaluate(parsed, &mut variables).unwrap()
    };

    assert_eq!(actual, expected);
}
