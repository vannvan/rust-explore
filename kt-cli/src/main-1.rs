// 初始版本
extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process;

fn main() {
    let _matches = App::new("kt")
        .version("1.0.0")
        .author("vannvan")
        .arg(
            Arg::with_name("FILE")
                .help("File to print.")
                .empty_values(false),
        )
        .get_matches();

    if let Some(file) = _matches.value_of("FILE") {
        println!("目标文件: {}", file);
        if Path::new(&file).exists() {
            // println!("文件存在!");
            let mut f = File::open(file).expect("[kt Error] 文件不存在");
            let mut data = String::new();
            f.read_to_string(&mut data)
                .expect("[kt Error] 文件读取失败");
            println!("{}", data)
        } else {
            eprintln!("[kt Error] 文件不存在");
            process::exit(1)
        }
    }
}
