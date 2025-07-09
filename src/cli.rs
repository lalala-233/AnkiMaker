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
    let mut filenames = args.path.into_iter().progress();
    match (args.default, args.poem, args.output) {
        (false, false, Some(output)) => {
            if filenames.len() == 1
                && let Some(filename) = filenames.next()
            {
                generate_one_file_to(&filename, &output)
            } else {
                generate_files_to(filenames, &output)
            }
        }
        (false, false, None) => generate_files(filenames),
        (true, false, None) => generate_default_file::<DefaultConfig>(filenames),
        (false, true, None) => generate_default_file::<PoemConfig>(filenames),
        (_, _, Some(_)) => Err(CLIError::DefaultOrPoemTogetherWithOutput)?,
        (true, true, _) => Err(CLIError::DefaultTogetherWithPoem)?,
    }
}
fn generate_one_file_to(filename: &str, output: &str) -> Result<(), Error> {
    let note = get_note_from_name(filename)?.generate().join("\n");
    write_file(output, &note)
}
fn generate_files_to(filenames: impl Iterator<Item = String>, output: &str) -> Result<(), Error> {
    let mut iter = filenames.map(|filename| get_notes_from_name(&filename));
    let first: Notes = iter.next().unwrap()?;
    let notes = iter.try_fold(first, |acc, note| note.map(|note| acc + note))?;

    let notes = notes.generate().join("\n");
    write_file(output, &notes)
}
fn generate_files(filenames: impl Iterator<Item = String>) -> Result<(), Error> {
    for filename in filenames {
        generate_one_file_to(&filename, &format!("{filename}.txt"))?;
    }
    Ok(())
}
fn generate_default_file<T: Template>(
    filenames: impl Iterator<Item = String>,
) -> Result<(), Error> {
    let content = toml::to_string(&T::default()).unwrap();
    for filename in filenames {
        write_file(&filename, &content)?;
    }
    Ok(())
}
fn read_file(filename: &str) -> Result<String, FileError> {
    fs::read_to_string(filename).map_err(|error_info| FileError::new(filename, &error_info))
}
fn write_file<'a>(filename: &'a str, content: &'a str) -> Result<(), Error> {
    fs::write(filename, content)
        .map_err(|error_info| Error::File(FileError::new(filename, &error_info)))
}
fn detect_mode(filename: &str, file_content: &str) -> Result<Mode, Error> {
    #[derive(Deserialize, Serialize, Default)]
    struct Config {
        info: Info,
    }
    #[derive(Deserialize, Serialize, Default)]
    struct Info {
        mode: String,
    }
    let config: Config = parse_from_toml(file_content)?;
    let mode = config.info.mode.into();
    if let Mode::Unknown(ref mode) = mode {
        warn!("Unknown mode {mode} detected in {filename}, using default mode instead.");
        warn!(
            "The file appears to have an unsupported mode configuration. Please check the file contents and ensure the mode is set correctly."
        );
    }
    Ok(mode)
}
fn get_note_from_name(filename: &str) -> Result<Note, Error> {
    let content = read_file(filename)?;
    let mode = detect_mode(filename, &content)?;
    match mode {
        Mode::Poem => get_note_from_content::<PoemConfig>(&content),
        Mode::Default | Mode::Unknown(_) => get_note_from_content::<DefaultConfig>(&content),
    }
}
fn get_notes_from_name(filename: &str) -> Result<Notes, Error> {
    get_note_from_name(filename).map(Notes::from)
}
fn get_note_from_content<T: Template>(content: &str) -> Result<Note, Error> {
    parse_from_toml::<T>(content)?.try_get_note()
}
fn parse_from_toml<'a, T: Deserialize<'a>>(content: &'a str) -> Result<T, SerdeError> {
    toml::from_str::<T>(content).map_err(SerdeError::from)
}
