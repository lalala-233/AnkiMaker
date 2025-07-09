use crate::prelude::*;
use clap::Parser;
use indicatif::ProgressIterator;
use log::warn;
use std::fs;

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

/// # Errors
///
/// Return Error if there is an internal error.
pub fn run() -> Result<(), Error> {
    let args = AnkiMaker::parse();
    let filenames = &args.path;
    match (args.default, args.poem, args.output) {
        (false, false, Some(output)) => generate_files_to(filenames, &output),
        (_, _, Some(_)) => Err(CLIError::DefaultOrPoemTogetherWithOutput)?,
        (false, false, None) => generate_each_file(filenames),
        (true, false, None) => generate_default_file::<DefaultConfig>(filenames),
        (false, true, None) => generate_default_file::<PoemConfig>(filenames),
        (true, true, _) => Err(CLIError::DefaultTogetherWithPoem)?,
    }
}
fn generate_files_to(filenames: &[String], output: &str) -> Result<(), Error> {
    let mut filenames = filenames.iter().progress();
    let mut notes = if let Some(filename) = filenames.next() {
        try_get_notes(filename)?
    } else {
        unreachable!()
    };
    for filename in filenames {
        notes = notes + try_get_notes(filename)?;
    }
    let content = notes.generate().join("\n");
    write_to_file(output, &content)?;
    Ok(())
}
fn generate_each_file(filenames: &[String]) -> Result<(), Error> {
    for filename in filenames.iter().progress() {
        let content = process_file(filename)?;
        write_to_file(&format!("{filename}.txt"), &content)?;
    }
    Ok(())
}
fn generate_default_file<T: Config>(filenames: &[String]) -> Result<(), Error> {
    let content = toml::to_string(&T::default()).unwrap();
    for filename in filenames.iter().progress() {
        write_to_file(filename, &content)?;
    }
    Ok(())
}
fn generate<T: Config>(filename: &str) -> Result<String, Error> {
    let content = read_file(filename)?;
    let config: T = toml::from_str(&content).map_err(SerdeError::from)?;
    let content: String = config.generate()?.join("\n");
    Ok(content)
}
fn write_to_file<'a>(filename: &'a str, content: &'a str) -> Result<(), FileError> {
    fs::write(filename, content).map_err(|error_info| FileError::IO {
        filename: filename.to_string(),
        kind: error_info.kind(),
    })
}
fn process_file(filename: &str) -> Result<String, Error> {
    let mode = try_detect_mode(filename)?;
    match mode.as_str() {
        "default" => generate::<DefaultConfig>(filename),
        "poem" => generate::<PoemConfig>(filename),
        mode => {
            warn!("Unknown mode {mode} detected in {filename}, using default mode instead.");
            warn!(
                "The file appears to have an unsupported mode configuration. Please check the file contents and ensure the mode is set correctly."
            );
            generate::<DefaultConfig>(filename)
        }
    }
}
fn read_file(filename: &str) -> Result<String, FileError> {
    fs::read_to_string(filename).map_err(|error_info| FileError::IO {
        filename: filename.to_string(),
        kind: error_info.kind(),
    })
}
fn try_detect_mode(filename: &str) -> Result<String, Error> {
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Default)]
    struct Config {
        info: Info,
    }
    #[derive(Deserialize, Serialize, Default)]
    struct Info {
        mode: String,
    }
    let content = read_file(filename)?;
    let toml: Config = toml::from_str(&content).map_err(SerdeError::from)?;
    Ok(toml.info.mode)
}
fn try_get_notes(filename: &str) -> Result<Notes, Error> {
    let content = read_file(filename)?;
    let mode = try_detect_mode(filename)?;
    match mode.as_str() {
        "default" => {
            let toml: DefaultConfig = toml::from_str(&content).map_err(SerdeError::from)?;
            Ok(toml.try_get_notes()?)
        }
        "poem" => {
            let toml: PoemConfig = toml::from_str(&content).map_err(SerdeError::from)?;
            Ok(toml.try_get_notes()?)
        }
        mode => {
            warn!("Unknown mode {mode} detected in {filename}, using default mode instead.");
            warn!(
                "The file appears to have an unsupported mode configuration. Please check the file contents and ensure the mode is set correctly."
            );
            let toml: DefaultConfig = toml::from_str(&content).map_err(SerdeError::from)?;
            Ok(toml.try_get_notes()?)
        }
    }
}
