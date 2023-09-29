use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    app::app("Anki Maker")?;
    Ok(())
}
