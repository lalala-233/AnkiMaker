use crate::mode::Mode;
use clap::Parser;
use indicatif::ProgressIterator;
use std::{error::Error, fs};

#[derive(Parser)]
/// Generate cards from toml files.
#[command(version)]
struct AnkiMaker {
    /// Path to the toml file.
    #[arg(required = true)]
    path: Vec<String>,
    /// Use default template.
    #[arg(long)]
    default: bool,
    /// Output file name.
    #[arg(short, long)]
    output: Option<String>,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = AnkiMaker::parse();
    if args.default {
        default_file(&args.path)?;
    } else {
        // A progress bar appears, but it seems too short to see
        for filename in args.path.iter().progress() {
            let content = process_file(filename)?;
            write_to_file(filename, &content)?
        }
    }
    Ok(())
}
fn write_to_file(filename: &str, content: &str) -> Result<(), Box<dyn Error>> {
    fs::write(format!("{filename}.txt"), content)
        .map_err(|error_info| format!("Error: In {filename}.\nDetails: {error_info}"))?;
    Ok(())
}
fn process_file(filename: &str) -> Result<String, Box<dyn Error>> {
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
        Mode::Default => crate::default_template::generate(filename),
        Mode::Poem => crate::poem_template::generate(filename),
        Mode::Unknown => crate::default_template::generate(filename),
    }
}
fn default_file(filenames: &[String]) -> Result<(), Box<dyn Error>> {
    let lines = crate::poem_template::Config::default();
    let content = toml::to_string(&lines).unwrap();
    for filename in filenames.iter().progress() {
        write_to_file(filename, &content)?
    }
    Ok(())
}
