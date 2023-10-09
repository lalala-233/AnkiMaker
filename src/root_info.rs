use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RootInfo {
    author: Option<String>,
    card_template: String,
    deck: String,
    dynasty: Option<String>,
    separator: Option<String>,
    title: String,
    mode: String,
}
impl Default for RootInfo {
    fn default() -> Self {
        let optional = Some("".to_string());
        let str = "".to_string();
        Self {
            author: optional.clone(),
            card_template: str.clone(),
            deck: str.clone(),
            dynasty: optional,
            separator: None,
            title: str.clone(),
            mode: str,
        }
    }
}
impl From<RootInfo> for crate::poem_template::Info {
    fn from(value: RootInfo) -> Self {
        let card_template = value.card_template;
        let deck = value.deck;
        let title = value.title;
        let author = value.author;
        let dynasty = value.dynasty;
        let separator = value.separator;
        Self::new(card_template, deck, title, author, dynasty, separator)
    }
}
impl From<RootInfo> for crate::default_template::Info {
    fn from(value: RootInfo) -> Self {
        let card_template = value.card_template;
        let deck = value.deck;
        let separator = value.separator;
        Self::new(card_template, deck, separator)
    }
}
