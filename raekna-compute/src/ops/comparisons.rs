use raekna_common::expression::Literal;

pub fn min(left: Literal, right: Literal) -> Option<Literal> {
    use Literal::*;
    let right_is_bigger = match (left, right) {
        (Integer(left), Integer(right)) => left <= right,
        (Integer(i), Float(f)) => (i as f64) <= f,
        (Float(f), Integer(i)) => f <= (i as f64),
        (Float(left), Float(right)) => left <= right,
    };
    let result = if right_is_bigger {
        left.maybe_truncate()
    } else {
        right.maybe_truncate()
    };
    Some(result)
}

pub fn max(left: Literal, right: Literal) -> Option<Literal> {
    use Literal::*;
    let right_is_smaller = match (left, right) {
        (Integer(left), Integer(right)) => left >= right,
        (Integer(i), Float(f)) => (i as f64) >= f,
        (Float(f), Integer(i)) => f >= (i as f64),
        (Float(left), Float(right)) => left >= right,
    };
    let result = if right_is_smaller {
        left.maybe_truncate()
    } else {
        right.maybe_truncate()
    };
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::test_utils::{float, int};

    #[test]
    fn minimum() {
        let test_cases = [
            // Integers on both sides
            (int(5), int(8), int(5)),
            (int(5), int(-8), int(-8)),
            (int(-5), int(8), int(-5)),
            (int(-5), int(-8), int(-8)),
            // Mixed integers and floats
            (int(5), float(8.0), int(5)),
            (int(5), float(-8.0), int(-8)),
            (int(-5), float(8.0), int(-5)),
            (int(-5), float(-8.0), int(-8)),
            (float(5.1), int(8), float(5.1)),
            (float(5.0), int(-8), int(-8)),
            (float(-5.0), int(8), int(-5)),
            (float(-5.0), int(-8), int(-8)),
            // Floats on both sides
            (float(5.0), float(8.0), int(5)),
            (float(5.0), float(-8.2), float(-8.2)),
            (float(-5.0), float(8.0), int(-5)),
            (float(-5.5), float(-8.5), float(-8.5)),
        ];
        for (left, right, expected) in test_cases.into_iter() {
            let actual = min(left, right).unwrap();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn maximum() {
        let test_cases = [
            // Integers on both sides
            (int(5), int(8), int(8)),
            (int(5), int(-8), int(5)),
            (int(-5), int(8), int(8)),
            (int(-5), int(-8), int(-5)),
            // Mixed integers and floats
            (int(5), float(8.0), int(8)),
            (int(5), float(-8.0), int(5)),
            (int(-5), float(8.1), float(8.1)),
            (int(-5), float(-8.0), int(-5)),
            (float(5.1), int(8), int(8)),
            (float(5.0), int(-8), int(5)),
            (float(-5.0), int(8), int(8)),
            (float(-5.0), int(-8), int(-5)),
            // Floats on both sides
            (float(5.0), float(8.0), int(8)),
            (float(5.1), float(-8.2), float(5.1)),
            (float(-5.0), float(8.0), int(8)),
            (float(-5.5), float(-8.5), float(-5.5)),
        ];
        for (left, right, expected) in test_cases.into_iter() {
            let actual = max(left, right).unwrap();
            assert_eq!(actual, expected);
        }
    }
}
