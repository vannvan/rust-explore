use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // 参数校验
        // 第一个参数是程序名，不用它
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("没有参数"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("没有目标文件"),
        };

        // 接收环境变量
        //  可以这么用： IGNORE_CASE=1 cargo run -- to test.txt
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 使用 ? 和特征对象来返回错误
    let contents = fs::read_to_string(config.file_path)?;

    // println!("文件内容:\n{contents}");

    // 支持环境变量
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
        // println!("{line}");
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }

    // results

    // 使用迭代器
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// 对大小写不敏感
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let query = query.to_lowercase();
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }

    // results
    // 使用迭代器
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";

        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        // 注意这里是大写
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
