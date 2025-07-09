use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error(transparent)]
    Serde(#[from] SerdeError),
    #[error(transparent)]
    File(#[from] FileError),
    #[error(transparent)]
    CommandLine(#[from] CLIError),
    #[error(transparent)]
    Content(#[from] ContentError),
    #[error(transparent)]
    Character(#[from] CharacterError),
}

#[derive(Debug, Error, PartialEq, Eq)]

pub enum CharacterError {
    #[error("RightQuotationMark can't be into Character")]
    RightQuotationMark,
    #[error("Invalid character: {0}")]
    InvalidCharacter(char),
}
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ContentError {
    #[error("All lines must have the same number of elements.")]
    NotSameLength,
}
#[derive(Debug, Error, PartialEq, Eq)]
pub enum CLIError {
    #[error("`--default` and `--poem` cannot be used together.")]
    DefaultTogetherWithPoem,

    #[error("`--default` and `--poem` cannot be used with `--output` together.")]
    DefaultOrPoemTogetherWithOutput,
}
#[derive(Debug, Error, PartialEq, Eq)]
pub enum FileError {
    #[error("In {filename}.\nError info: {error_info}")]
    IO {
        filename: String,
        error_info: String,
    },
}
impl FileError {
    pub fn new(filename: &str, error_info: &std::io::Error) -> Self {
        Self::IO {
            filename: filename.to_string(),
            error_info: error_info.to_string(),
        }
    }
}
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SerdeError {
    #[error(transparent)]
    Deserialize(#[from] toml::de::Error),
}
