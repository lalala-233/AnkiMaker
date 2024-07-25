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
            Character::Symbol(content) | Character::Text(content) => content,
        }
    }
}
impl std::ops::DerefMut for Character {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Character::Symbol(content) | Character::Text(content) => content,
        }
    }
}
impl From<RawCharacter> for Character {
    fn from(val: RawCharacter) -> Self {
        match val {
            RawCharacter::Symbol(symbol) => Character::Symbol(symbol),
            RawCharacter::Text(text) => Character::Text(text),
            RawCharacter::RightQuotationMark(_) => {
                panic!("RightQuotationMark can't be into Character")
            }
        }
    }
}
