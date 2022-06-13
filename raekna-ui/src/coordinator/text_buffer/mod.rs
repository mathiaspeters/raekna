pub mod entry;

use entry::Entry;

use crate::coordinator::dimensions::Dimensions;

#[derive(Debug)]
pub struct TextBuffer {
    pub entries: Vec<Entry>,
    line_widths: Vec<usize>,
    line_counts: Vec<usize>,
    pub current_scroll: f32,
}

impl TextBuffer {
    pub fn new(lines: (&[String], &[String]), dimensions: &Dimensions) -> Self {
        let current_scroll = 0.0;

        let mut buffer = Self {
            entries: vec![],
            line_widths: vec![],
            line_counts: vec![],
            current_scroll,
        };

        buffer.update(lines, dimensions);

        buffer
    }

    pub fn update_scroll(&mut self, scroll: f32) {
        self.current_scroll = scroll;
    }

    pub fn line_widths(&self) -> &[usize] {
        &self.line_widths
    }

    pub fn line_counts(&self) -> &[usize] {
        &self.line_counts
    }

    pub fn update(&mut self, lines: (&[String], &[String]), dimensions: &Dimensions) {
        self.entries.clear();
        lines.0.iter().zip(lines.1.iter()).for_each(|(c, r)| {
            let entry = Entry::new(c.len(), r.len(), dimensions);
            self.entries.push(entry);
        });
        self.update_line_widths();
        self.update_line_counts();
    }

    fn update_line_widths(&mut self) {
        self.line_widths = self
            .entries
            .iter()
            .flat_map(|entry| entry.line_widths())
            .collect();
    }

    fn update_line_counts(&mut self) {
        self.line_counts = self
            .entries
            .iter()
            .map(|entry| entry.line_count())
            .collect();
    }
}
