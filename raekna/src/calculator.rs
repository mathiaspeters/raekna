use std::collections::HashMap;

use raekna_common::{
    errors::CommonResult, expression::Literal, EditAction, EditPosition, RCalculator,
};
use raekna_compute::evaluate;
use raekna_parser::parse;
use raekna_storage::storage::Storage;

#[derive(Debug, Default)]
pub struct Calculator {
    storage: Storage,
}

impl Calculator {
    fn init_variables() -> HashMap<String, Literal> {
        let mut variables = HashMap::with_capacity(6);
        variables.insert("e".to_string(), Literal::Float(std::f64::consts::E));
        variables.insert("E".to_string(), Literal::Float(std::f64::consts::E));
        variables.insert("pi".to_string(), Literal::Float(std::f64::consts::PI));
        variables.insert("PI".to_string(), Literal::Float(std::f64::consts::PI));
        variables.insert("tau".to_string(), Literal::Float(std::f64::consts::TAU));
        variables.insert("TAU".to_string(), Literal::Float(std::f64::consts::TAU));
        variables
    }
}

impl RCalculator for Calculator {
    fn get_all_lines(&self) -> (&[String], &[String]) {
        self.storage.get_lines()
    }

    fn get_line(&self, index: usize) -> CommonResult<(&str, &str)> {
        self.storage.get_line(index)
    }

    fn update_line(&mut self, actions: Vec<EditAction>) {
        self.storage.handle_actions(actions);
        let mut variables = Self::init_variables();
        let (contents, results) = self.storage.get_lines_mut();
        results.iter_mut().zip(contents.iter()).for_each(|(r, c)| {
            let ast = parse(c);
            if let Ok(ast) = ast {
                *r = match evaluate(ast, &mut variables) {
                    Ok(res) => res.to_string(),
                    _ => "Error".to_owned(),
                }
            }
        });
    }

    fn get_selection(&self, selection_start: EditPosition, selection_end: EditPosition) -> String {
        self.storage.get_selection(selection_start, selection_end)
    }
}
