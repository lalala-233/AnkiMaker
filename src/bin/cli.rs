use std::{env, error::Error, fs};

fn main() {
    run().unwrap_or_else(|error| {
        eprintln!("错误！错误信息：{}", error);
    })
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    if args.len() == 1 {
        return Err("参数过少！".into());
    }
    args.next();
    for path in args {
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
    let mut toml: anki_maker::Config = toml::from_str(&content)?;
    match toml.generate_with_line() {
        Ok(lines) => {
            let lines: String = lines.into_iter().map(|line| format!("{line}\n")).collect();
            path.push_str(".txt");
            fs::write(path, lines)?;
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
    let lines = anki_maker::Config::default();
    let lines = toml::to_string(&lines).unwrap();
    fs::write(filename, lines)?;
    Ok(())
}