use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct Content {
    paragraph: Vec<Vec<String>>,
}
impl Content {
    pub fn into_iter(self) -> impl Iterator<Item = Vec<String>> {
        let length = self.paragraph.first().unwrap().len();
        self.paragraph.into_iter().map(move |text| {
            if text.len() != length {
                error!("All lines must have the same number of elements");
                error!("Error at: {:?}", text);
                panic!("All lines must have the same number of elements");
            }
            text
        })
    }
    pub fn _new(paragraph: Vec<Vec<String>>) -> Self {
        Self {
            paragraph: paragraph.into_iter().map(|text| text.into()).collect(),
        }
    }
}

#[cfg(test)]
mod public {
    use super::Content;
    #[test]
    fn into_iter() {
        let expect = vec![
            vec![
                "信誓旦旦",
                "誓言诚恳可信。",
                "出自《诗经·卫风·氓》：「总角之宴，言笑晏晏，信誓旦旦，不思其反。」
旦旦：诚实的样子。",
            ],
            vec![
                "信手拈来",
                "随手拿来。
形容做事轻松不费力，好像随手就能得到或完成一样。",
                "信手：随手。
拈：用手指捏取东西。",
            ],
        ];
        let content = Content {
            paragraph: expect
                .iter()
                .map(|text| text.into_iter().map(|str| str.to_string()).collect())
                .collect(),
        };
        let actual: Vec<_> = content.into_iter().collect();
        assert_eq!(expect, actual)
    }
    #[test]
    #[should_panic]
    fn panic_into_iter() {
        let expect = vec![
            vec![
                "信誓旦旦",
                "誓言诚恳可信。",
                "出自《诗经·卫风·氓》：「总角之宴，言笑晏晏，信誓旦旦，不思其反。」",
                "旦旦：诚实的样子。",
            ],
            vec![
                "信手拈来",
                "随手拿来。
形容做事轻松不费力，好像随手就能得到或完成一样。",
                "信手：随手。
拈：用手指捏取东西。",
            ],
        ];
        let content = Content {
            paragraph: expect
                .iter()
                .map(|text| text.into_iter().map(|str| str.to_string()).collect())
                .collect(),
        };
        let actual: Vec<_> = content.into_iter().collect();
        assert_eq!(expect, actual)
    }
}
