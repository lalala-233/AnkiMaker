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
    let content = fs::read_to_string(filename)?;
    let toml: Config = toml::from_str(&content)?;
    let lines: String = toml
        .generate()
        .into_iter()
        .map(|line| format!("{}\n", line))
        .collect();
    eprintln!("{}", lines);
    /* fs::write(format!("{filename}.txt"), lines)
    .map_err(|error_info| format!("Error: In {filename}.\nDetails: {error_info}"))?; */
    Ok(())
}
