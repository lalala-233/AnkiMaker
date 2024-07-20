use crate::{config::Config, default_template::DefaultConfig, poem_template::PoemConfig};
use clap::Parser;
use indicatif::ProgressIterator;
use log::warn;
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
    /// Use poem template.
    #[arg(long)]
    poem: bool,
    /// Output file name.
    #[arg(short, long)]
    output: Option<String>,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = AnkiMaker::parse();
    match (args.default, args.poem) {
        (true, true) => Err("--default and --poem cannot be used together".into()),
        (true, false) => default_file::<DefaultConfig>(&args.path),
        (false, true) => default_file::<PoemConfig>(&args.path),
        (false, false) =>
        // A progress bar appears, but it seems too short to see
        {
            for filename in args.path.iter().progress() {
                let content = process_file(filename)?;
                write_to_file(&format!("{filename}.txt"), &content)?
            }
            Ok(())
        }
    }
}
fn write_to_file(filename: &str, content: &str) -> Result<(), Box<dyn Error>> {
    fs::write(filename, content)
        .map_err(|error_info| format!("In {filename}.\nDetails: {error_info}"))?;
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
        notetype: String,
        deck: String,
        mode: String,
    }
    let content = fs::read_to_string(filename)?;
    let toml: Config = toml::from_str(&content)?;
    match toml.info.mode.as_str() {
        "default" => generate::<DefaultConfig>(filename),
        "poem" => generate::<PoemConfig>(filename),
        mode => {
            warn!(
                "Warning: Unknown mode {mode} detected in {filename}, using default mode instead."
            );
            warn!("The file appears to have an unsupported mode configuration. Please check the file contents and ensure the mode is set correctly.");
            generate::<DefaultConfig>(filename)
        }
    }
}
fn default_file<T: Config>(filenames: &[String]) -> Result<(), Box<dyn Error>> {
    let lines = T::default();
    let content = toml::to_string(&lines).unwrap();
    for filename in filenames.iter().progress() {
        write_to_file(filename, &content)?
    }
    Ok(())
}
fn generate<T: Config>(filename: &str) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;
    let toml: T = toml::from_str(&content)?;
    let content: String = toml
        .generate()?
        .into_iter()
        .fold(String::new(), |mut output, line| {
            use std::fmt::Write;
            let _ = writeln!(output, "{}", line);
            output
        });
    Ok(content)
}
