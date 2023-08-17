extern crate commander;
use commander::Commander;
use owo_colors::colors::*;
use owo_colors::OwoColorize;
// use spinners_rs::{Spinner, Spinners};
// use std::{thread, time::Duration};

use inquire::{error::InquireError, Select, Text};

fn main() {
    let command = Commander::new()
        .version(&env!("CARGO_PKG_VERSION").to_string())
        .usage("test")
        .usage_desc("Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.")
        .option_list(
            "-l, --list [value]",
            "list",
            Some(vec!["a".to_string(), "b".to_string(), "c".to_string()]),
        )
        .option_int("--enum [value]", "enum", None)
        .option_int("-d, --debug [value]", "debug", Some(123))
        .option_str(
            "-c, --copy [value]",
            "copy content",
            Some("source".to_string()),
        )
        .option("-r", "enable recursive", None)
        .parse_env_or_exit();
    if let Some(s) = command.get_str("c") {
        println!("执行到 c = {}", s);
    }

    println!("My number is {:#x}!", 10.green());
    // Background colors
    println!("My number is not {}!", 4.on_red());

    let name = Text::new("输入名称?").prompt();

    match name {
        Ok(name) => println!("Hello {}", name),
        Err(_) => println!("输入出错"),
    }

    let options: Vec<&str> = vec!["吃饭", "睡觉", "打豆豆"];

    let ans: Result<&str, InquireError> = Select::new("选择 select?", options).prompt();

    match ans {
        Ok(choice) => println!("你选择的 {}", choice),
        Err(_) => println!("select出错"),
    }

    // let mut sp = Spinner::new(Spinners::Arrow, "Doing Some Things...");

    // sp.start();

    // thread::sleep(Duration::from_secs(3));

    println!(
        "\n{}",
        String::from("下载成功！").fg::<White>().bg::<Green>()
    );
}
