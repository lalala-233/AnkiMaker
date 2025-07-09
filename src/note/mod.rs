use crate::prelude::*;
pub mod notes;
pub struct Note{}
pub trait ToNote: ToHeader + 'static {
    fn try_into_iter(self) -> Result<impl Iterator<Item = Vec<String>>, CharacterError>;
    fn try_get_notes(self) -> Result<Notes, CharacterError>
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
        Ok(Notes::new(Box::new(result), headers))
    }
}
