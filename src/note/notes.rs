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
impl From<Note> for Notes {
    fn from(value: Note) -> Self {
        Self {
            notes: value.notes,
            headers: value.header.into(),
        }
    }
}
impl Notes {
    pub fn generate(self) -> Vec<String> {
        let mut result = self.headers.generate_header();
        let separator = self.headers.separator();

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
