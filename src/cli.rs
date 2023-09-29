use crate::Config;
use clap::Parser;
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
    for path in args.path {
        if path == "default" {
            default_file()?
        } else {
            generate(path)?
        }
    }
    Ok(())
}

fn generate(mut path: String) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(path.clone())?;
    let mut toml: Config = toml::from_str(&content)?;
    match toml.generate_with_line() {
        Ok(lines) => {
            let lines: String = lines.into_iter().map(|line| format!("{line}\n")).collect();
            path.push_str(".txt");
            fs::write(path.clone(), lines)
                .map_err(|error_info| format!("Error: In {path}.\nDetails:{error_info}"))?;
        }
        Err(error_info) => {
            let error_info = format!("Error: In {path}.\nDetails:{error_info}");
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
