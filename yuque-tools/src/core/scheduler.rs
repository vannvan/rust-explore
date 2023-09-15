/*
 * Description: 调度
 * Created: 2023-08-31 09:41:21
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */

use std::process;

use crate::{
    core::yuque::YuqueApi,
    libs::{
        constants::schema::MutualAnswer,
        log::Log,
        tools::{get_cache_books_info, get_local_cookies, get_user_config},
    },
};
use inquire::{error::InquireError, Confirm, MultiSelect};

pub struct Scheduler;
impl Scheduler {
    pub async fn start() -> Result<(), &'static str> {
        let cookies = get_local_cookies();

        if cookies.is_empty() {
            match get_user_config() {
                Ok(user_config) => {
                    match YuqueApi::login(user_config).await {
                        Ok(_resp) => {
                            Log::success("登录成功!");
                            // 接着就开始获取知识库
                            if let Ok(_books_info) = YuqueApi::get_user_bookstacks().await {
                                Log::success("获取知识库成功")
                                // TODO 进入询问环节
                            }
                        }
                        Err(_err) => {
                            Log::error("登录失败，请检查账号信息是否正确或重试");
                            process::exit(1)
                        }
                    }
                }
                Err(err) => Log::error(err),
            }
        } else {
            // 有cookie，不走登录
            // println!("cookies-> {}", cookies);
            // TODO
            // 先去获取本地缓存的知识库，如果在半小时之内，就不用重复获取了
            match get_cache_books_info() {
                Ok(_books_info) => {
                    let sss = Self::inquiry_user();
                    println!("{:?}", sss.books)
                }
                Err(_) => {
                    if let Ok(_books_info) = YuqueApi::get_user_bookstacks().await {
                        Log::success("获取知识库成功");
                        let _ = Self::inquiry_user();
                        // print!("{:?}", serde_json::from_value(_books_info).unwrap())
                    }
                }
            }
        }
        Ok(())
    }

    /// 询问
    fn inquiry_user() -> MutualAnswer<&'static str> {
        let mut answer = MutualAnswer {
            books: [].to_vec(),
            skip: true,
            line_break: true,
        };

        match get_cache_books_info() {
            Ok(mut books_info) => {
                if cfg!(debug_assertions) {
                    println!("知识库信息：{:?}", books_info);
                }

                // 询问知识库
                let mut options: Vec<&str> = vec![];

                for item in &mut books_info {
                    options.push(&item.name)
                }

                // 选择知识库
                let books_ans: Result<Vec<&str>, InquireError> =
                    MultiSelect::new("请选择知识库", options)
                        .with_help_message("空格选中，⬆ ⬇ 键移动选择")
                        .prompt();

                match books_ans {
                    Ok(choice) => answer.books = choice.clone(),
                    Err(_) => println!("选择出错，请重新尝试"),
                }

                // 确认是否跳过本地文件
                let skip_ans = Confirm::new("是否跳过本地文件?")
                    .with_default(true)
                    .prompt();

                match skip_ans {
                    Ok(true) => answer.skip = true,
                    Ok(false) => answer.skip = false,
                    Err(_) => println!("选择出错，请重新尝试"),
                }

                // 确认是否保留语雀换行标识
                let lb_ans = Confirm::new("是否保留语雀换行标识?")
                    .with_default(true)
                    .with_help_message("</br>在不同平台处理逻辑存在差异，可按需选择是否保留")
                    .prompt();

                match lb_ans {
                    Ok(true) => answer.line_break = true,
                    Ok(false) => answer.line_break = false,
                    Err(_) => println!("选择出错，请重新尝试"),
                }

                println!(
                    "将按以下配置进行导出：\n  知识库：{:?}\n  跳过本地：{}\n  保留换行：{}",
                    answer.books, answer.skip, answer.line_break
                );

                // answer
            }
            Err(_) => {
                Log::error("知识库文件读取失败,退出程序");
                // answer
                process::exit(1);
            }
        }
        answer
        // let book_info = serde_json::
    }
}
