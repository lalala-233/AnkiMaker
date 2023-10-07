use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    anki_maker::app::app("Anki Maker")?;
    Ok(())
}
