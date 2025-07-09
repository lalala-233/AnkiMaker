pub mod cli;
mod default_template;
mod error;
mod header;
mod mode;
mod note;
mod poem_template;
mod template;

mod prelude {
    pub use crate::{
        default_template::DefaultConfig,
        error::{CLIError, CharacterError, ContentError, Error, FileError, SerdeError},
        header::{Header, ToHeader, headers::Headers},
        mode::Mode,
        note::{Note, ToNote, notes::Notes},
        poem_template::PoemConfig,
        template::Template,
    };
    pub use serde::{Deserialize, Serialize};
}
