use std::{cmp::Ordering, env, error::Error, fs};

fn main() {
    run().unwrap_or_else(|error| {
        eprintln!("错误！错误信息：{}", error);
    })
}

fn run() -> Result<(), Box<dyn Error>> {
    let lines = get_lines()?;
    println!("{lines}");
    Ok(())
}

fn get_lines() -> Result<String, Box<dyn Error>> {
    match env::args().len().cmp(&2) {
        Ordering::Less => return Err("参数过少！".into()),
        Ordering::Greater => return Err("参数过多！".into()),
        _ => (),
    };
    let mut args = env::args();
    args.next();
    let path = args.next().unwrap();
    let content = fs::read_to_string(path)?;
    let mut toml: anki_generate::Config = toml::from_str(&content)?;
    let lines = toml
        .generate_with_line()?
        .into_iter()
        .map(|line| format!("{line}\n"))
        .collect();
    Ok(lines)
}
