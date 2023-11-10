use super::{Content, Info};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    info: Info,
    content: Content,
}
impl Config {
    pub fn generate(self) -> Vec<String> {
        let separator = self.info.separator();
        let paragraph = self.content.into_iter();
        let mut result = Vec::new();
        result.extend(self.info.generate_header());
        let lines = paragraph.map(|text| {
            let text = text.into_iter();
            text.map(|content| format!("\"{}\"{}", content, separator))
                .collect()
        });
        result.extend(lines);
        result
    }
}
#[cfg(test)]
mod public {
    use super::Config;

    #[test]
    pub fn generate() {
        let file="
[info]
card_template = \"单面::例句::单词::注释\"
deck = \"New::英语::单词\"
mode = \"emphasis\"

[content]
paragraph = [[\"\"\"Physiology is the study of how living things work.\"\"\", \"\"\"physiology\"\"\", \"\"\"
生理学是研究生物功能的学科。
[uncountable]
the scientific study of the normal functions of living things 生理学
[uncountable, singular]
the way in which a particular living thing functions 生理机能\"\"\"]]
";
        let expect =vec![ "#separator:|".to_string(),
"#html:true".to_string(),
"#notetype:单面::例句::单词::注释".to_string(),
"#deck:New::英语::单词".to_string(),
"\"Physiology is the study of how living things work.\"|\"physiology\"|\"生理学是研究生物功能的学科。
[uncountable]
the scientific study of the normal functions of living things 生理学
[uncountable, singular]
the way in which a particular living thing functions 生理机能\"|".to_string(),
];

        let config: Config = toml::from_str(file).unwrap();
        let actual = config.generate();
        assert_eq!(expect, actual);
    }
}
