// 优化版本
extern crate clap;

use clap::{App, Arg}; // 命令行
use std::fs::File;
use std::io::{Read, Write}; // 注意这里指定权限
use std::path::Path;
use std::process;

fn main() {
    let _matches = App::new("kt")
        .version("1.0.0")
        .author("vannvan")
        .about("一个类似cat命令的rust命令行工具")
        .arg(
            Arg::with_name("FILE")
                .help("File to print.")
                .empty_values(false),
        )
        .get_matches();

    if let Some(file) = _matches.value_of("FILE") {
        println!("😀目标文件: {}", file);
        if Path::new(&file).exists() {
            match File::open(file) {
                Ok(mut f) => {
                    let mut data = String::new();
                    f.read_to_string(&mut data)
                        .expect("[kt Error] 文件读取失败");

                    let stdout = std::io::stdout(); // 获取全局 stdout 对象
                    let mut handle = std::io::BufWriter::new(stdout); // 可选项：将 handle 包装在缓冲区中
                    match writeln!(&mut handle, "{}", data) {
                        Ok(_res) => {}
                        Err(err) => {
                            eprintln!("[kt Error] 内容输出错误. {:?}", err);
                            process::exit(1)
                        }
                    }
                }
                Err(err) => {
                    eprintln!("[ht Error] 文件读取失败. {:?}", err);
                    process::exit(1)
                }
            }
        } else {
            eprintln!("[kt Error] 文件不存在");
            process::exit(1)
        }
    }
}
