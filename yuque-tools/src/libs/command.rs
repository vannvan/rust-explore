use crate::libs::log::Log;
use crate::libs::tools::Tools;
use clap::{Parser, Subcommand};
use std::process;

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
                // Log::success("成功的消息!");
                // Log::error("成功的消息!");
                // Log::warn("警告的消息!");
                // Log::info("普通的消息!");

                match Tools::get_user_config() {
                    Ok(user_config) => {
                        if let Ok(_resp) = Tools::login_yuque_and_save_cookies(user_config).await {
                            Log::success("登录成功!")
                        } else {
                            Log::error("登录失败");
                            process::exit(1)
                        }
                    }
                    Err(err) => Log::error(err),
                }

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
