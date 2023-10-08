use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Text(Vec<String>);
impl Default for Text {
    fn default() -> Self {
        Self(vec!["".to_string(); 3])
    }
}
impl Text {
    pub fn into_iter(self) -> std::vec::IntoIter<String> {
        let text = self.0;
        text.into_iter()
    }
}
#[cfg(test)]
mod public {
    use super::Text;
    #[test]
    pub fn into_iter() {
        let expect:Vec<_> = vec![
            "Physiology means the scientific study of the normal functions of living things".to_string(),
            "physiology".to_string(),
            "[uncountable] the scientific study of the normal functions of living things生理学<br>[uncountable, singular] the way in which a particular living thing functions生理机能".to_string(),
        ].into_iter().collect();
        let text=Text(vec!["Physiology means the scientific study of the normal functions of living things".to_string(),  "physiology".to_string(), "[uncountable] the scientific study of the normal functions of living things生理学<br>[uncountable, singular] the way in which a particular living thing functions生理机能".to_string()]);
        let actual: Vec<_> = text.into_iter().collect();
        assert_eq!(expect, actual)
    }
}
