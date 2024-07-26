use super::ToHeader;

#[derive(Debug)]
pub struct Headers {
    html: bool,
    separator: String,
    notetype: String,
    deck: String,
}
impl Headers {
    pub fn new(to_header: impl ToHeader) -> Self {
        todo!()
    }
}
