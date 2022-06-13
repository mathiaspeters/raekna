use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Lines {
    pub content: Vec<String>,
    pub results: Vec<String>,
}

impl Lines {
    pub fn insert(&mut self, index: usize, c: String) {
        self.content.insert(index, c);
        self.results.insert(index, "".to_owned());
    }

    pub fn new_at(&mut self, index: usize) {
        self.insert(index, "".to_owned());
    }

    pub fn get(&self, index: usize) -> Option<(&str, &str)> {
        if index < self.content.len() {
            Some((&self.content[index], &self.results[index]))
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<(&mut str, &str)> {
        if index < self.content.len() {
            Some((&mut self.content[index], &self.results[index]))
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn remove(&mut self, index: usize) -> (String, String) {
        let c = self.content.remove(index);
        let r = self.results.remove(index);
        if self.content.is_empty() {
            self.content.push("".to_owned());
            self.results.push("".to_owned());
        }
        (c, r)
    }
}

impl Default for Lines {
    fn default() -> Self {
        let content = vec!["".to_owned()];
        let results = vec!["".to_owned()];
        Self { content, results }
    }
}

impl Index<usize> for Lines {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        &self.content[index]
    }
}

impl IndexMut<usize> for Lines {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.content[index]
    }
}
