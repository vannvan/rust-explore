/*
 * Description: 调度
 * Created: 2023-08-31 09:41:21
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */

use inquire::PasswordDisplayMode;
use inquire::{error::InquireError, Confirm, MultiSelect, Password, Text};
use regex::Regex;
use std::process;
use std::time::Duration;
use std::{cell::RefCell, thread::sleep};

use crate::{
    core::yuque::YuqueApi,
    libs::{
        constants::{
            schema::{MutualAnswer, TreeNone, YuqueAccount},
            GLOBAL_CONFIG,
        },
        file::File,
        log::Log,
        tools,
    },
};

pub struct Scheduler;
impl Scheduler {
    pub async fn start() -> Result<(), &'static str> {
        let cookies = tools::get_local_cookies();
        // 没有cookie缓存，进入登录环节
        if cookies.is_empty() {
            match tools::get_user_config() {
                Ok(user_config) => {
                    if cfg!(debug_assertions) {
                        println!("user_config: {:?}", user_config);
                    }
                    // 默认使用配置中的账号信息
                    let mut account = YuqueAccount {
                        username: user_config.username.to_string(),
                        password: user_config.password.to_string(),
                    };
                    // 如果配置中缺少字段，就进入询问环节
                    if account.username.is_empty() || account.password.is_empty() {
                        account = Self::inquire_yuque_account();
                    }

                    match YuqueApi::login(&account.username, &account.password).await {
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
            let books_info = tools::get_cache_books_info();

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

    /// 交互式登录
    fn inquire_yuque_account() -> YuqueAccount {
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

    /// 执行询问程序
    fn handle_inquiry() {
        let mut answer = MutualAnswer {
            toc_range: vec![],
            line_break: true,
            skip: true,
        };

        if let Ok(user_config) = tools::get_user_config() {
            if cfg!(debug_assertions) {
                println!("用户配置的参数: {:?}", user_config);
            }
            answer.toc_range = user_config.toc_range;
            answer.skip = user_config.skip;
            answer.line_break = user_config.line_break
        }

        // 如果从配置传入的参数有效就不进入询问环节
        if answer.toc_range.len() > 0 {
            Self::download_task_pre_construction(answer);
        } else {
            answer = Self::inquiry_user();
            if answer.toc_range.len() > 0 {
                Self::download_task_pre_construction(answer);
            } else {
                Log::error("未选择知识库，程序退出");
                process::exit(1)
            }
        }
    }

    /// 询问并返回结果
    fn inquiry_user() -> MutualAnswer {
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

    /// 导出任务预先构造程序
    fn download_task_pre_construction(answer: MutualAnswer) {
        let f = File::new();

        println!(
            "将按以下配置进行导出：\n  知识库：{:?}\n  跳过本地：{}\n  保留换行：{}",
            answer.toc_range, answer.skip, answer.line_break
        );

        // 获取知识库，去掉二级目录
        let toc_range = tools::get_top_level_toc_from_toc_range(&answer.toc_range);

        // 树形 docs列表
        let new_nodes = Self::build_docs_nodes_for_tree(&toc_range);
        // 扁平 docs列表
        let flat_docs_list = Self::filter_valid_docs_to_flat(&new_nodes);

        // 输出两个文件
        if cfg!(debug_assertions) {
            let _ = f.write(
                "./dev/tree-doc.json",
                serde_json::to_string_pretty(&new_nodes).unwrap(),
            );

            let _ = f.write(
                "./dev/flat-doc.json",
                serde_json::to_string_pretty(&flat_docs_list).unwrap(),
            );
        }

        Self::delay_download_doc_task(answer, flat_docs_list)
    }

    /// 构造便于递归操作的node结构,将便于操作的nodes结构返回
    /// # Arguments
    /// * target_toc_range - 选中的知识库范围
    fn build_docs_nodes_for_tree(target_toc_range: &Vec<String>) -> Vec<Vec<TreeNone>> {
        let cached_toc_info = tools::get_cache_books_info();
        let f = File::new();

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
                                children: vec![],
                                name: "".to_string(),   // 文档级别没有name
                                user: "".to_string(),   // 在没递归之前是空的
                                p_slug: "".to_string(), // 在没递归之前是空的
                                uuid: child.uuid.clone(),
                                visible: child.visible,
                                full_path: child.title.to_string(),
                                parent_id: child.parent_uuid.to_string(),
                                title: child.title.to_string(),
                                child_uuid: child.child_uuid.to_string(),
                                node_type: child.node_type.to_string(), // DOC 或 TITLE
                                url: child.url.clone(),                 // 只有文档级别有
                            })
                            .collect();
                        // 这一级是知识库级别
                        Some(TreeNone {
                            parent_id: "".to_string(),
                            uuid: "".to_string(),
                            full_path: "".to_string(),
                            title: "".to_string(), // 知识库级别没有标题
                            child_uuid: "".to_string(),
                            node_type: "".to_string(),
                            url: "".to_string(),
                            visible: 1,
                            p_slug: item.slug.to_string(), // 作为文档上一级slug拼接
                            name: item.name.clone(),       // 知识库名称
                            user: item.user_login.to_string(), // 当前文档所属用户
                            children,
                        })
                    } else {
                        None
                    }
                })
                .collect();

            let new_nodes: Vec<_> = nodes
                .iter()
                .map(|node| {
                    // 这里要提前创建知识库顶级目录,makeup_tree_toc_dir是创建知识库下每一层目录
                    let target_dir = format!("{}/{}", GLOBAL_CONFIG.target_output_dir, node.name);
                    if let Err(_) = f.mkdir(target_dir.as_str()) {
                        Log::error("知识库目录创建失败")
                    }
                    Self::makeup_tree_toc_dir(
                        &node.children,
                        "",
                        node.name.to_owned(),
                        &node.user,
                        &node.p_slug,
                    )
                })
                .collect();

            new_nodes
        }
    }

    /// 定时导出任务
    /// # Arguments
    /// * download_config - 导出配置
    /// * flat_docs_list -  扁平文档列表
    fn delay_download_doc_task(download_config: MutualAnswer, flat_docs_list: Vec<TreeNone>) {
        let f = File::new();

        let mut target_doc_list = flat_docs_list.clone();

        let len = target_doc_list.iter().len();
        let need_time = len * &GLOBAL_CONFIG.duration / 1000;

        if cfg!(debug_assertions) {
            println!("导出任务配置： {:?}", download_config);
        }

        Log::info(
            &format!(
                "开始执行导出任务，共 {} 篇文档，预计需要 {} 秒",
                len, need_time
            )
            .to_string(),
        );

        // 二次过滤，因为可能只需要导出知识库下某目录的文档
        // 如果配置知识库范围中有反斜杠就认为有二级目录
        let is_have_sub_dir = download_config.toc_range.join("").contains("/");

        let target_toc_range_str = download_config.toc_range.join("|");
        if cfg!(debug_assertions) {
            println!("匹配正则：{}", target_toc_range_str)
        }

        let reg_set = Regex::new(&target_toc_range_str).unwrap();
        if is_have_sub_dir {
            target_doc_list = flat_docs_list
                .into_iter()
                .filter(|item| reg_set.is_match(&item.full_path))
                .collect::<Vec<TreeNone>>()
                .to_vec();
        }

        if cfg!(debug_assertions) {
            let _ = f.write(
                "./dev/secend_filter_doc.json",
                serde_json::to_string_pretty(&target_doc_list)
                    .unwrap()
                    .to_string(),
            );
        }

        for item in target_doc_list {
            tokio::spawn(Self::get_and_save_content(item, download_config.clone()));
            sleep(Duration::from_millis(
                GLOBAL_CONFIG.duration.try_into().unwrap(),
            ));
        }
        Log::success("导出任务执行完毕!");
    }

    /// 获取内容并保存文件
    async fn get_and_save_content(item: TreeNone, download_config: MutualAnswer) {
        let f = File::new();

        let target_path = format!("{}/{}.md", GLOBAL_CONFIG.target_output_dir, &item.full_path);

        let url = format!("/{}/{}/{}", item.user, item.p_slug, item.url);

        if let Ok(content) = YuqueApi::get_markdown_content(&url, download_config.line_break).await
        {
            if f.exists(&target_path) && download_config.skip {
                Log::info(&format!("{} 跳过", item.full_path))
            } else {
                Log::success(&format!("{} 导出成功", item.full_path));
                let _ = f.write(&target_path, content.to_string());
            }
        } else {
            Log::error(&format!("{} 导出失败", item.full_path).to_string())
        }
    }

    /// 从树形列表中拿到有效的文档列表，并以扁平结构返回
    /// # Arguments
    /// * tree - 树形列表
    fn filter_valid_docs_to_flat(tree: &Vec<Vec<TreeNone>>) -> Vec<TreeNone> {
        let list: RefCell<Vec<TreeNone>> = RefCell::new(vec![]);

        fn each(list: &RefCell<Vec<TreeNone>>, docs: &Vec<TreeNone>) {
            if !docs.is_empty() {
                docs.iter().for_each(|doc| {
                    if doc.node_type == "DOC" && doc.visible == 1 {
                        let cloned_doc = doc.clone();
                        list.borrow_mut().push(cloned_doc);
                    }

                    if !doc.children.is_empty() {
                        each(list, &doc.children);
                    }
                });
            }
        }

        tree.iter().for_each(|item| {
            item.iter().for_each(|sub_item| {
                if sub_item.node_type == "DOC" && sub_item.visible == 1 {
                    list.borrow_mut().push(sub_item.clone());
                }
                each(&list, &sub_item.children);
            });
        });

        list.take()
    }

    /// 递归创建树形目录，顺便将文档路径拼接完成，同时记录文档对应父级slug和user，用于最终download环节
    /// # Arguments
    /// * items - 递归下一级
    /// * uuid - 用于匹配下一级的uuid
    /// * prev_path - 前一层完整路径，用于下一级继续拼接
    /// * p_user - 文档的所属user
    /// * p_slug - 文档的父级slug
    fn makeup_tree_toc_dir(
        items: &Vec<TreeNone>,
        uuid: &str,
        prev_path: String,
        p_user: &str,
        p_slug: &str,
    ) -> Vec<TreeNone> {
        let f = File::new();
        items
            .iter()
            .filter(|item| item.parent_id == uuid)
            .map(|item| {
                // 替换名称中的特殊字符
                let regex = Regex::new(r#"[<>:"\/\\|?*\x00-\x1F]"#).unwrap();
                let full_path = format!("{}/{}", prev_path, regex.replace_all(&item.title, ""));

                // 目标路径
                let target_dir = format!("{}/{}", GLOBAL_CONFIG.target_output_dir, full_path);
                // 打印路径
                // if cfg!(debug_assertions) {
                //     println!("目标路径: {}", target_dir);
                // }
                if item.node_type == "TITLE" || !item.child_uuid.is_empty() {
                    if let Err(_) = f.mkdir(target_dir.as_str()) {
                        Log::error("知识库目录创建失败")
                    }
                }

                // 当前层
                let current_item = TreeNone {
                    parent_id: uuid.to_string(),
                    uuid: item.uuid.to_string(),
                    name: item.name.clone(),
                    title: item.title.clone(),
                    node_type: item.node_type.clone(),
                    child_uuid: item.child_uuid.clone(),
                    visible: item.visible.clone(),
                    url: item.url.clone(),
                    // 之后是来自上一级的信息
                    full_path: full_path.to_string(),
                    p_slug: p_slug.to_string(),
                    user: p_user.to_string(),

                    children: Self::makeup_tree_toc_dir(
                        items, &item.uuid, full_path, p_user, p_slug,
                    ),
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
    fn test_build_docs_nodes_for_tree() {
        Scheduler::build_docs_nodes_for_tree(&["test-book".to_string()].to_vec());
    }
    #[test]
    fn test_build_docs_nodes_for_tree_second_dir() {
        Scheduler::build_docs_nodes_for_tree(&["test-book/测试目录".to_string()].to_vec());
    }
    #[test]
    fn test_download_task_pre_construction() {
        let answer = MutualAnswer {
            toc_range: ["test-book".to_string()].to_vec(),
            skip: true,
            line_break: true,
        };
        Scheduler::download_task_pre_construction(answer)
    }
    #[test]
    /// 二级目录
    fn test_download_task_pre_construction_second_dir() {
        let answer = MutualAnswer {
            toc_range: ["test-book/测试目录".to_string()].to_vec(),
            skip: true,
            line_break: true,
        };
        Scheduler::download_task_pre_construction(answer)
    }
}
