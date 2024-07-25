use log::error;
fn main() {
    env_logger::init();
    #[cfg(not(feature = "gui"))]
    anki_maker::cli::run().unwrap_or_else(|error| error!("{error}."));
    #[cfg(feature = "gui")]
    anki_maker::app::app("Anki Maker").unwrap_or_else(|error| error!("{error}."));
}
