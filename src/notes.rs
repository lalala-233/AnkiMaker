use crate::header::{Headers, ToHeader};
pub struct Notes {
    notes: Box<dyn Iterator<Item = Vec<String>>>,
    headers: Headers,
}
impl std::ops::Add for Notes {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let headers = self.headers + rhs.headers;
        let notes = Box::new(self.notes.chain(rhs.notes));
        Self { notes, headers }
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
            text.insert(0, notetype.clone());
            text.insert(1, deck.clone());
            text
        });
        Ok(Notes {
            notes: Box::new(result),
            headers,
        })
    }
}
