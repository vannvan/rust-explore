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
        constants::{
            schema::{MutualAnswer, TreeNone},
            GLOBAL_CONFIG,
        },
        file::File,
        log::Log,
        tools::{get_cache_books_info, get_local_cookies, get_user_config},
    },
};
use inquire::{error::InquireError, Confirm, MultiSelect};

pub struct Scheduler;
impl Scheduler {
    pub async fn start() -> Result<(), &'static str> {
        let cookies = get_local_cookies();
        // 没有cookie缓存，进入登录环节
        if cookies.is_empty() {
            match get_user_config() {
                Ok(user_config) => {
                    match YuqueApi::login(user_config).await {
                        Ok(_resp) => {
                            Log::success("登录成功!");
                            // 接着就开始获取知识库
                            if let Ok(_books_info) = YuqueApi::get_user_bookstacks().await {
                                Log::success("获取知识库成功");
                                Self::handle_inquiry()
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
            let books_info = get_cache_books_info();
            if books_info.is_ok() {
                Self::handle_inquiry()
            } else {
                if let Ok(_books_info) = YuqueApi::get_user_bookstacks().await {
                    Log::success("获取知识库成功");
                    Self::handle_inquiry()
                }
            }
        }
        Ok(())
    }

    /// 执行询问程序
    fn handle_inquiry() {
        let answer = Self::inquiry_user();
        if answer.toc_range.len() > 0 {
            Self::download_markdown_task(answer)
        } else {
            Log::error("未选择知识库，程序退出");
            process::exit(1)
        }
    }

    /// 询问
    fn inquiry_user() -> MutualAnswer {
        let mut answer = MutualAnswer {
            toc_range: vec![],
            skip: true,
            line_break: true,
        };

        match get_cache_books_info() {
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

                println!(
                    "将按以下配置进行导出：\n  知识库：{:?}\n  跳过本地：{}\n  保留换行：{}",
                    answer.toc_range, answer.skip, answer.line_break
                );
            }
            Err(_) => {
                Log::error("知识库文件读取失败,退出程序");
                process::exit(1);
            }
        }
        answer
    }

    /// 下载markdown
    fn download_markdown_task(answer: MutualAnswer) {
        if cfg!(debug_assertions) {
            println!("download_markdown_task 执行")
        }
        let MutualAnswer {
            toc_range,
            skip,
            line_break,
        } = answer;
        println!("{},{},{:?}", skip, line_break, toc_range);

        //

        Self::mkdir_for_toc_tree(toc_range)
    }
    /// 生成与知识库结构相同的树形目录
    fn mkdir_for_toc_tree(target_toc_range: Vec<String>) {
        let cached_toc_info = get_cache_books_info();
        if cached_toc_info.is_err() {
            panic!("知识库信息读取失败，程序退出");
        } else {
            let mut cached_toc_info = cached_toc_info.unwrap();

            let nodes: Vec<TreeNone> = cached_toc_info
                .iter_mut()
                .filter_map(|item| {
                    if target_toc_range.contains(&item.name) {
                        let children = item
                            .docs
                            .iter()
                            .map(|child| TreeNone {
                                parent_id: child.parent_uuid.to_string(),
                                uuid: child.uuid.clone(),
                                full_path: child.title.to_string(),
                                children: vec![],
                                name: "".to_string(), // 文档级别没有name
                                title: child.title.to_string(),
                            })
                            .collect();

                        Some(TreeNone {
                            parent_id: "".to_string(),
                            uuid: "".to_string(),
                            full_path: "".to_string(),
                            name: item.name.clone(),
                            title: "".to_string(), // 知识库级别没有标题
                            children,
                        })
                    } else {
                        None
                    }
                })
                .collect();

            for node in nodes.iter() {
                Self::mk_tree_toc_dir(&node.children, "", node.name.to_owned());
            }
        }
    }

    fn mk_tree_toc_dir(items: &Vec<TreeNone>, uuid: &str, prev_path: String) -> Vec<TreeNone> {
        let f = File::new();
        items
            .iter()
            .filter(|item| item.parent_id == uuid)
            .map(|item| {
                let full_path = format!("{}/{}", prev_path, item.title);
                if cfg!(debug_assertions) {
                    println!("目标路径: {}", full_path);
                }

                // 目标路径
                let target_dir = format!("{}/{}", GLOBAL_CONFIG.target_output_dir, full_path);

                if let Err(_) = f.mkdir(target_dir.as_str()) {
                    Log::error("知识库目录创建失败")
                }

                // 当前层
                let current_item = TreeNone {
                    parent_id: uuid.to_string(),
                    uuid: item.uuid.to_string(),
                    full_path: full_path.to_string(),
                    name: item.name.clone(),
                    title: item.title.clone(),
                    children: Self::mk_tree_toc_dir(items, &item.uuid, full_path),
                };

                current_item
            })
            .collect()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    #[test]
    fn test_mkdir_for_toc_tree() {
        Scheduler::mkdir_for_toc_tree(["test-book".to_string()].to_vec())
    }
}
