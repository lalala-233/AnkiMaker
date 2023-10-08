use std::{error::Error, fs};

mod config;
mod content;
mod info;
mod text;
pub use config::Config;
pub use content::Content;
pub use info::Info;
pub use text::Text;
pub fn generate(filename: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}
