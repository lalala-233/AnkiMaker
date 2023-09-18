use anki_maker::app::App;
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
    for mut path in args {
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
    }
    Ok(())
}

fn app() -> eframe::Result<()> {
    let app_name = "Anki Maker";
    let app_options = eframe::NativeOptions::default();
    eframe::run_native(app_name, app_options, Box::new(|cc| Box::new(App::new(cc))))
}
