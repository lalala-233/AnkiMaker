use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Info {
    author: Option<String>,
    card_template: String,
    deck: String,
    dynasty: Option<String>,
    separator: Option<char>,
    title: String,
}
impl Default for Info {
    fn default() -> Self {
        let optional = Some("".to_string());
        let str = "".to_string();
        Self {
            author: optional.clone(),
            card_template: str.clone(),
            deck: str.clone(),
            dynasty: optional,
            separator: None,
            title: str.clone(),
        }
    }
}
impl Info {
    const DEFAULT_SEPARATOR: char = '|';
    pub fn generate_author_info(&self) -> String {
        let mut author_info = String::new();
        if let Some(dynasty) = self.dynasty.clone() {
            let str = format!("（{}）", dynasty);
            author_info.push_str(&str);
        }
        if let Some(author) = self.author.clone() {
            author_info.push_str(&author);
        }
        author_info
    }
    pub fn generate_header(&self) -> Vec<String> {
        let mut header = Vec::new();
        header.push(format!("#separator:{}", self.separator()));
        header.push("#html:true".to_string());
        header.push(format!("#notetype:{}", self.card_template));
        header.push(format!("#deck:{}::{}", self.deck, self.title));
        header
    }
    pub fn separator(&self) -> char {
        if let Some(separator) = self.separator {
            separator
        } else {
            Self::DEFAULT_SEPARATOR
        }
    }
    pub fn title(&self) -> &String {
        &self.title
    }
}

#[cfg(test)]
mod public {
    use super::*;
    #[test]
    pub fn generate_author_info() {
        let author = "我".to_string();
        let card_template = "测试模板".to_string();
        let deck = "New::语文".to_string();
        let dynasty = "现代".to_string();
        let separator = Some('|');
        let title = "我真的好帅".to_string();
        //all
        let info = Info {
            author: Some(author.clone()),
            card_template,
            deck,
            dynasty: Some(dynasty.clone()),
            separator,
            title,
        };
        let expect = format!("（{}）{}", dynasty, author);
        let actual = info.generate_author_info();
        assert_eq!(expect, actual);
        //no Option
        let info = Info {
            author: None,
            dynasty: None,
            separator: None,
            ..info
        };
        let expect = String::new();
        let actual = info.generate_author_info();
        assert_eq!(expect, actual);
    }
    #[test]
    pub fn generate_header() {
        let author = Some("我".to_string());
        let card_template = "测试模板".to_string();
        let deck = "New::语文".to_string();
        let dynasty = Some("现代".to_string());
        let separator = ',';
        let title = "我真的好帅".to_string();
        //all
        let info = Info {
            author,
            card_template: card_template.clone(),
            deck,
            dynasty,
            separator: Some(separator),
            title,
        };
        let mut expect = vec![
            format!("#separator:{}", separator),
            "#html:true".to_string(),
            format!("#notetype:{}", &card_template),
            "#deck:New::语文::我真的好帅".to_string(),
        ];
        let actual = info.generate_header();
        assert_eq!(expect, actual);
        //no Option
        let info = Info {
            author: None,
            dynasty: None,
            separator: None,
            ..info
        };
        expect[0] = format!("#separator:{}", Info::DEFAULT_SEPARATOR);
        let actual = info.generate_header();
        assert_eq!(expect, actual);
    }
    #[test]
    pub fn title() {
        let author = Some("我".to_string());
        let card_template = "测试模板".to_string();
        let deck = "New::语文".to_string();
        let dynasty = Some("现代".to_string());
        let separator = Some('|');
        let title = "我真的好帅".to_string();
        //all
        let info = Info {
            author,
            card_template,
            deck,
            dynasty,
            separator,
            title: title.clone(),
        };
        let expect = &title;
        let actual = info.title();
        assert_eq!(expect, actual);
        //no Option
        let info = Info {
            author: None,
            dynasty: None,
            separator: None,
            ..info
        };
        let actual = info.title();
        assert_eq!(expect, actual);
    }
    #[test]
    pub fn separator() {
        let author = Some("我".to_string());
        let card_template = "测试模板".to_string();
        let deck = "New::语文".to_string();
        let dynasty = Some("现代".to_string());
        let separator = ',';
        let title = "我真的好帅".to_string();
        //all
        let info = Info {
            author,
            card_template,
            deck,
            dynasty,
            separator: Some(separator),
            title,
        };
        let expect = separator;
        let actual = info.separator();
        assert_eq!(expect, actual);
        //no Option
        let info = Info {
            author: None,
            dynasty: None,
            separator: None,
            ..info
        };
        let expect = Info::DEFAULT_SEPARATOR;
        let actual = info.separator();
        assert_eq!(expect, actual);
    }
}
