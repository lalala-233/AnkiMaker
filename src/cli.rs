use crate::{
    config::Config,
    default_template::DefaultConfig,
    notes::{Notes, ToNotes},
    poem_template::PoemConfig,
};
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
        (false, false) =>
        // A progress bar appears, but it seems too short to see
        {
            if let Some(filename) = args.output {
                let mut filenames = args.path.into_iter().progress();
                let mut notes = try_get_notes(&filenames.next().unwrap())?;
                for filename in filenames {
                    notes = notes + try_get_notes(&filename)?;
                }
                let content = notes.generate().join("\n");
                write_to_file(&filename, &content)?
            } else {
                for filename in args.path.into_iter().progress() {
                    let content = process_file(&filename)?;
                    write_to_file(&format!("{filename}.txt"), &content)?
                }
            }
            Ok(())
        }
        (true, false) => default_file::<DefaultConfig>(&args.path, args.output),
        (false, true) => default_file::<PoemConfig>(&args.path, args.output),
        (true, true) => Err("--default and --poem cannot be used together.")?,
    }
}
fn write_to_file(filename: &str, content: &str) -> Result<(), Box<dyn Error>> {
    fs::write(filename, content)
        .map_err(|error_info| format!("In {filename}.\nDetails: {error_info}"))?;
    Ok(())
}
fn process_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let mode = try_detect_mode(filename)?;
    match mode.as_str() {
        "default" => generate::<DefaultConfig>(filename),
        "poem" => generate::<PoemConfig>(filename),
        mode => {
            warn!("Unknown mode {mode} detected in {filename}, using default mode instead.");
            warn!("The file appears to have an unsupported mode configuration. Please check the file contents and ensure the mode is set correctly.");
            generate::<DefaultConfig>(filename)
        }
    }
}
fn default_file<T: Config>(
    filenames: &[String],
    output: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let lines = T::default();
    match output {
        Some(filename) if filenames.len() == 1 => {
            warn!("Using --output when using --default or --poem is not recommended.");
            let content = toml::to_string(&lines).unwrap();
            write_to_file(&filename, &content)?
        }
        None => {
            let content = toml::to_string(&lines).unwrap();
            for filename in filenames.iter().progress() {
                write_to_file(filename, &content)?
            }
        }
        _ => {
            Err("Use --output when creating multiple files using --default or --poem.".to_string())?
        }
    }
    Ok(())
}
fn generate<T: Config>(filename: &str) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;
    let toml: T = toml::from_str(&content)?;
    let content: String = toml.generate()?.join("\n");
    Ok(content)
}
fn try_detect_mode(filename: &str) -> Result<String, Box<dyn Error>> {
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Default)]
    struct Config {
        info: Info,
    }
    #[derive(Deserialize, Serialize, Default)]
    struct Info {
        mode: String,
    }
    let content = fs::read_to_string(filename)?;
    let toml: Config = toml::from_str(&content)?;
    Ok(toml.info.mode)
}
fn try_get_notes(filename: &str) -> Result<Notes, Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;
    let mode = try_detect_mode(filename)?;
    let notes = match mode.as_str() {
        "default" => {
            let toml: DefaultConfig = toml::from_str(&content)?;
            toml.try_get_notes()
        }
        "poem" => {
            let toml: PoemConfig = toml::from_str(&content)?;
            toml.try_get_notes()
        }
        mode => {
            warn!("Unknown mode {mode} detected in {filename}, using default mode instead.");
            warn!("The file appears to have an unsupported mode configuration. Please check the file contents and ensure the mode is set correctly.");
            let toml: DefaultConfig = toml::from_str(&content)?;
            toml.try_get_notes()
        }
    }?;
    Ok(notes)
}
