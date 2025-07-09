use crate::prelude::*;
pub mod notes;

pub trait ToNote: ToHeader + 'static {
    fn try_into_iter(self) -> Result<impl Iterator<Item = Vec<String>>, CharacterError>;
    fn try_get_note(self) -> Result<Note, Error>
    where
        Self: Sized,
    {
        let header = Header::from(&self);
        let notetype = self.notetype();
        let deck = self.deck();
        let iter = self.try_into_iter()?;
        let result = iter.map(move |mut text| {
            text.insert(0, deck.clone());
            text.insert(1, notetype.clone());
            text
        });
        Ok(Note::new(Box::new(result), header))
    }
}

pub struct Note {
    notes: Box<dyn Iterator<Item = Vec<String>>>,
    header: Header,
}
impl Note {
    pub fn new(notes: Box<dyn Iterator<Item = Vec<String>>>, header: Header) -> Self {
        Self { notes, header }
    }
    pub fn generate(self) -> Vec<String> {
        let mut result = self.header.generate_header();
        let separator = self.header.separator();

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
