pub mod cli;
mod config;
mod default_template;
mod error;
mod header;
mod notes;
mod poem_template;

mod prelude {
    pub use crate::{
        config::Config,
        default_template::DefaultConfig,
        error::{CLIError, CharacterError, ContentError, Error, FileError, SerdeError},
        header::{Headers, SingleFileHeader, ToHeader},
        notes::{Notes, ToNotes},
        poem_template::PoemConfig,
    };
    pub use serde::{Deserialize, Serialize};
}
