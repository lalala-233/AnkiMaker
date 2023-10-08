use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Info {
    card_template: String,
    deck: String,
    separator: Option<char>,
}
impl Default for Info {
    fn default() -> Self {
        let optional = Some("".to_string());
        let str = "".to_string();
        Self {
            card_template: str.clone(),
            deck: str.clone(),
            separator: None,
        }
    }
}