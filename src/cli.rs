use clap::Parser;
use indicatif::ProgressIterator;
use std::{error::Error, fs};

use crate::mode::Mode;

#[derive(Parser)]
/// Generate cards from files.
#[command(version)]
struct AnkiMaker {
    /// The path to the file to read
    #[arg(required = true)]
    path: Vec<String>,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = AnkiMaker::parse();
    // A progress bar appears, but it seems too short to see
    for filename in args.path.iter().progress() {
        choose_template(filename)?;
    }
    Ok(())
}

fn choose_template(filename: &str) -> Result<(), Box<dyn Error>> {
    if filename == "default" {
        default_file()?;
        return Ok(());
    }
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Default)]
    struct Config {
        info: Info,
    }
    #[derive(Deserialize, Serialize, Default)]
    struct Info {
        card_template: String,
        deck: String,
        mode: String,
    }
    let content = fs::read_to_string(filename)?;
    let toml: Config = toml::from_str(&content)?;
    match toml.info.mode.into() {
        Mode::Default => crate::default_template::generate(filename)?,
        Mode::Poem => crate::poem_template::generate(filename)?,
        Mode::Unknown => crate::default_template::generate(filename)?,
    }
    Ok(())
}
fn default_file() -> Result<(), Box<dyn Error>> {
    let filename = "default.toml";
    let lines = crate::poem_template::Config::default();
    let lines = toml::to_string(&lines).unwrap();
    fs::write(filename, lines)
        .map_err(|error_info| format!("Error: In {filename}.\nDetails:{error_info}"))?;
    Ok(())
}
