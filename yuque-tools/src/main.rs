use std::process;

use crate::core::command::YCommand;
use crate::libs::log::Log;
mod core;
mod libs;
#[tokio::main]
async fn main() {
    YCommand::new().await.unwrap_or_else(|err| {
        Log::error(err);
        process::exit(1)
    });
}
