use crate::poem_template::Config;
use clap::Parser;
use indicatif::ProgressIterator;
use std::{error::Error, fs};

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
        deal_with(filename)?;
    }
    Ok(())
}

fn deal_with(filename: &str) -> Result<(), Box<dyn Error>> {
    if filename == "default" {
        default_file()?
    } else {
        generate(filename)?
    }
    Ok(())
}

fn generate(filename: &str) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(filename.clone())?;
    let mut toml: Config = toml::from_str(&content)?;
    match toml.generate_with_line() {
        Ok(lines) => {
            let lines: String = lines.into_iter().map(|line| format!("{line}\n")).collect();
            fs::write(format!("{filename}.txt"), lines)
                .map_err(|error_info| format!("Error: In {filename}.\nDetails:{error_info}"))?;
        }
        Err(error_info) => {
            let error_info = format!("Error: In {filename}.\nDetails:{error_info}");
            return Err(error_info.into());
        }
    }
    Ok(())
}
fn default_file() -> Result<(), Box<dyn Error>> {
    let filename = "default.toml";
    let lines = Config::default();
    let lines = toml::to_string(&lines).unwrap();
    fs::write(filename.clone(), lines)
        .map_err(|error_info| format!("Error: In {filename}.\nDetails:{error_info}"))?;
    Ok(())
}
