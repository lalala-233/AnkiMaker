use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Info {
    card_template: String,
    deck: String,
    title: String,
    author: Option<String>,
    dynasty: Option<String>,
    separator: Option<String>,
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
            title: str,
        }
    }
}
impl Info {
    const DEFAULT_SEPARATOR: &'static str = "|";
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
        header.push("#html:false".to_string());
        header.push(format!("#notetype:{}", self.card_template));
        header.push(format!("#deck:{}::{}", self.deck, self.title));
        header
    }
    pub fn separator(&self) -> &str {
        if let Some(separator) = &self.separator {
            separator
        } else {
            Self::DEFAULT_SEPARATOR
        }
    }
    pub fn title(&self) -> &String {
        &self.title
    }
    pub fn new(
        card_template: String,
        deck: String,
        title: String,
        author: Option<String>,
        dynasty: Option<String>,
        separator: Option<String>,
    ) -> Self {
        Self {
            card_template,
            deck,
            title,
            author,
            dynasty,
            separator,
        }
    }
}

#[cfg(test)]
mod public {
    use super::Info;
    use default::default;
    pub mod default {
        use super::Info;
        pub fn default() -> (
            String,
            String,
            String,
            Option<String>,
            Option<String>,
            Option<String>,
            Info,
        ) {
            let card_template = "测试模板".to_string();
            let deck = "New::语文".to_string();
            let title = "我真的好帅".to_string();
            let author = Some("我".to_string());
            let dynasty = Some("现代".to_string());
            let separator = Some("|".to_string());
            let info = Info {
                author: author.clone(),
                card_template: card_template.clone(),
                deck: deck.clone(),
                dynasty: dynasty.clone(),
                separator: separator.clone(),
                title: title.clone(),
            };
            (card_template, deck, title, author, dynasty, separator, info)
        }
    }
    #[test]
    pub fn generate_author_info() {
        let (_card_template, _deck, _titlee, author, dynasty, _separator, info) = default();
        //all
        let expect = format!("（{}）{}", dynasty.unwrap(), author.unwrap());
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
        let (card_template, _deck, _title, _author, _dynasty, separator, info) = default();
        let mut expect = vec![
            format!("#separator:{}", separator.unwrap()),
            "#html:false".to_string(),
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
        let (_card_template, _deck, title, _author, _dynasty, _separator, info) = default();
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
        let (_card_template, _deck, _title, _author, _dynasty, separator, info) = default();
        let expect = separator.unwrap();
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
