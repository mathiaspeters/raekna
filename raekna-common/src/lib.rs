use errors::CommonResult;

pub mod errors;
pub mod expression;
pub mod function_name;

pub trait RCalculator {
    fn get_all_lines(&self) -> (&[String], &[String]);
    fn get_line(&self, index: usize) -> CommonResult<(&str, &str)>;
    fn update_line(&mut self, actions: Vec<EditAction>);
    fn get_selection(&self, selection_start: EditPosition, selection_end: EditPosition) -> String;
}

#[derive(Copy, Clone, Debug)]
pub struct EditPosition {
    pub line: usize,
    pub column: usize,
}

impl EditPosition {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

#[derive(Clone, Debug)]
pub enum EditAction {
    NewLine(EditPosition),
    Insert(EditPosition, char),
    InsertMultiple(EditPosition, String),
    Delete {
        selection_start: EditPosition,
        selection_end: Option<EditPosition>,
    },
    DeleteForward(EditPosition),
}
