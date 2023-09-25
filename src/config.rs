use crate::{Content, Info};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    info: Info,
    content: Content,
}

impl Config {
    pub fn generate_with_line(&mut self) -> Result<Vec<String>, String> {
        let (info, content) = (&mut self.info, &self.content);
        let mut result = Vec::new();
        //header
        result.extend(info.generate_header());
        let author = &info.generate_author_info();
        let title = info.title();
        let separator = info.separator();
        let paragraphs = match content.parse_to_line(separator) {
            Ok(paragraphs) => paragraphs,
            Err(error_symbol) => {
                return Err(format!(
                    "Content 中存在英文符号「{error_symbol}」，会影响笔记生成的效果，请检查！"
                ))
            }
        };

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
deck = \"New::语文::挖空\"
card_template = \"语文::古诗文::挖空\"
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
        // 存在英文句号、英文逗号
        let mut config_err: Config = toml::from_str(
            "
[info]
author = \"李斯\"
deck = \"New::语文::挖空\"
card_template = \"语文::古诗文::挖空\"
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
        let expect = Err("Content 中存在英文符号「.」，会影响笔记生成的效果，请检查！".to_string());
        let actual = config_err.generate_with_line();
        assert_eq!(expect, actual)
    }
}
