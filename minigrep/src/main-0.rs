use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("参数错误!")
    }
    let config = parse_config(&args);

    println!("查询的参数 {}", config.query);
    println!("目标文件 {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("文件内容:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();
    Config { query, file_path }
}
