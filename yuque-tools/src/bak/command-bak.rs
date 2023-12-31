use crate::libs::log::Log;
use crate::libs::tools::Tools;
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
    #[command(arg_required_else_help = true)]
    Pull {
        /// 语雀帐号
        username: String,
        /// 语雀密码
        password: String,
        /// 知识库范围，如 XXX,YYY 或 XXX/XXX子目录,YYY
        toc_range: Option<String>,
        /// 是否覆盖，默认跳过
        skip: Option<String>,
        /// 是否保持换行，默认保持换行
        lb: Option<String>,
    },
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
    pub fn new() -> Result<(), &'static str> {
        let args = Cli::parse();
        match args.command {
            Commands::Pull {
                username,
                password,
                toc_range,
                skip,
                lb,
            } => {
                let skip = skip.unwrap();
                let lb = lb.unwrap();
                let toc_range = toc_range.unwrap();

                Log::success("成功的消息!");
                Log::error("成功的消息!");
                Log::warn("警告的消息!");
                Log::info("普通的消息!");

                println!("用户信息 {username},{password},{toc_range},{skip},{lb}");
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
