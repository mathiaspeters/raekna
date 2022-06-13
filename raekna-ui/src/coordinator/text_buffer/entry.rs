use crate::coordinator::dimensions::Dimensions;

#[derive(Debug)]
pub struct Entry {
    pub content: Vec<Content>,
    pub results: Vec<Result>,
}

impl Entry {
    pub fn new(content_length: usize, result_length: usize, dimensions: &Dimensions) -> Self {
        let content_width = dimensions.content_columns();
        let result_width = dimensions.result_columns();
        let content = Self::build_content_lines(content_length, content_width);
        let results = Self::build_result_lines(result_length, result_width, content.len());
        Self { content, results }
    }

    pub fn line_widths(&self) -> Vec<usize> {
        self.content.iter().map(|c| c.len()).collect()
    }

    pub fn line_count(&self) -> usize {
        self.content.len()
    }

    fn build_content_lines(content_length: usize, line_width: usize) -> Vec<Content> {
        let mut content = Vec::with_capacity(content_length % line_width);
        let mut next_start = 0_usize;
        let mut next_end = line_width;
        while next_end < content_length {
            content.push(Content::new(vec![Segment::new(next_start, line_width)]));
            next_start += line_width;
            next_end += line_width;
        }
        content.push(Content::new(vec![Segment::new(next_start, content_length)]));
        content
    }

    fn build_result_lines(
        result_length: usize,
        line_width: usize,
        content_lines: usize,
    ) -> Vec<Result> {
        let mut results = vec![Result::None; content_lines];
        if content_lines > 0 {
            results[content_lines.saturating_sub(1)] = if result_length > line_width {
                Result::Elipsis
            } else {
                Result::Full
            };
        }
        results
    }
}

#[derive(Debug)]
pub struct Content {
    pub segments: Vec<Segment>,
}

impl Content {
    pub fn new(segments: Vec<Segment>) -> Self {
        Self { segments }
    }

    pub fn len(&self) -> usize {
        self.segments.iter().map(|s| s.end - s.start).sum::<usize>()
    }
}

#[derive(Debug)]
pub struct Segment {
    pub start: usize,
    pub end: usize,
}

impl Segment {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Result {
    None,
    Elipsis,
    Full,
}
