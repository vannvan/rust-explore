use crate::libs::log::Log;

use std::process;

use libs::command::YCommand;

mod libs;

fn main() {
    YCommand::new().unwrap_or_else(|err| {
        Log::error(err);
        process::exit(1)
    });
}
