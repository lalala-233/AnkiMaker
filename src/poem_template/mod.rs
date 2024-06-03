mod character;
mod config;
mod content;
mod info;
mod raw_character;
mod text;
pub use character::Character;
pub use config::Config;
pub use content::Content;
pub use info::Info;
pub use raw_character::RawCharacter;
pub use text::Text;

use std::{error::Error, fs};
pub fn generate(filename: &str) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;
    let mut toml: Config = toml::from_str(&content)?;
    let content = match toml.generate_with_line() {
        Ok(lines) => lines.into_iter().fold(String::new(), |mut output, line| {
            use std::fmt::Write;
            let _ = writeln!(output, "{}", line);
            output
        }),
        Err(error_info) => {
            let error_info = format!("Error: In {filename}.\nDetails:{error_info}");
            return Err(error_info.into());
        }
    };
    Ok(content)
}
