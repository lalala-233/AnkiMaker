use std::{
    error::Error,
    fs::{self, File},
    io::{self, Write, stdin},
};

fn main() {
    run().unwrap_or_else(|error| {
        eprintln!("错误！错误信息：{}", error);
    })
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut path = String::new();
    stdin().read_line(&mut path)?;
    let path = path.trim();

    // let path = "test.toml".to_string();
    let content = fs::read_to_string(path)?;

    let mut toml: anki_generate::Config = toml::from_str(&content)?;
    for i in toml.generate_with_line() {
        println!("{i}");
    }
    // Read the origin.toml
    // parse the header and the content
    // Write the result.txt

    Ok(())
}
fn write(content: Vec<String>) -> io::Result<()> {
    let mut file = File::create("result.txt")?;
    for line in content {
        file.write_all(line.as_bytes())?;
    }
    Ok(())
}
