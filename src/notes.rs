use crate::header::{Headers, ToHeader};
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
    /// May panic!()
    pub fn generate(self) -> Vec<String> {
        let headers = self.headers;
        let mut result = headers.generate_header();
        let separator = headers.separator();
        let lines = self.notes.map(|texts| {
            let vec = texts
                .into_iter()
                .map(|text| format!("\"{}\"", text))
                .collect::<Vec<_>>();
            vec.join(&separator)
        });
        result.extend(lines);
        result
    }
}
pub trait ToNotes: ToHeader + 'static {
    fn try_into_iter(self) -> Result<impl Iterator<Item = Vec<String>>, String>;
    fn try_get_notes(self) -> Result<Notes, String>
    where
        Self: Sized,
    {
        let headers = Headers::from(&self);
        let notetype = self.notetype();
        let deck = self.deck();
        let iter = self.try_into_iter()?;
        let result = iter.map(move |mut text| {
            text.insert(0, deck.clone());
            text.insert(1, notetype.clone());
            text
        });
        Ok(Notes {
            notes: Box::new(result),
            headers,
        })
    }
}
