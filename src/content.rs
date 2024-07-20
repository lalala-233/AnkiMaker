pub trait ToNotes {
    fn into_iter(self) -> impl Iterator<Item = Vec<String>>;
}
