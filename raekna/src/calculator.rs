use std::collections::HashMap;

use raekna_common::{errors::CommonResult, EditAction, EditPosition, RCalculator};
use raekna_compute::evaluate;
use raekna_parser::parse;
use raekna_storage::storage::Storage;

#[derive(Debug, Default)]
pub struct Calculator {
    storage: Storage,
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
        let mut variables = HashMap::new();
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

    fn get_word_boundaries(&self, origin: EditPosition) -> Option<(EditPosition, EditPosition)> {
        self.storage.get_word_boundaries(origin)
    }
}
