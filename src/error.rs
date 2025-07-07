use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Character(#[from] CharacterError),
}

#[derive(Debug, Error)]

pub enum CharacterError {
    #[error("RightQuotationMark can't be into Character")]
    RightQuotationMark,
}
