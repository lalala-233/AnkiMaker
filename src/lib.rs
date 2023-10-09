#[cfg(feature = "gui")]
pub mod app;
pub mod cli;
mod default_template;
mod info;
mod mode;
mod poem_template;
pub use info::Info;
