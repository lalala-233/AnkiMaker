use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Content {
    paragraph: Vec<String>,
}

const PARSED_SYMBOL: &[char] = &['，', '。', '：', '！', '？'];

impl Content {
    pub fn parse_to_line(&self, separator: char) -> Vec<Vec<String>> {
        // 匹配每个段落，分段、合成
        self.paragraph
            .iter()
            .map(|paragraph| {
                let mut lines = vec!["".to_string()];
                lines.extend(paragraph.split_inclusive(PARSED_SYMBOL).map(String::from));
                lines.push("".to_string());
                //subtrait two ""
                let mut paragraph = Vec::with_capacity(lines.len() - 2);
                paragraph.extend(lines.windows(3).map(|window| {
                    format!(
                        "{}{}{}{}{}",
                        window[0], separator, window[1], separator, window[2]
                    )
                }));
                paragraph
            })
            .collect()
    }
}

#[cfg(test)]
mod public {
    use super::*;
    #[test]
    pub fn parse_to_line() {
        let paragraph = vec!["某人：你好，我好，大家好！不是吗？".to_string()];
        let expect: Vec<Vec<String>> = vec![vec![
            "|某人：|你好，",
            "某人：|你好，|我好，",
            "你好，|我好，|大家好！",
            "我好，|大家好！|不是吗？",
            "大家好！|不是吗？|",
        ]]
        .into_iter()
        .map(|vec_str| vec_str.into_iter().map(|str| str.to_string()).collect())
        .collect();

        let content = Content { paragraph };
        let actual = content.parse_to_line('|');
        assert_eq!(expect, actual);
    }
}
