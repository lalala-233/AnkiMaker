use std::{error::Error, fs};

mod config;
mod content;
mod info;
mod text;
pub use config::Config;
pub use content::Content;
pub use info::Info;
pub use text::Text;

pub fn generate(filename: &str) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;
    let toml: Config = toml::from_str(&content)?;
    let content: String = toml
        .generate()
        .into_iter()
        .fold(String::new(), |mut output, line| {
            use std::fmt::Write;
            let _ = writeln!(output, "{}", line);
            output
        });
    Ok(content)
}
