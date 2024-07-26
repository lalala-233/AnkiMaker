use crate::header::{Headers, ToHeader};
#[derive(Debug)]
struct Notes {
    notes: Vec<String>,
    headers: Headers,
}
pub trait ToNotes: ToHeader {
    fn try_into_iter(self) -> Result<impl Iterator<Item = Vec<String>>, String>;
}
