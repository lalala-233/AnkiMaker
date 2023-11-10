use std::error::Error;
#[cfg(feature = "gui")]
fn main() -> Result<(), Box<dyn Error>> {
    anki_maker::app::app("Anki Maker")?;
    Ok(())
}
