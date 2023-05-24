use serde::Deserialize;

#[derive(Deserialize)]
pub struct Content {
    paragraph: Vec<String>,
}

impl Content {
    const ERROR_SYMBOL: &[char] = &[
        '!', '(', ')', '-', '_', ':', ';', '\'', '\"', '<', '>', '?', ',', '.',
    ];
    const PARSED_SYMBOL: &[char] = &['！', '：', '；', '，', '。', '？'];
    pub fn has_error_symbol(&self) -> bool {
        for string in &self.paragraph {
            if string.find(Self::ERROR_SYMBOL).is_some() {
                return true;
            };
        }
        false
    }
    pub fn parse_to_line(&self, separator: char) -> Vec<Vec<String>> {
        // 匹配每个段落，分段、合成
        self.paragraph
            .iter()
            .map(|paragraph| {
                let mut lines = vec!["".to_string()];
                lines.extend(
                    paragraph
                        .split_inclusive(Self::PARSED_SYMBOL)
                        .map(String::from),
                );
                lines.push("".to_string());
                //subtrait two ""
                let mut paragraph = Vec::with_capacity(lines.len() - 2);
                paragraph.extend(lines.windows(3).map(|window| {
                    format!(
                        "{}{separator}{}{separator}{}",
                        window[0], window[1], window[2]
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
    pub fn has_error_symbol() {
        let content = Content {
            paragraph: vec!["qwe".to_string(), "asd".to_string()],
        };
        assert!(!content.has_error_symbol());
        for symbol in Content::ERROR_SYMBOL {
            let content = Content {
                paragraph: vec!["qwe".to_string(), symbol.to_string(), "asd".to_string()],
            };
            assert!(content.has_error_symbol());
        }
    }
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
