use crate::prelude::*;
pub trait Config: for<'a> Deserialize<'a> + Serialize + Default {
    fn generate(self) -> Result<Vec<String>, CharacterError>;
}
