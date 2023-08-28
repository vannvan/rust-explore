use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // unwrap_or_else 是定义在 Result<T,E> 上的常用方法，如果 Result 是 Ok，那该方法就类似 unwrap：
    // 返回 Ok 内部的值；如果是 Err，就调用闭包中的自定义代码对错误进行进一步处理
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("发生错误: {err}");
        process::exit(1)
    });

    println!("查询的参数 {}", config.query);
    println!("目标文件 {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("程序错误： {e}");
        process::exit(1)
    }
}
