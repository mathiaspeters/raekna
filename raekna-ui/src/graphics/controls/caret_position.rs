#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct CaretPosition {
    pub line: usize,
    pub column: usize,
    pub actual_column: usize,
}

impl CaretPosition {
    pub fn new(line_widths: &[usize]) -> Self {
        let line = line_widths.len().saturating_sub(1);
        let column = if line_widths.is_empty() {
            0
        } else {
            line_widths[line]
        };
        let actual_column = column;
        Self {
            line,
            column,
            actual_column,
        }
    }

    pub fn set_position(&mut self, line: usize, column: usize) {
        self.line = line;
        self.column = column;
        self.actual_column = column;
    }

    pub fn move_up(&mut self, line_widths: &[usize]) -> bool {
        if self.line == 0 && self.column == 0 {
            false
        } else {
            if self.line == 0 {
                self.column = 0;
            } else {
                self.line = self.line.saturating_sub(1);
                let line_width = line_widths[self.line];
                if line_width < self.actual_column {
                    self.column = line_width;
                } else if self.column != self.actual_column {
                    self.column = self.actual_column;
                }
            }
            true
        }
    }
    pub fn move_down(&mut self, line_widths: &[usize]) -> bool {
        if self.line == line_widths.len().saturating_sub(1) && self.column == line_widths[self.line]
        {
            false
        } else {
            if self.line == line_widths.len().saturating_sub(1) {
                self.column = line_widths[self.line];
            } else {
                self.line += 1;
                let line_width = line_widths[self.line];
                if line_width < self.actual_column {
                    self.column = line_width;
                } else if self.column != self.actual_column {
                    self.column = self.actual_column;
                }
            }
            true
        }
    }

    pub fn move_left(&mut self, line_widths: &[usize]) -> bool {
        if self.line == 0 && self.column == 0 {
            false
        } else {
            if self.column == 0 {
                self.line -= 1;
                self.column = line_widths[self.line];
            } else {
                self.column -= 1;
            }
            self.actual_column = self.column;
            true
        }
    }
    pub fn move_right(&mut self, line_widths: &[usize]) -> bool {
        if self.line == line_widths.len() - 1 && self.column == line_widths[self.line] {
            false
        } else {
            if self.column == line_widths[self.line] {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
            self.actual_column = self.column;
            true
        }
    }

    pub fn home(&mut self) -> bool {
        if self.column == 0 {
            self.actual_column = self.column;
            false
        } else {
            self.column = 0;
            self.actual_column = self.column;
            true
        }
    }

    pub fn end(&mut self, line_widths: &[usize]) -> bool {
        if self.column == line_widths[self.line] {
            self.actual_column = self.column;
            false
        } else {
            self.column = line_widths[self.line];
            self.actual_column = self.column;
            true
        }
    }

    pub fn page_up(&mut self) -> bool {
        if self.line == 0 && self.column == 0 {
            false
        } else {
            self.line = 0;
            self.column = 0;
            self.actual_column = self.column;
            true
        }
    }

    pub fn page_down(&mut self, line_widths: &[usize]) -> bool {
        if self.line == line_widths.len() - 1 && self.column == line_widths[self.line] {
            false
        } else {
            self.line = line_widths.len() - 1;
            self.column = line_widths[self.line];
            self.actual_column = self.column;
            true
        }
    }
}
