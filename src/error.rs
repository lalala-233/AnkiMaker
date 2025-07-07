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
pub enum CLIError {
    #[error("`--default` and `--poem` cannot be used together.")]
    DefaultAndPoemTogether,

    #[error("Use `--output` when creating multiple files by `--default` or `--poem`.")]
    UseOutputWhenCreatingMultipleFiles,
}
#[derive(Debug, Error, PartialEq, Eq)]
pub enum FileError {
    #[error("In {filename}.\nError kind: {kind}")]
    IO {
        filename: String,
        kind: std::io::ErrorKind,
    },
}
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SerdeError {
    #[error(transparent)]
    Deserialize(#[from] toml::de::Error),
}
