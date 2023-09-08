/*
 * Description: 命令入口
 * Created: 2023-08-31 09:41:21
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */

use crate::libs::{
    constants::{schema::UserCliConfig, GLOBAL_CONFIG},
    file::File,
    log::Log,
};

use super::scheduler::Scheduler;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "ytool")]
#[command(about = "语雀知识库CLI工具", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// 获取知识库
    #[command(arg_required_else_help = false)]
    Pull,
    /// 初始化配置
    #[command(arg_required_else_help = false)]
    Init,
    /// 清除缓存
    #[command(arg_required_else_help = false)]
    Clear,
    /// 工具更新
    #[command(arg_required_else_help = false)]
    Upgrade,
}

pub struct YCommand;

impl YCommand {
    pub async fn new() -> Result<(), &'static str> {
        let args = Cli::parse();
        match args.command {
            Commands::Pull => {
                let _ = Scheduler::start().await;
                Ok(())
            }
            Commands::Clear => {
                let _ = Self::clear_local_cache();
                Ok(())
            }
            Commands::Init => {
                let _ = Self::generate_cli_config();
                Ok(())
            }
            Commands::Upgrade => {
                println!("更新");
                Ok(())
            }
        }
    }

    /// 生成一套配置
    fn generate_cli_config() -> Result<bool, bool> {
        let user_cli_config = UserCliConfig {
            username: "".to_owned(),
            password: "".to_owned(),
            doc_range: "".to_owned(),
            skip: "".to_owned(),
        };

        // 格式化json文件
        let json_string = serde_json::to_string_pretty(&user_cli_config).unwrap();

        let f = File::new();

        match f.write(&GLOBAL_CONFIG.user_cli_config_file, json_string) {
            Ok(_) => {
                let mut success_info = String::from("配置文件已初始化，见👉 ");
                success_info.push_str(&GLOBAL_CONFIG.user_cli_config_file.to_string());
                Log::info(&success_info);
                return Ok(true);
            }
            Err(err) => {
                // if cfg!(debug_assertions) {
                println!("{}", err);
                // }
                Log::error("配置文件生成失败");
                return Err(false);
            }
        }
    }

    /// 清除本地缓存
    fn clear_local_cache() -> Result<bool, bool> {
        let f = File::new();

        match f.exists(&GLOBAL_CONFIG.meta_dir) {
            true => match f.rmdir(&GLOBAL_CONFIG.meta_dir) {
                Err(err) => {
                    Log::error("清除失败");
                    if cfg!(debug_assertions) {
                        println!("{}", err);
                    }
                    Err(false)
                }
                Ok(_) => {
                    Log::success("缓存已清除~");
                    Ok(true)
                }
            },
            false => {
                Log::warn("暂无缓存");
                Err(false)
            }
        }
    }
}
