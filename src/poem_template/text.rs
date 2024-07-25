use super::Character;
use super::RawCharacter;
use log::warn;
use std::str::FromStr;
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Text {
    text: Vec<Character>,
}
impl Text {
    pub fn push(&mut self, raw_char: RawCharacter) {
        let last = self.text.last_mut();
        match (last, raw_char.clone()) {
            (Some(Character::Symbol(last)), RawCharacter::Symbol(second_symbol)) => {
                warn!("Symbol followed by symbol.");
                warn!("Try to search {}{} for help.", last, second_symbol);
                last.push_str(&raw_char);
            }
            (None, _)
            | (Some(Character::Text(_)), RawCharacter::Symbol(_))
            | (Some(Character::Symbol(_)), RawCharacter::Text(_)) => {
                self.text.push(raw_char.into())
            }

            (Some(character), _) => character.push_str(&raw_char),
        }
    }
}
impl FromStr for Text {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Text::default();
        for symbol in s.chars() {
            result.push(RawCharacter::try_from(symbol)?);
        }
        Ok(result)
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
            None => (), //Just a empty paragraph
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
    use std::str::FromStr;
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
            Text::from_str(&tests.0).unwrap(),
            Text::from_str(&tests.1).unwrap(),
            Text::from_str(&tests.2),
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
            Text::from_str("「你爱我，我爱你，蜜雪冰城甜蜜蜜！」")
                .unwrap()
                .into(),
            Text::from_str("你爱我，我爱你，蜜雪冰城甜蜜蜜！")
                .unwrap()
                .into(),
        );

        assert_eq!(expect, actual)
    }
    #[test]
    pub fn push() {
        let raw_chars = vec![
            RawCharacter::Symbol("，".to_string()),
            RawCharacter::Text("测试".to_string()),
            RawCharacter::Symbol("，".to_string()),
            RawCharacter::RightQuotationMark("』".to_string()),
            RawCharacter::Text("测试".to_string()),
            RawCharacter::RightQuotationMark("」".to_string()),
            RawCharacter::Text("测试".to_string()),
            RawCharacter::RightQuotationMark("」".to_string()),
            RawCharacter::Symbol("，，".to_string()),
            RawCharacter::RightQuotationMark("』".to_string()),
        ];
        let expect = Text {
            text: vec![
                Character::Symbol("，".to_string()),
                Character::Text("测试".to_string()),
                Character::Symbol("，』".to_string()),
                Character::Text("测试」测试」".to_string()),
                Character::Symbol("，，』".to_string()),
            ],
        };
        let actual = raw_chars
            .into_iter()
            .fold(Text::default(), |mut text, raw_char| {
                text.push(raw_char);
                text
            });
        assert_eq!(expect, actual);
    }
}
