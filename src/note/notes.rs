use crate::prelude::*;
pub struct Notes {
    notes: Box<dyn Iterator<Item = Vec<String>>>,
    headers: Headers,
}
impl std::ops::Add for Notes {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let notes = if self.headers.len() >= rhs.headers.len() {
            Box::new(self.notes.chain(rhs.notes))
        } else {
            Box::new(rhs.notes.chain(self.notes))
        };
        let headers = self.headers + rhs.headers;

        Self { notes, headers }
    }
}
impl Notes {
    pub fn new(notes: Box<dyn Iterator<Item = Vec<String>>>, headers: Headers) -> Self {
        Self { notes, headers }
    }
    pub fn generate(self) -> Vec<String> {
        let headers = self.headers;
        let mut result = headers.generate_header();
        let separator = headers.separator();
        let lines = self.notes.map(|texts| {
            let vec = texts
                .into_iter()
                .map(|text| format!("\"{text}\""))
                .collect::<Vec<_>>();
            vec.join(&separator)
        });
        result.extend(lines);
        result
    }
}
