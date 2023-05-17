use serde::Deserialize;

use crate::{Info, Content};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub info: Info,
    pub content: Content,
}
