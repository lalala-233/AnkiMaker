use super::Text;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Default)]
pub struct Content {
    paragraph: Vec<String>,
}

impl Content {
    pub fn parse_to_line(&self, separator: &str) -> Result<Vec<Vec<String>>, String> {
        // 匹配每个段落，分段、合成
        let texts: Vec<Vec<String>> = self
            .generate_texts()?
            .into_iter()
            .map(|text| text.into_vec_string())
            .map(|paragraph| {
                paragraph
                    .windows(3)
                    .map(|window| {
                        let (left, middle, right) = (&window[0], &window[1], &window[2]);
                        format!("{left}{separator}{middle}{separator}{right}")
                    })
                    .collect()
            })
            .collect();
        Ok(texts)
    }
    pub fn _new(paragraph: Vec<String>) -> Self {
        Self { paragraph }
    }
}
impl Content {
    //private
    fn generate_texts(&self) -> Result<Vec<Text>, String> {
        let mut texts = Vec::with_capacity(self.paragraph.len());
        for text in &self.paragraph {
            texts.push(
                Text::from(text).map_err(|err| format!("Content 中存在不合规的字符「{err}」。"))?,
            )
        }
        Ok(texts)
    }
}

#[cfg(test)]
mod public {
    use super::*;
    #[test]
    pub fn parse_to_line() {
        let paragraph = vec![
            "某人：你好，我好，大家好！不是吗？".to_string(),
            "哦，是的。我不是！".to_string(),
        ];
        let expect: Vec<Vec<String>> = vec![
            vec![
                "|某人：|你好，",
                "某人：|你好，|我好，",
                "你好，|我好，|大家好！",
                "我好，|大家好！|不是吗？",
                "大家好！|不是吗？|",
            ],
            vec!["|哦，|是的。", "哦，|是的。|我不是！", "是的。|我不是！|"],
        ]
        .into_iter()
        .map(|vec_str| vec_str.into_iter().map(|str| str.to_string()).collect())
        .collect();
        let actual = Content { paragraph }.parse_to_line("|").unwrap();
        assert_eq!(expect, actual);
        // 存在英文感叹号、英文冒号、英文逗号、英文问号
        let paragraph = vec![
            "哦，是的。我不是！".to_string(),
            "某人:你好,我好，大家好！不是吗?".to_string(),
            "哦，是的。我不是!".to_string(),
        ];
        let expect = Err("Content 中存在不合规的字符「:」。".to_string());
        let actual = Content { paragraph }.parse_to_line("|");
        assert_eq!(expect, actual);
    }
}
#[cfg(test)]
mod private {
    use super::{Content, Text};
    #[test]
    fn generate_texts() {
        let paragraph = vec![
            "某人：你好，我好，大家好！不是吗？".to_string(),
            "哦，是的。我不是！".to_string(),
        ];
        let expect = vec![
            Text::from(&paragraph[0]).unwrap(),
            Text::from(&paragraph[1]).unwrap(),
        ];
        let actual = Content { paragraph }.generate_texts().unwrap();
        assert_eq!(expect, actual);
        // 错误符号：英文逗号、英文逗号、英文句号
        let paragraph = vec![
            "哦，是的.我不是！".to_string(),
            "某人：你好,我好，大家好！不是吗？".to_string(),
        ];
        let expect = Err("Content 中存在不合规的字符「.」。".to_string());
        let actual = Content { paragraph }.generate_texts();
        assert_eq!(expect, actual)
    }
}
