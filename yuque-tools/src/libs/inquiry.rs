/*
 * Description: 交互式
 * Created: 2023-09-30 12:31:06
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */

use std::process;

use inquire::{Confirm, InquireError, MultiSelect, Password, PasswordDisplayMode, Text};

use super::{
    constants::schema::{MutualAnswer, YuqueAccount},
    log::Log,
    tools,
};

/// 询问用户导出知识库的选项
pub fn ask_user_toc_options() -> MutualAnswer {
    let mut answer = MutualAnswer {
        toc_range: vec![],
        skip: true,
        line_break: true,
    };

    match tools::get_cache_books_info() {
        Ok(books_info) => {
            if cfg!(debug_assertions) {
                // println!("知识库信息：{:?}", books_info);
            }

            // 询问知识库
            let mut options: Vec<&str> = vec![];

            for item in &books_info {
                options.push(&item.name)
            }

            // 选择知识库
            let books_ans: Result<Vec<&str>, InquireError> =
                MultiSelect::new("请选择知识库", options)
                    .with_help_message("空格选中/取消选中，⬆ ⬇ 键移动选择")
                    .prompt();
            // 因为choice是 Vec<&str> 类型，所以要转换一下
            match books_ans {
                Ok(choice) => answer.toc_range = choice.iter().map(|s| s.to_string()).collect(),
                Err(_) => panic!("选择出错，请重新尝试"),
            }

            // 确认是否跳过本地文件
            let skip_ans = Confirm::new("是否跳过本地文件?")
                .with_default(true)
                .prompt();

            match skip_ans {
                Ok(true) => answer.skip = true,
                Ok(false) => answer.skip = false,
                Err(_) => panic!("选择出错，请重新尝试"),
            }

            // 确认是否保留语雀换行标识
            let lb_ans = Confirm::new("是否保留语雀换行标识?")
                .with_default(true)
                .with_help_message("</br>在不同平台处理逻辑存在差异，可按需选择是否保留")
                .prompt();

            match lb_ans {
                Ok(true) => answer.line_break = true,
                Ok(false) => answer.line_break = false,
                Err(_) => panic!("选择出错，请重新尝试"),
            }
        }
        Err(_) => {
            Log::error("知识库文件读取失败,退出程序");
            process::exit(1);
        }
    }
    answer
}

/// 交互式登录
pub fn ask_user_account() -> YuqueAccount {
    let mut account = YuqueAccount {
        username: "".to_string(),
        password: "".to_string(),
    };

    let username = Text::new("yuque username:").prompt();
    match username {
        Ok(username) => account.username = username,
        Err(_) => println!("An error happened when asking for your name, try again later."),
    }

    let password = Password::new("yuque password:")
        .without_confirmation()
        .with_display_mode(PasswordDisplayMode::Masked)
        .prompt();

    match password {
        Ok(password) => account.password = password,
        Err(_) => {
            println!("An error happened when asking for your password, try again later.")
        }
    }
    account
}
