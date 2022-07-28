use raekna_common::expression::Literal;

pub fn evaluate(name: &str) -> Option<Literal> {
    match name.to_lowercase().as_str() {
        "pi" => Some(Literal::Float(std::f64::consts::PI)),
        "tau" => Some(Literal::Float(std::f64::consts::TAU)),
        "e" => Some(Literal::Float(std::f64::consts::E)),
        _ => None,
    }
}
