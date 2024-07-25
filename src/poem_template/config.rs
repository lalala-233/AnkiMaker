use super::{Content, Info};
use crate::config::Config;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct PoemConfig {
    info: Info,
    content: Content,
}
impl Config for PoemConfig {
    fn generate(self) -> Result<Vec<String>, String> {
        let Self { info, content } = self;
        let mut result = Vec::new();
        //header
        result.extend(info.generate_header());
        let author = &info.generate_author_info();
        let title = info.title();
        let separator = &info.separator();
        let paragraph = content.try_parse()?.into_iter();
        let lines = paragraph.map(|mut text| {
            let index = text.remove(0);
            let title = format!("{}{}", title, index);
            text.insert(0, title);
            text.insert(1, author.to_owned());
            text.join(&separator)
        });
        result.extend(lines);
        //result.extend();
        Ok(result)
    }
}
#[cfg(test)]
mod public {
    use super::*;
    #[test]
    pub fn generate_with_line() {
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
