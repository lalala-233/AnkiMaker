use crate::prelude::*;
pub trait Config: for<'a> Deserialize<'a> + Serialize + Default + ToNotes {
    fn generate(self) -> Result<Vec<String>, CharacterError> {
        let mut result = Vec::new();
        let header = SingleFileHeader::from(&self).generate_header();
        result.extend(header);
        let separator = self.separator();
        let iter = self.try_into_iter()?.map(|texts| texts.join(&separator));
        result.extend(iter);
        Ok(result)
    }
}

pub struct UnionConfig {
    info: UnionInfo,
    content: UnionContent,
}
pub struct UnionInfo {}
pub struct UnionContent {}
