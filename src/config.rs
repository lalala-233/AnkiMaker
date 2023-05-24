use serde::Deserialize;

use crate::{Content, Info};

#[derive(Deserialize)]
pub struct Config {
    pub info: Info,
    pub content: Content,
}

impl Config {
    const LINE_TYPE: &str = "挖空";
    pub fn generate_with_line(&mut self) -> Result<Vec<String>, &'static str> {
        if self.content.has_error_symbol() {
            return Err("Content 中存在英文符号，会影响笔记生成的效果，请检查！");
        }
        let (info, content) = (&mut self.info, &self.content);
        let mut result = Vec::new();
        //header
        info.set_note_type(Self::LINE_TYPE);
        result.extend(info.generate_header().into_iter());
        let author = &info.generate_author_info();
        let title = info.title();
        let separator = info.separator();
        let paragraphs = content.parse_to_line(separator);

        let mut sum_para = 0;
        let mut sum_line = 0;
        result.extend(paragraphs.into_iter().flat_map(|paragraph| {
            sum_para += 1;
            paragraph.into_iter().map(move |line| {
                sum_line += 1;
                format!(
                    "{}（{}-{}）{separator}{}{separator}{}",
                    title, sum_para, sum_line, author, line
                )
            })
        }));
        Ok(result)
    }
}

#[cfg(test)]
mod public {
    use super::*;
    #[test]
    pub fn generate_with_line() {
        let mut config: Config = toml::from_str(
            "
[info]
author = \"李斯\"
deck = \"New::语文\"
card_template = \"语文::古诗文\"
term = \"高一下\"
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
            "#html:true",
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
        let actual = config.generate_with_line().unwrap();
        assert_eq!(expect, actual);

        let mut config_err: Config = toml::from_str(
            "
[info]
author = \"李斯\"
deck = \"New::语文\"
card_template = \"语文::古诗文\"
term = \"高一下\"
title = \"谏逐客书\"

[content]
paragraph = [
    \"臣闻吏议逐客，窃以为过矣。昔缪公求士，\",
    \"\",
    \"臣闻地广者粟多,国大者人众，\",
    \"夫物不产于秦，\"
]
",
        )
        .unwrap();
        let actual = config_err.generate_with_line();
        assert!(actual.is_err());
    }
}
