use super::{Content, Info};
use crate::{
    config::Config,
    header::{SingleFileHeader, ToHeader},
    notes::ToNotes,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct DefaultConfig {
    info: Info,
    content: Content,
}
impl ToNotes for DefaultConfig {
    fn try_into_iter(self) -> Result<impl Iterator<Item = Vec<String>>, String> {
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
}
impl Config for DefaultConfig {
    fn generate(self) -> Result<Vec<String>, String> {
        let mut result = Vec::new();
        let header = SingleFileHeader::from(&self).generate_header();
        result.extend(header);
        let separator = self.info.separator();
        let paragraph = self.content.into_iter();
        let lines = paragraph.map(|texts| {
            let vec = texts
                .into_iter()
                .map(|text| format!("\"{}\"", text))
                .collect::<Vec<_>>();
            vec.join(&separator)
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
        let file = "
[info]
notetype = \"单面::例句::单词::注释\"
deck = \"New::英语::单词\"
mode = \"emphasis\"

[content]
paragraph = [
[\"\"\"Physiology is the study of how living things work.\"\"\",
\"\"\"physiology\"\"\",
\"\"\"生理学是研究生物功能的学科。
[uncountable]
the scientific study of the normal functions of living things 生理学
[uncountable, singular]
the way in which a particular living thing functions 生理机能\"\"\"]]";
        let expect =vec![ "#separator:|",
"#html:false",
"#notetype:单面::例句::单词::注释",
"#deck:New::英语::单词",
"\"Physiology is the study of how living things work.\"|\"physiology\"|\"生理学是研究生物功能的学科。
[uncountable]
the scientific study of the normal functions of living things 生理学
[uncountable, singular]
the way in which a particular living thing functions 生理机能\"",
];

        let config: DefaultConfig = toml::from_str(file).unwrap();
        let actual = config.generate().unwrap();
        assert_eq!(expect, actual);
        let file = "[info]
notetype = \"双面::正面::反面::注释\"
deck = \"New::语文::成语\"
mode = \"default\"

[content]
paragraph = [
    [
        \"\"\"哀而不伤\"\"\",
        \"\"\"忧愁而不悲伤。
形容感情或行为有节制，不太过分。
也形容诗歌、音乐等表现的伤感情调适度。\"\"\",
        \"\"\"哀：悲哀。
伤：妨害。\"\"\",
    ],
    [
        \"\"\"哀鸿遍野\"\"\",
        \"\"\"比喻呻吟呼号、流离失所的灾民到处都是。\"\"\",
        \"\"\"哀鸿：哀鸣的大雁，比喻悲哀呼号的灾民。\"\"\",
    ],
    [
        \"\"\"安土重迁\"\"\",
        \"\"\"在家乡住惯了，安于本乡本土，不愿轻易迁移。
形容留恋故土。\"\"\",
        \"\"\"重：看得很重。\"\"\",
    ],
]";
        let expect = vec!["#separator:|",
"#html:false",
"#notetype:双面::正面::反面::注释",
"#deck:New::语文::成语",
"\"哀而不伤\"|\"忧愁而不悲伤。
形容感情或行为有节制，不太过分。
也形容诗歌、音乐等表现的伤感情调适度。\"|\"哀：悲哀。
伤：妨害。\"",
"\"哀鸿遍野\"|\"比喻呻吟呼号、流离失所的灾民到处都是。\"|\"哀鸿：哀鸣的大雁，比喻悲哀呼号的灾民。\"",
"\"安土重迁\"|\"在家乡住惯了，安于本乡本土，不愿轻易迁移。
形容留恋故土。\"|\"重：看得很重。\"",];
        let config: DefaultConfig = toml::from_str(file).unwrap();
        let actual = config.generate().unwrap();
        assert_eq!(expect, actual);
    }
}
