use super::Text;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
#[derive(Deserialize, Serialize, Default, Clone)]
pub struct Content {
    paragraph: Vec<String>,
}
impl Content {
    pub fn try_into_iter(self) -> Result<impl Iterator<Item = Vec<String>>, String> {
        let iter = self.try_get_texts()?.into_iter();
        let result = iter.enumerate().flat_map(|(index_left, text)| {
            let iter = text.windows(3).map(|string| string.to_vec());
            let result = iter
                .enumerate()
                .map(|(index_reght, mut text)| {
                    text.insert(0, format!("（{}-{}）", index_left + 1, index_reght + 1));
                    text
                })
                .collect::<Vec<_>>();
            result.into_iter()
        });
        Ok(result)
    }
    pub fn _new(paragraph: Vec<String>) -> Self {
        Self { paragraph }
    }
}

impl Content {
    //private
    fn try_get_texts(&self) -> Result<Vec<Vec<String>>, String> {
        let mut texts = Vec::with_capacity(self.paragraph.len());
        for text in &self.paragraph {
            texts.push(
                Text::from_str(text)
                    .map_err(|unexpected_symbol| {
                        format!("unexpected symbol `{unexpected_symbol}` in the file.")
                    })?
                    .into(),
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
            "某人：「你好，我好，大家好！不是吗？」".to_string(),
            "义已逝，吾亦死！".to_string(),
            "哼哼啊啊啊啊啊啊啊啊".to_string(),
            "".to_string(),
            "哦，是的。我不是！".to_string(),
        ];

        let expect: Vec<Vec<_>> = vec![
            ["（1-1）", "", "某人：", "「你好，"],
            ["（1-2）", "某人：", "「你好，", "我好，"],
            ["（1-3）", "「你好，", "我好，", "大家好！"],
            ["（1-4）", "我好，", "大家好！", "不是吗？」"],
            ["（1-5）", "大家好！", "不是吗？」", ""],
            ["（2-1）", "", "义已逝，", "吾亦死！"],
            ["（2-2）", "义已逝，", "吾亦死！", ""],
            ["（3-1）", "", "哼哼啊啊啊啊啊啊啊啊", ""],
            ["（5-1）", "", "哦，", "是的。"],
            ["（5-2）", "哦，", "是的。", "我不是！"],
            ["（5-3）", "是的。", "我不是！", ""],
        ]
        .into_iter()
        .map(|vec_str| vec_str.into_iter().map(|str| str.to_string()).collect())
        .collect();
        let actual = Content { paragraph }
            .try_into_iter()
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(expect, actual);
        // 存在英文冒号（第一个）、英文逗号、英文问号、英文感叹号
        let paragraph = vec![
            "哦，是的。我不是！".to_string(),
            "某人:「你好,我好，大家好！不是吗?」".to_string(),
            "哦，是的。我不是!".to_string(),
        ];
        let expect = "unexpected symbol `:` in the file.".to_string();
        let actual = Content { paragraph }
            .try_into_iter()
            .is_err_and(|error_info| expect == error_info);
        assert!(actual)
    }
}
#[cfg(test)]
mod private {
    use super::{Content, Text};
    use std::str::FromStr;
    #[test]
    fn generate_texts() {
        let paragraph = vec![
            "某人：你好，我好，大家好！不是吗？".to_string(),
            "哦，是的。我不是！".to_string(),
        ];
        let expect: Vec<Vec<_>> = vec![
            Text::from_str(&paragraph[0]).unwrap().into(),
            Text::from_str(&paragraph[1]).unwrap().into(),
        ];
        let actual = Content { paragraph }.try_get_texts().unwrap();
        assert_eq!(expect, actual);
        // 错误符号：英文逗号、英文逗号、英文句号
        let paragraph = vec![
            "哦，是的.我不是！".to_string(),
            "某人：你好,我好，大家好！不是吗？".to_string(),
        ];
        let expect = Err("unexpected symbol `.` in the file.".to_string());
        let actual = Content { paragraph }.try_get_texts();
        assert_eq!(expect, actual)
    }
}
