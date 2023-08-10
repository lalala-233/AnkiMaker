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
        let mut toml: anki_generate::Config = toml::from_str(&content)?;
        let lines: String = toml
            .generate_with_line()?
            .into_iter()
            .map(|line| format!("{line}\n"))
            .collect();
        path.push_str(".txt");
        fs::write(path, lines)?;
    }
    Ok(())
}
