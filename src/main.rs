use env_logger::Env;
use log::error;
fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();
    #[cfg(not(feature = "gui"))]
    anki_maker::cli::run().unwrap_or_else(|error| error!("{error}"));
    #[cfg(feature = "gui")]
    anki_maker::app::app("Anki Maker").unwrap_or_else(|error| error!("{error}"));
}
