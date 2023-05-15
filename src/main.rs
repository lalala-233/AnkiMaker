use std::{
    error::Error,
    fs::File,
    io::{stdin, BufRead, BufReader},
};

fn main() {
    run().unwrap_or_else(|error| {
        eprintln!("错误！错误信息：{}", error);
    })
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut path = String::new();
    stdin().read_line(&mut path)?;
    let file = File::open(path.trim())?;
    let reader = BufReader::new(&file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
