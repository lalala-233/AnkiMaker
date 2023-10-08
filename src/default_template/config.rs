use super::{Content, Info};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    info: Info,
    content: Content,
}