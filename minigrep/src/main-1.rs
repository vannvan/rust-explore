use std::env;
use std::fs;
use std::process;

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

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("文件内容:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        // 参数校验

        if args.len() < 3 {
            return Err("参数错误");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
