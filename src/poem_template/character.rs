use super::RawCharacter;

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
impl From<RawCharacter> for Character {
    fn from(val: RawCharacter) -> Self {
        match val {
            RawCharacter::Symbol(symbol) => Self::Symbol(symbol),
            RawCharacter::Text(text) => Self::Text(text),
            RawCharacter::RightQuotationMark(_) => {
                panic!("RightQuotationMark can't be into Character")
            }
        }
    }
}
