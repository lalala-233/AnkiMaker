#[cfg(feature = "gui")]
compile_error!("This bin cannot be compiled with the 'my_feature' feature enabled.");
fn main() {
    anki_maker::cli::run().unwrap_or_else(|error| {
        eprintln!("错误！错误信息：{}", error);
    })
}
