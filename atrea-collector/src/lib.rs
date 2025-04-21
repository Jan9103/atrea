mod log_streams;
pub use log_streams::LogStreams;
mod settings;
pub use settings::Settings;
mod my_error;
pub use my_error::MyError;
pub mod cli;
#[cfg(feature = "eventsub")]
pub mod event_sub;
#[cfg(feature = "irc")]
pub mod irc;

#[cfg(all(feature = "irc", feature = "eventsub"))]
compile_error!("Can't use irc and eventsub at the same time. choose one.");
