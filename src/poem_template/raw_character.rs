#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawCharacter {
    Symbol(String),
    Text(String),
    RightQuotationMark,
}
impl RawCharacter {
    const PASSED_CHARACTER: &'static [char] = &['！', '：', '；', '，', '。', '？'];
    pub const RIGHT_QUOTATION_MARK: &'static str = "」";
    pub fn from(character: char) -> Result<RawCharacter, String> {
        let is_error_char = |character: char| {
            character.is_ascii_graphic()
                || character.is_whitespace()
                || matches!(character, '‘' | '’' | '“' | '”')
        };
        if is_error_char(character) {
            return Err(format!("{character}"));
        }
        let symbol = character.to_string();
        if symbol.contains(Self::PASSED_CHARACTER) {
            return Ok(Self::Symbol(symbol));
        }
        if symbol.contains(Self::RIGHT_QUOTATION_MARK) {
            return Ok(Self::RightQuotationMark);
        }
        Ok(Self::Text(symbol.to_string()))
    }
}
impl ToString for RawCharacter {
    fn to_string(&self) -> String {
        match self {
            RawCharacter::Symbol(symbol) => symbol.to_string(),
            RawCharacter::Text(text) => text.to_string(),
            RawCharacter::RightQuotationMark => Self::RIGHT_QUOTATION_MARK.to_string(),
        }
    }
}
impl std::ops::Deref for RawCharacter {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match self {
            RawCharacter::Symbol(content) | RawCharacter::Text(content) => content,
            RawCharacter::RightQuotationMark => Self::RIGHT_QUOTATION_MARK,
        }
    }
}
#[cfg(test)]
mod public {
    use super::RawCharacter;
    #[test]
    pub fn from() {
        let test_symbol = |char| {
            let actual = RawCharacter::from(char).unwrap();
            let expect = RawCharacter::Symbol(char.to_string());
            assert_eq!(expect, actual)
        };
        let test_text = |char| {
            let actual = RawCharacter::from(char).unwrap();
            let expect = RawCharacter::Text(char.to_string());
            assert_eq!(expect, actual)
        };
        let test_err = |char| {
            let actual = RawCharacter::from(char);
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
        let actual = RawCharacter::from('」').unwrap();
        let expect = RawCharacter::RightQuotationMark;
        assert_eq!(expect, actual)
    }
}
