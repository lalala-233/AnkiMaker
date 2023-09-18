use crate::symbol::Symbol;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Text {
    text: Vec<Symbol>,
}
impl Text {
    pub fn push(&mut self, symbol: Symbol) {
        let last = self.text.last_mut();
        match last {
            Some(Symbol::Symbol(last)) => match symbol {
                Symbol::Symbol(symbol) => last.push_str(&symbol),
                Symbol::RightQuotationMark => last.push(Symbol::RIGHT_QUOTATION_MARK),
                Symbol::Text(_) => self.text.push(symbol),
            },
            Some(Symbol::Text(last)) => match symbol {
                Symbol::Symbol(_) => self.text.push(symbol),
                Symbol::RightQuotationMark => last.push(Symbol::RIGHT_QUOTATION_MARK),
                Symbol::Text(text) => last.push_str(&text),
            },
            _ => self.text.push(symbol),
        }
    }
    pub fn from(text: &str) -> Result<Text, String> {
        let mut result = Text::default();
        for symbol in text.chars() {
            result.push(Symbol::from(symbol)?);
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
        let mut iter = val.text.iter();
        result.push("".to_string());
        match iter.next() {
            Some(Symbol::Symbol(symbol)) => {
                result.push(symbol.to_string());
                result
                    .last_mut()
                    .unwrap()
                    .push_str(&iter.next().unwrap().to_string());
            }
            Some(Symbol::Text(text)) => result.push(text.to_string()),
            _ => (), //RightQuotationMark are not possible in Text
        }
        for symbol in iter {
            match symbol {
                Symbol::Text(text) => result.push(text.to_string()),
                Symbol::Symbol(symbol) => result.last_mut().unwrap().push_str(symbol),
                _ => (),
            }
        }
        result.push("".to_string());
        result
    }
}
#[cfg(test)]
mod public {
    use crate::text::{Symbol, Text};
    #[test]
    pub fn from() {
        let tests = (
            "「你爱我，我爱你，蜜雪冰城甜蜜蜜！」".to_string(),
            "「yyds」是一句网络用语。".to_string(),
            "你爱我,我爱你，蜜雪冰城甜蜜蜜!".to_string(),
        );
        let symbols1 = vec![
            Symbol::Text("「你爱我".to_string()),
            Symbol::Symbol("，".to_string()),
            Symbol::Text("我爱你".to_string()),
            Symbol::Symbol("，".to_string()),
            Symbol::Text("蜜雪冰城甜蜜蜜".to_string()),
            Symbol::Symbol("！」".to_string()),
        ];
        let symbols2 = vec![
            Symbol::Text("「yyds」是一句网络用语".to_string()),
            Symbol::Symbol("。".to_string()),
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
            Symbol::Symbol("，".to_string()),
            Symbol::Text("测试".to_string()),
            Symbol::RightQuotationMark,
            Symbol::Text("测试".to_string()),
            Symbol::RightQuotationMark,
            Symbol::Symbol("，，".to_string()),
            Symbol::RightQuotationMark,
        );
        let expect = Text {
            text: vec![
                Symbol::Symbol("，".to_string()),
                Symbol::Text("测试」测试」".to_string()),
                Symbol::Symbol("，，」".to_string()),
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
