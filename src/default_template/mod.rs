mod content;
mod info;
use crate::prelude::*;
use content::Content;
use info::Info;

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct DefaultConfig {
    info: Info,
    content: Content,
}
impl ToNote for DefaultConfig {
    fn try_into_iter(self) -> Result<impl Iterator<Item = Vec<String>>, CharacterError> {
        Ok(self.content.into_iter())
    }
}
impl ToHeader for DefaultConfig {
    fn notetype(&self) -> String {
        self.info.notetype()
    }
    fn deck(&self) -> String {
        self.info.deck()
    }
    fn separator(&self) -> String {
        self.info.separator()
    }

    fn len(&self) -> usize {
        self.content.len()
    }
}
// impl Config for DefaultConfig {
//     fn config_generate(self) -> Result<Vec<String>, CharacterError> {
//         let mut result = Vec::new();
//         let header = Header::from(&self).generate_header();
//         result.extend(header);
//         let separator = self.info.separator();
//         let lines = self.content.into_iter().map(|texts| {
//             let vec = texts
//                 .into_iter()
//                 .map(|text| format!("\"{text}\""))
//                 .collect::<Vec<_>>();
//             vec.join(&separator)
//         });
//         result.extend(lines);
//         Ok(result)
//     }
// }
