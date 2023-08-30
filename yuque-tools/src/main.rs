use std::process;

use crate::libs::log::Log;
use libs::command::YCommand;

mod libs;
#[tokio::main]
async fn main() {
    YCommand::new().await.unwrap_or_else(|err| {
        Log::error(err);
        process::exit(1)
    });
}
