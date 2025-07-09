mod character;
mod content;
mod info;
mod raw_character;
mod text;
use crate::prelude::*;
use character::Character;
use content::Content;
use info::Info;
use raw_character::RawCharacter;
use text::Text;

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct PoemConfig {
    info: Info,
    content: Content,
}
impl ToHeader for PoemConfig {
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
        5
    }
}
impl ToNote for PoemConfig {
    fn try_into_iter(self) -> Result<impl Iterator<Item = Vec<String>>, CharacterError> {
        let Self { info, content } = self;
        let author = info.generate_author_info();
        let title = info.title().clone();
        let iter = content.try_into_iter()?.map(move |mut texts| {
            let index = texts.remove(0);
            let title = format!("{title}{index}");
            texts.insert(0, title);
            texts.insert(1, author.clone());
            texts
        });
        Ok(iter)
    }
}
