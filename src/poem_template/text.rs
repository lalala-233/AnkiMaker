use super::Character;
use super::RawCharacter;
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Text {
    text: Vec<Character>,
}
impl Text {
    pub fn push(&mut self, raw_char: RawCharacter) {
        let last = self.text.last_mut();
        match last {
            Some(Character::Symbol(last)) => match raw_char {
                RawCharacter::Symbol(_) | RawCharacter::RightQuotationMark => {
                    last.push_str(&raw_char)
                }
                RawCharacter::Text(_) => self.text.push(raw_char.into()),
            },
            Some(Character::Text(last)) => match raw_char {
                RawCharacter::Symbol(_) => self.text.push(raw_char.into()),
                RawCharacter::RightQuotationMark | RawCharacter::Text(_) => {
                    last.push_str(&raw_char)
                }
            },
            None => self.text.push(raw_char.into()),
        }
    }
    pub fn from(text: &str) -> Result<Text, String> {
        let mut result = Text::default();
        for symbol in text.chars() {
            result.push(RawCharacter::from(symbol)?);
        }
        Ok(result)
    }
    pub fn into_vec_string(self) -> Vec<String> {
        Vec::from(self)
    }
}
impl From<Text> for Vec<String> {
    fn from(val: Text) -> Self {
        let mut result = Vec::with_capacity(val.text.len() / 2 + 3);
        let mut iter = val.text.into_iter();
        result.push("".to_string());
        match iter.next() {
            Some(Character::Symbol(symbol)) => {
                result.push(symbol);
                result.last_mut().unwrap().push_str(&iter.next().unwrap());
            }
            Some(Character::Text(text)) => result.push(text),
            _ => (), //Just a empty paragraph
        }
        for symbol in iter {
            match symbol {
                Character::Text(text) => result.push(text),
                Character::Symbol(symbol) => result.last_mut().unwrap().push_str(&symbol),
            }
        }
        result.push("".to_string());
        result
    }
}
#[cfg(test)]
mod public {
    use super::{Character, RawCharacter, Text};
    #[test]
    pub fn from() {
        let tests = (
            "「你爱我，我爱你，蜜雪冰城甜蜜蜜！」".to_string(),
            "「鸡汤来咯」是一句网络用语。".to_string(),
            "你爱我,我爱你，蜜雪冰城甜蜜蜜!".to_string(),
        );
        let symbols1 = vec![
            Character::Text("「你爱我".to_string()),
            Character::Symbol("，".to_string()),
            Character::Text("我爱你".to_string()),
            Character::Symbol("，".to_string()),
            Character::Text("蜜雪冰城甜蜜蜜".to_string()),
            Character::Symbol("！」".to_string()),
        ];
        let symbols2 = vec![
            Character::Text("「鸡汤来咯」是一句网络用语".to_string()),
            Character::Symbol("。".to_string()),
        ];
        let expect = (
            Text { text: symbols1 },
            Text { text: symbols2 },
            Err(",".to_string()),
        );
        let actual = (
            Text::from(&tests.0).unwrap(),
            Text::from(&tests.1).unwrap(),
            Text::from(&tests.2),
        );
        assert_eq!(expect, actual)
    }
    #[test]
    pub fn into_string() {
        let expect = (
            vec![
                "".to_string(),
                "「你爱我，".to_string(),
                "我爱你，".to_string(),
                "蜜雪冰城甜蜜蜜！」".to_string(),
                "".to_string(),
            ],
            vec![
                "".to_string(),
                "你爱我，".to_string(),
                "我爱你，".to_string(),
                "蜜雪冰城甜蜜蜜！".to_string(),
                "".to_string(),
            ],
        );
        let actual = (
            Text::from("「你爱我，我爱你，蜜雪冰城甜蜜蜜！」")
                .unwrap()
                .into_vec_string(),
            Text::from("你爱我，我爱你，蜜雪冰城甜蜜蜜！")
                .unwrap()
                .into_vec_string(),
        );

        assert_eq!(expect, actual)
    }
    #[test]
    pub fn push() {
        let symbols = (
            RawCharacter::Symbol("，".to_string()),
            RawCharacter::Text("测试".to_string()),
            RawCharacter::RightQuotationMark,
            RawCharacter::Text("测试".to_string()),
            RawCharacter::RightQuotationMark,
            RawCharacter::Symbol("，，".to_string()),
            RawCharacter::RightQuotationMark,
        );
        let expect = Text {
            text: vec![
                Character::Symbol("，".to_string()),
                Character::Text("测试」测试」".to_string()),
                Character::Symbol("，，」".to_string()),
            ],
        };
        let mut actual = Text::default();
        actual.push(symbols.0);
        actual.push(symbols.1);
        actual.push(symbols.2);
        actual.push(symbols.3);
        actual.push(symbols.4);
        actual.push(symbols.5);
        actual.push(symbols.6);
        assert_eq!(expect, actual);
    }
}
