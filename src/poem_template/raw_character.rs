#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawCharacter {
    Symbol(String),
    Text(String),
    RightQuotationMark(String),
}
impl RawCharacter {
    const PASSED_SYMBOL: &'static [char] = &['！', '：', '；', '，', '。', '？'];
    const RIGHT_QUOTATION_MARK: &'static [char] = &['」', '』'];
}
impl TryFrom<char> for RawCharacter {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let is_error_char = |character: char| {
            character.is_ascii_graphic()
                || character.is_whitespace()
                || matches!(character, '‘' | '’' | '“' | '”')
        };
        if is_error_char(value) {
            return Err(format!("{value}"))?;
        }
        let symbol = value.to_string();
        if symbol.contains(Self::PASSED_SYMBOL) {
            return Ok(Self::Symbol(symbol));
        }
        if symbol.contains(Self::RIGHT_QUOTATION_MARK) {
            return Ok(Self::RightQuotationMark(symbol));
        }
        Ok(Self::Text(symbol.to_string()))
    }
}
impl std::fmt::Display for RawCharacter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RawCharacter::*;
        write!(
            f,
            "{}",
            match self {
                Symbol(content) | Text(content) | RightQuotationMark(content) => {
                    content
                }
            }
        )
    }
}
impl std::ops::Deref for RawCharacter {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match self {
            RawCharacter::Symbol(content) | RawCharacter::Text(content) => content,
            RawCharacter::RightQuotationMark(mark) => mark,
        }
    }
}
#[cfg(test)]
mod public {
    use super::RawCharacter;
    #[test]
    pub fn try_from() {
        let test_symbol = |char: char| {
            let actual = RawCharacter::try_from(char).unwrap();
            let expect = RawCharacter::Symbol(char.to_string());
            assert_eq!(expect, actual)
        };
        let test_text = |char: char| {
            let actual = RawCharacter::try_from(char).unwrap();
            let expect = RawCharacter::Text(char.to_string());
            assert_eq!(expect, actual)
        };
        let test_err = |char: char| {
            let actual = RawCharacter::try_from(char);
            let expect = Err(char.to_string());
            assert_eq!(expect, actual)
        };
        test_symbol('：');
        test_symbol('；');
        test_symbol('，');
        test_symbol('。');
        test_symbol('！');
        test_text('啊');
        test_text('我');
        test_text('平');
        test_text('、');
        test_text('（');
        test_err(',');
        test_err('!');
        test_err('(');
        test_err('@');
        test_err('8');
        test_err(':');
        test_err(';');
        let actual = RawCharacter::try_from('」').unwrap();
        let expect = RawCharacter::RightQuotationMark("」".to_string());
        assert_eq!(expect, actual);
        let actual = RawCharacter::try_from('』').unwrap();
        let expect = RawCharacter::RightQuotationMark("』".to_string());
        assert_eq!(expect, actual);
    }
}
