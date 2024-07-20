#[cfg(feature = "gui")]
compile_error!("This bin cannot be compiled with the 'my_feature' feature enabled.");
fn main() {
    use log::error;
    env_logger::init();
    anki_maker::cli::run().unwrap_or_else(|error| error!("{error}"))
}
