/*
 * Description: 命令入口
 * Created: 2023-08-31 09:41:21
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */

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
                println!("清除缓存哈哈哈");
                Ok(())
            }
            Commands::Init => {
                println!("初始化配置");
                Ok(())
            }
            Commands::Upgrade => {
                println!("更新");
                Ok(())
            }
        }
    }
}
