use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Info {
    notetype: String,
    deck: String,
    mode: String,
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
            notetype: str.clone(),
            deck: str.clone(),
            mode: "poem".to_string(),
            title: str,
            author: optional.clone(),
            dynasty: optional,
            separator: None,
        }
    }
}
impl Info {
    pub fn notetype(&self) -> String {
        self.notetype.clone()
    }
    pub fn deck(&self) -> String {
        format!("{}::{}", self.deck, self.title)
    }
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
    pub fn separator(&self) -> String {
        self.separator.clone().unwrap_or("|".to_string())
    }
    pub fn title(&self) -> &String {
        &self.title
    }
    pub fn _new(
        notetype: String,
        deck: String,
        mode: String,
        title: String,
        author: Option<String>,
        dynasty: Option<String>,
        separator: Option<String>,
    ) -> Self {
        Self {
            notetype,
            deck,
            mode,
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
    const DEFAULT_SEPARATOR: &str = "|";
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
            let notetype = "测试模板".to_string();
            let deck = "New::语文".to_string();
            let title = "我真的好帅".to_string();
            let author = Some("我".to_string());
            let dynasty = Some("现代".to_string());
            let separator = Some("|".to_string());
            let info = Info {
                author: author.clone(),
                notetype: notetype.clone(),
                deck: deck.clone(),
                dynasty: dynasty.clone(),
                separator: separator.clone(),
                title: title.clone(),
                ..Default::default()
            };
            (notetype, deck, title, author, dynasty, separator, info)
        }
    }
    #[test]
    pub fn generate_author_info() {
        let (_notetype, _deck, _titlee, author, dynasty, _separator, info) = default();
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
    pub fn title() {
        let (_notetype, _deck, title, _author, _dynasty, _separator, info) = default();
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
        let (_notetype, _deck, _title, _author, _dynasty, separator, info) = default();
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
        let expect = DEFAULT_SEPARATOR;
        let actual = info.separator();
        assert_eq!(expect, actual);
    }
}
