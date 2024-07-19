use super::{Content, Info};
use crate::config::Config;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct DefaultConfig {
    info: Info,
    content: Content,
}

impl Config for DefaultConfig {
    fn generate(self) -> Result<Vec<std::string::String>, std::string::String> {
        let separator = self.info.separator();
        let paragraph = self.content.into_iter();
        let mut result = Vec::new();
        result.extend(self.info.generate_header());
        let lines = paragraph.map(|text| {
            let text = text.into_iter();
            text.fold(String::new(), |mut output, content| {
                use std::fmt::Write;
                let _ = write!(output, "\"{}\"{}", content, separator);
                output
            })
        });
        result.extend(lines);
        Ok(result)
    }
}
#[cfg(test)]
mod public {
    use super::DefaultConfig;
    use crate::config::Config;

    #[test]
    pub fn generate() {
        let file="
[info]
notetype = \"单面::例句::单词::注释\"
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
        let expect =vec![ "#separator:|",
"#html:false",
"#notetype:单面::例句::单词::注释",
"#deck:New::英语::单词",
"\"Physiology is the study of how living things work.\"|\"physiology\"|\"生理学是研究生物功能的学科。
[uncountable]
the scientific study of the normal functions of living things 生理学
[uncountable, singular]
the way in which a particular living thing functions 生理机能\"|",
];

        let config: DefaultConfig = toml::from_str(file).unwrap();
        let actual = config.generate().unwrap();
        assert_eq!(expect, actual);
    }
}
