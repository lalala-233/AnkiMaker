pub trait ToNotes {
    fn try_into_iter(self) -> Result<impl Iterator<Item = Vec<String>>, String>;
}
