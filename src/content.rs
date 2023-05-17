use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Content {
    paragraph: Vec<String>,
}

impl Content {
    pub fn parse(&self) -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod public {
    use super::*;

    pub fn parse() {
        let content = Content {
            paragraph: vec!["你好，我好，大家好。".to_string()],
        };
        content.parse();
    }
}
