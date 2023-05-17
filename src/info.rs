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
        let expect = info.generate_header();
        let actual = vec![
            format!("#separator:{}", SEPARATOR),
            "#html:true".to_string(),
            format!("#notetype:{}", &notetype),
            format!("#deck:{}", &title),
        ];
        assert_eq!(expect, actual);
        //no author
        let info = Info {
            author: None,
            ..info
        };
        let expect = info.generate_header();
        assert_eq!(expect, actual);
        //no author and dynasty
        let info = Info {
            dynasty: None,
            ..info
        };
        let expect = info.generate_header();
        assert_eq!(expect, actual);
        //no dynasty
        let info = Info {
            author,
            dynasty,
            ..info
        };
        let expect = info.generate_header();
        assert_eq!(expect, actual);
    }
}
