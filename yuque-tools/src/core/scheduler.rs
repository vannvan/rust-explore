/*
 * Description: 调度
 * Created: 2023-08-31 09:41:21
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */

use progress_bar::*;
use regex::Regex;
use serde_json::Value;
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
        inquiry,
        log::Log,
        tools,
    },
};

pub struct Scheduler;
impl Scheduler {
    /// 知识库启动程序
    pub async fn start() -> Result<(), &'static str> {
        let cookies = tools::get_local_cookies();

        // 没有cookie缓存，进入登录环节
        if cookies.is_empty() {
            match tools::get_user_config() {
                Ok(user_config) => {
                    if cfg!(debug_assertions) {
                        println!("user_config: {:?}", user_config);
                    }
                    // 尝试默认使用配置中的账号信息
                    let account = YuqueAccount {
                        username: user_config.username.to_string(),
                        password: user_config.password.to_string(),
                    };

                    // 如果配置中缺少账户信息，就进入询问环节
                    if account.username.is_empty() || account.password.is_empty() {
                        Self::start_program(None).await;
                    } else {
                        // 填入用户的配置进入后面的流程
                        Self::start_program(Some(account)).await;
                    }
                }
                Err(_err) => {
                    if cfg!(debug_assertions) {
                        println!("没有配置文件开始问询");
                    }
                    Self::start_program(None).await;
                }
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

    /// 所有环节进入问询程序
    async fn start_program(arg: Option<YuqueAccount>) {
        let account = match arg {
            Some(config_account) => config_account,
            None => inquiry::ask_user_account(),
        };

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
            answer = inquiry::ask_user_toc_options();
            if answer.toc_range.len() > 0 {
                Self::download_task_pre_construction(answer);
            } else {
                Log::error("未选择知识库，程序退出");
                process::exit(1)
            }
        }
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

        // 导出报告文件
        let report_file_name_ref: String =
            format!("{}/导出报告.md", &GLOBAL_CONFIG.target_output_dir);

        if cfg!(debug_assertions) {
            println!("导出任务配置： {:?}", download_config);
        }

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
        let _ = f.write(&report_file_name_ref, "# 导出报告\n".to_string());

        // 文档数量
        let target_doc_count = target_doc_list.iter().len();
        // 耗时计算
        let need_time = target_doc_count * &GLOBAL_CONFIG.duration / 1000;

        Log::info(
            &format!(
                "开始执行导出任务，共 {} 篇文档，预计需要 {} 秒",
                target_doc_count, need_time
            )
            .to_string(),
        );

        init_progress_bar(target_doc_count);
        set_progress_bar_action("Loading", Color::Blue, Style::Bold);

        for item in target_doc_list {
            tokio::spawn(Self::get_and_save_content(
                item,
                download_config.clone(),
                report_file_name_ref.clone(),
            ));

            sleep(Duration::from_millis(
                GLOBAL_CONFIG.duration.try_into().unwrap(),
            ));
            inc_progress_bar();
        }

        finalize_progress_bar();
        Log::success("导出任务执行完毕!");
    }

    /// 获取内容并保存文件
    async fn get_and_save_content(
        item: TreeNone,
        download_config: MutualAnswer,
        report_file_name: String,
    ) {
        let f = File::new();

        // 本地保存路径
        let target_save_path =
            format!("{}/{}.md", GLOBAL_CONFIG.target_output_dir, &item.full_path);

        // yuque的文档地址
        let target_doc_url = format!("/{}/{}/{}", item.user, item.p_slug, item.url);

        if let Ok(content) =
            YuqueApi::get_markdown_content(&target_doc_url, download_config.line_break).await
        {
            if f.exists(&target_save_path) && download_config.skip {
                print_progress_bar_info("Skip", &item.full_path, Color::Cyan, Style::Normal);
                let _ = f.append(
                    &report_file_name,
                    format!("- 🌈 Skip {}\n", &item.full_path).to_string(),
                );
            } else {
                print_progress_bar_info("Success", &item.full_path, Color::Green, Style::Bold);
                // 写入文件
                let _ = f.write(&target_save_path, content.to_string());
                let _ = f.append(
                    &report_file_name,
                    format!("- 🌈 Success {}\n", &item.full_path).to_string(),
                );
            }
        } else {
            print_progress_bar_info("Failed", &item.full_path, Color::Red, Style::Normal);
            let _ = f.append(
                &report_file_name,
                format!("- ❌ Failed {}\n", &item.full_path).to_string(),
            );
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

    /// 团队支持库下载启动程序
    pub async fn start_grd() -> Result<(), bool> {
        Log::info("团队资源下载程序开始");

        match tools::get_user_config() {
            Ok(user_config) => {
                if cfg!(debug_assertions) {
                    println!("user_config: {:?}", user_config);
                }
                // 尝试默认使用配置中的账号信息
                let account = YuqueAccount {
                    username: user_config.username.to_string(),
                    password: user_config.password.to_string(),
                };

                match YuqueApi::login(&account.username, &account.password).await {
                    Ok(_resp) => {
                        Log::success("登录成功!");
                        // 接着就开始资源
                        Self::get_group_resource_base_info().await
                    }
                    Err(_err) => {
                        Log::error("登录失败，请检查账号信息是否正确或重试");
                        process::exit(1)
                    }
                }
            }
            Err(_err) => {
                if cfg!(debug_assertions) {
                    println!("没有配置文件开始问询");
                }
            }
        }

        Ok(())
    }

    /// 获取团队资源基础信息
    async fn get_group_resource_base_info() {
        Log::info("开始获取团队资源信息");

        match tools::get_user_config() {
            Ok(user_config) => {
                if !user_config.host.is_empty() {
                    if let Ok(source_info) = YuqueApi::get_group_resource_base_info().await {
                        Log::info("获取团队资源信息成功");

                        for item in source_info.as_array().unwrap() {
                            if item
                                .get("target")
                                .unwrap()
                                .get("settings")
                                .unwrap()
                                .get("resource_enable")
                                .unwrap()
                                .as_u64()
                                == Some(1)
                            {
                                Self::get_resource_detail_list(
                                    item.get("title").unwrap().clone(),
                                    item.get("target").unwrap().get("login").unwrap().clone(),
                                    item.get("target_id").unwrap().clone(),
                                )
                                .await
                            }
                        }
                    }
                } else {
                    Log::error("请配置团队空间域名")
                }
            }
            Err(_) => Log::error("请配置团队空间域名"),
        }
    }

    /// 获取团队资源详情
    async fn get_resource_detail_list(title: Value, base_slug: Value, resource_base_id: Value) {
        Log::info(format!("开始获取【{}】资源信息, {}", title, base_slug).as_str());
        if let Ok(source_info) =
            YuqueApi::get_group_resource_detail_list(&resource_base_id.to_string()).await
        {
            Log::info(format!("获取【{}】资源信息成功", title).as_str());
            if cfg!(debug_assertions) {
                // println!("资源详情：{:?}", source_info);
            }

            if source_info.as_array().unwrap().len() > 0 {
                for item in source_info.as_array().unwrap() {
                    if item.get("type").unwrap().as_str().eq(&Some("Resource")) {
                        Log::info(&format!(
                            "开始获取【{}】资源详情",
                            item.get("name").unwrap().to_string()
                        ));
                        let id = item.get("id").unwrap();
                        if let Ok(list) =
                            YuqueApi::get_group_resource_list(&id.to_string(), Some("")).await
                        {
                            //
                        }
                    }
                }
            }
        } else {
            Log::error(format!("获取【{}】资源信息失败", title).as_str());
        }
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
    #[tokio::test]

    async fn test_get_group_resource_base_info() {
        if let Ok(source_info) = YuqueApi::get_group_resource_base_info().await {
            Log::info("获取团队资源信息成功");

            for item in source_info.as_array().unwrap() {
                if item
                    .get("target")
                    .unwrap()
                    .get("settings")
                    .unwrap()
                    .get("resource_enable")
                    .unwrap()
                    .as_u64()
                    == Some(1)
                {
                    Scheduler::get_resource_detail_list(
                        item.get("title").unwrap().clone(),
                        item.get("target").unwrap().get("login").unwrap().clone(),
                        item.get("target_id").unwrap().clone(),
                    )
                    .await
                }
            }
        }
    }
}
