use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("查询的参数 {}", query);
    println!("目标文件 {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("文件内容:\n{contents}");
}
