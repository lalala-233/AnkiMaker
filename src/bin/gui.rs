fn main() -> Result<(), Box<dyn std::error::Error>> {
    anki_maker::app::app("Anki Maker")?;
    Ok(())
}
