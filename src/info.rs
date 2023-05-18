use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Info {
    author: Option<String>,
    dynasty: Option<String>,
    notetype: String,
    term: String,
    title: String,
}

const SEPARATOR: char = '|';

impl Info {
    pub fn title(&self) -> &String {
        &self.title
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
    pub fn generate_header(&self) -> Vec<String> {
        let mut header = Vec::new();
        header.push(format!("#separator:{}", SEPARATOR));
        header.push("#html:true".to_string());
        header.push(format!("#notetype:{}", self.notetype));
        header.push(format!("#deck:{}", self.title));
        header
    }
}

#[cfg(test)]
mod public {
    use super::*;
    #[test]
    pub fn generate_author_info() {
        let author = Some("我".to_string());
        let dynasty = Some("现代".to_string());
        let notetype = "测试类型".to_string();
        let term = "高中".to_string();
        let title = "我真的好帅".to_string();
        //all
        let info = Info {
            author: author.clone(),
            dynasty: dynasty.clone(),
            notetype,
            term,
            title,
        };
        let expect = format!(
            "（{}）{}",
            dynasty.clone().unwrap(),
            author.clone().unwrap()
        );
        let actual = info.generate_author_info();
        assert_eq!(expect, actual);
        //no author
        let info = Info {
            author: None,
            ..info
        };
        let expect = format!("（{}）", dynasty.unwrap());
        let actual = info.generate_author_info();
        assert_eq!(expect, actual);
        //no author and dynasty
        let info = Info {
            dynasty: None,
            ..info
        };
        let expect = String::new();
        let actual = info.generate_author_info();
        assert_eq!(expect, actual);
        //no dynasty
        let info = Info {
            author: author.clone(),
            ..info
        };
        let expect = author.unwrap();
        let actual = info.generate_author_info();
        assert_eq!(expect, actual);
    }
    #[test]
    pub fn generate_header() {
        let author = Some("我".to_string());
        let dynasty = Some("现代".to_string());
        let notetype = "测试类型".to_string();
        let term = "高中".to_string();
        let title = "我真的好帅".to_string();
        //all
        let info = Info {
            author: author.clone(),
            dynasty: dynasty.clone(),
            notetype: notetype.clone(),
            term,
            title: title.clone(),
        };
        let expect = vec![
            format!("#separator:{}", SEPARATOR),
            "#html:true".to_string(),
            format!("#notetype:{}", &notetype),
            format!("#deck:{}", &title),
        ];
        let actual = info.generate_header();
        assert_eq!(expect, actual);
        //no author
        let info = Info {
            author: None,
            ..info
        };
        let actual = info.generate_header();
        assert_eq!(expect, actual);
        //no author and dynasty
        let info = Info {
            dynasty: None,
            ..info
        };
        let actual = info.generate_header();
        assert_eq!(expect, actual);
        //no dynasty
        let info = Info {
            author,
            dynasty,
            ..info
        };
        let actual = info.generate_header();
        assert_eq!(expect, actual);
    }
    #[test]
    pub fn title() {
        let author = Some("我".to_string());
        let dynasty = Some("现代".to_string());
        let notetype = "测试类型".to_string();
        let term = "高中".to_string();
        let title = "我真的好帅".to_string();
        //all
        let info = Info {
            author: author.clone(),
            dynasty: dynasty.clone(),
            notetype,
            term,
            title: title.clone(),
        };
        let expect = &title;
        let actual = info.title();
        assert_eq!(expect, actual);
        //no author
        let info = Info {
            author: None,
            ..info
        };
        let actual = info.title();
        assert_eq!(expect, actual);
        //no author and dynasty
        let info = Info {
            dynasty: None,
            ..info
        };
        let actual = info.title();
        assert_eq!(expect, actual);
        //no dynasty
        let info = Info {
            author,
            dynasty,
            ..info
        };
        let actual = info.title();
        assert_eq!(expect, actual);
    }
}
