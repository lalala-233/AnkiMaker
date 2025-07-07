use super::RawCharacter;
use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Character {
    Symbol(String),
    Text(String),
}
impl std::ops::Deref for Character {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Symbol(content) | Self::Text(content) => content,
        }
    }
}
impl std::ops::DerefMut for Character {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Symbol(content) | Self::Text(content) => content,
        }
    }
}
impl TryFrom<RawCharacter> for Character {
    type Error = CharacterError;

    fn try_from(value: RawCharacter) -> Result<Self, Self::Error> {
        match value {
            RawCharacter::Symbol(symbol) => Ok(Self::Symbol(symbol)),
            RawCharacter::Text(text) => Ok(Self::Text(text)),
            RawCharacter::RightQuotationMark(_) => Err(CharacterError::RightQuotationMark),
        }
    }
}
