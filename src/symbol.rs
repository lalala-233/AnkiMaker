#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Symbol {
    Symbol(String),
    Text(String),
}
impl Symbol {
    const ERROR_SYMBOL: &[char] = &[
        ' ', '!', '(', ')', '-', '_', ':', ';', '\'', '\"', '<', '>', '?', ',', '.',
    ];
    const PASSED_SYMBOL: &[char] = &['！', '：', '；', '，', '。', '？', '」'];
    pub fn from(symbol: char) -> Result<Symbol, String> {
        let symbol = symbol.to_string();
        if symbol.contains(Self::ERROR_SYMBOL) {
            Err(symbol)
        } else if symbol.contains(Self::PASSED_SYMBOL) {
            Ok(Self::Symbol(symbol))
        } else {
            Ok(Self::Text(symbol))
        }
    }
}
impl ToString for Symbol {
    fn to_string(&self) -> String {
        match self {
            Symbol::Symbol(symbol) => symbol.to_string(),
            Symbol::Text(text) => text.to_string(),
        }
    }
}
#[cfg(test)]
mod public {
    use crate::symbol::Symbol;

    #[test]
    pub fn from() {
        let test_symbol = |char| {
            let actual = Symbol::from(char).unwrap();
            let expect = Symbol::Symbol(char.to_string());
            assert_eq!(expect, actual)
        };
        let test_text = |char| {
            let actual = Symbol::from(char).unwrap();
            let expect = Symbol::Text(char.to_string());
            assert_eq!(expect, actual)
        };
        let test_err = |char| {
            let actual = Symbol::from(char);
            let expect = Err(char.to_string());
            assert_eq!(expect, actual)
        };
        test_symbol('：');
        test_symbol('；');
        test_symbol('，');
        test_symbol('。');
        test_symbol('！');
        test_symbol('」');
        test_text('1');
        test_text('0');
        test_text('Q');
        test_text('a');
        test_text('啊');
        test_text('我');
        test_text('平');
        test_text('、');
        test_text('（');
        test_err(',');
        test_err('!');
        test_err('(');
        test_err(':');
        test_err(';');
    }
}
