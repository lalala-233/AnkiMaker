use super::ToHeader;
use log::error;

#[derive(Debug)]
pub struct Headers {
    html: bool,
    separator: String,
}
impl Headers {
    pub fn generate_header(&self) -> Vec<String> {
        vec![
            format!("#separator:{}", self.separator),
            format!("#html:{}", self.html),
            format!("#tags column:1",),
            format!("#deck column:2",),
            format!("#notetype column:3",),
        ]
    }
    pub fn separator(&self) -> String {
        self.separator.clone()
    }
}
impl<T: ToHeader> From<&T> for Headers {
    fn from(value: &T) -> Self {
        Self {
            separator: value.separator(),
            html: value.html(),
        }
    }
}
impl std::ops::Add for Headers {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if self.html != rhs.html {
            error!("Different value in html field");
            panic!("Different value in html field");
        } else if self.separator != rhs.separator {
            error!("Different value in separator field");
            panic!("Different value in separator field");
        } else {
            self
        }
    }
}
