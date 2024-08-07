use super::{Content, Info};
use crate::{
    config::Config,
    header::{SingleFileHeader, ToHeader},
    notes::ToNotes,
};
use serde::{Deserialize, Serialize};

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
impl Config for PoemConfig {
    fn generate(self) -> Result<Vec<String>, String> {
        let mut result = Vec::new();
        let header = SingleFileHeader::from(&self).generate_header();
        result.extend(header);
        let separator = self.separator();
        let iter = self.try_into_iter()?.map(|texts| texts.join(&separator));
        result.extend(iter);
        //result.extend();
        Ok(result)
    }
}
impl ToNotes for PoemConfig {
    fn try_into_iter(self) -> Result<impl Iterator<Item = Vec<String>>, String> {
        let Self { info, content } = self;
        let author = info.generate_author_info();
        let title = info.title().clone();
        let iter = content.try_into_iter()?.map(move |mut texts| {
            let index = texts.remove(0);
            let title = format!("{}{}", title, index);
            texts.insert(0, title);
            texts.insert(1, author.to_owned());
            texts
        });
        Ok(iter)
    }
}
#[cfg(test)]
mod public {
    use super::*;
    #[test]
    pub fn generate() {
        let config: PoemConfig = toml::from_str(
            "
[info]
author = \"李斯\"
deck = \"New::语文::挖空\"
notetype = \"语文::古诗文::挖空\"
mode = \"poem\"
title = \"谏逐客书\"

[content]
paragraph = [
    \"臣闻吏议逐客，窃以为过矣。昔缪公求士，\",
    \"\",
    \"臣闻地广者粟多，国大者人众，\",
    \"夫物不产于秦，\"
]
",
        )
        .unwrap();
        let expect: Vec<String> = vec![
            "#separator:|",
            "#html:false",
            "#notetype:语文::古诗文::挖空",
            "#deck:New::语文::挖空::谏逐客书",
            "谏逐客书（1-1）|李斯||臣闻吏议逐客，|窃以为过矣。",
            "谏逐客书（1-2）|李斯|臣闻吏议逐客，|窃以为过矣。|昔缪公求士，",
            "谏逐客书（1-3）|李斯|窃以为过矣。|昔缪公求士，|",
            "谏逐客书（3-1）|李斯||臣闻地广者粟多，|国大者人众，",
            "谏逐客书（3-2）|李斯|臣闻地广者粟多，|国大者人众，|",
            "谏逐客书（4-1）|李斯||夫物不产于秦，|",
        ]
        .into_iter()
        .map(|str| str.to_string())
        .collect();
        let actual = config.generate().unwrap();
        assert_eq!(expect, actual);
        // 存在英文句号、英文逗号
        let config_err: PoemConfig = toml::from_str(
            "
[info]
author = \"李斯\"
deck = \"New::语文::挖空\"
notetype = \"语文::古诗文::挖空\"
mode = \"poem\"
title = \"谏逐客书\"

[content]
paragraph = [
    \"臣闻吏议逐客，窃以为过矣。昔缪公求士，\",
    \"\",
    \"臣闻地广者粟多，国大者人众.\",
    \"夫物不产于秦,\"
]
",
        )
        .unwrap();
        let actual = config_err.generate();
        assert!(actual.is_err())
    }
}
