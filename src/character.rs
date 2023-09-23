#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Character {
    Symbol(String),
    Text(String),
    RightQuotationMark,
}
impl Character {
    const PASSED_SYMBOL: &[char] = &['！', '：', '；', '，', '。', '？'];
    pub const RIGHT_QUOTATION_MARK: char = '」';
    pub fn from(symbol: char) -> Result<Character, String> {
        let is_error_char = |symbol: char| {
            symbol.is_ascii_graphic()
                || symbol.is_whitespace()
                || matches!(symbol, '‘' | '’' | '“' | '”')
        };

        if is_error_char(symbol) {
            return Err(symbol.to_string());
        }
        let symbol = symbol.to_string();
        if symbol.contains(Self::PASSED_SYMBOL) {
            return Ok(Self::Symbol(symbol));
        }
        if symbol.contains(Self::RIGHT_QUOTATION_MARK) {
            return Ok(Self::RightQuotationMark);
        }
        Ok(Self::Text(symbol.to_string()))
    }
}
impl ToString for Character {
    fn to_string(&self) -> String {
        match self {
            Character::Symbol(symbol) => symbol.to_string(),
            Character::Text(text) => text.to_string(),
            Character::RightQuotationMark => Self::RIGHT_QUOTATION_MARK.to_string(),
        }
    }
}
#[cfg(test)]
mod public {
    use crate::character::Character;

    #[test]
    pub fn from() {
        let test_symbol = |char| {
            let actual = Character::from(char).unwrap();
            let expect = Character::Symbol(char.to_string());
            assert_eq!(expect, actual)
        };
        let test_text = |char| {
            let actual = Character::from(char).unwrap();
            let expect = Character::Text(char.to_string());
            assert_eq!(expect, actual)
        };
        let test_err = |char| {
            let actual = Character::from(char);
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
        let actual = Character::from('」').unwrap();
        let expect = Character::RightQuotationMark;
        assert_eq!(expect, actual)
    }
}
