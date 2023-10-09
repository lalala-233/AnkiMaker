use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Info {
    author: Option<String>,
    card_template: String,
    deck: String,
    dynasty: Option<String>,
    separator: Option<String>,
    title: String,
    mode: String,
}
impl Default for Info {
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
