use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    author: Option<String>,
    card_template: String,
    deck: String,
    dynasty: Option<String>,
    separator: Option<char>,
    term: Option<String>,
    title: String,
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
    pub fn set_note_type(&mut self, note_type: &str) {
        self.deck = format!("{}::{}", self.deck, note_type);
        self.card_template = format!("{}::{}", self.card_template, note_type);
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
        let term = Some("高中".to_string());
        let title = "我真的好帅".to_string();
        //all
        let info = Info {
            author: Some(author.clone()),
            card_template,
            deck,
            dynasty: Some(dynasty.clone()),
            separator,
            term,
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
        let term = Some("高中".to_string());
        let title = "我真的好帅".to_string();
        //all
        let info = Info {
            author,
            card_template: card_template.clone(),
            deck,
            dynasty,
            separator: Some(separator),
            term,
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
    pub fn set_note_type() {
        let author = Some("我".to_string());
        let card_template = "测试模板".to_string();
        let deck = "New::语文".to_string();
        let dynasty = Some("现代".to_string());
        let separator = Some('|');
        let term = Some("高中".to_string());
        let title = "我真的好帅".to_string();
        //all
        let mut info = Info {
            author,
            card_template: card_template.clone(),
            deck: deck.clone(),
            dynasty,
            separator,
            term,
            title,
        };
        let note_type = "type".to_string();
        let expect = ("New::语文::type".to_string(), "测试模板::type".to_string());
        info.set_note_type(&note_type);
        let actual = (info.deck, info.card_template);
        assert_eq!(expect, actual);
        //no Option
        let mut info = Info {
            author: None,
            card_template,
            deck,
            dynasty: None,
            separator: None,
            ..info
        };
        info.set_note_type(&note_type);
        let actual = (info.deck, info.card_template);
        assert_eq!(expect, actual);
    }
    #[test]
    pub fn title() {
        let author = Some("我".to_string());
        let card_template = "测试模板".to_string();
        let deck = "New::语文".to_string();
        let dynasty = Some("现代".to_string());
        let separator = Some('|');
        let term = Some("高中".to_string());
        let title = "我真的好帅".to_string();
        //all
        let info = Info {
            author,
            card_template,
            deck,
            dynasty,
            separator,
            term,
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
        let term = Some("高中".to_string());
        let title = "我真的好帅".to_string();
        //all
        let info = Info {
            author,
            card_template,
            deck,
            dynasty,
            separator: Some(separator),
            term,
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
