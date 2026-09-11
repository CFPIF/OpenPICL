use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

pub mod commands;

/// Nebula CLI 工具
#[derive(Parser)]
#[command(name = "nebula")]
#[command(about = "Nebula CLI 工具", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 模板管理
    Template {
        #[command(subcommand)]
        action: TemplateAction,
    },
}

#[derive(Subcommand)]
enum TemplateAction {
    /// 初始化新项目
    Init {
        /// 模板名称
        template: String,
        /// 项目名称
        #[arg(short, long)]
        name: String,
        /// 输出父目录，项目根目录由模板路径决定
        #[arg(short, long, default_value = ".")]
        output: PathBuf,
        /// 设置模板变量，格式为 key=value, 可重复使用
        #[arg(long = "var", value_parser = commands::parse_key_value)]
        vars: Vec<(String, String)>,
        /// 作者，等价于 --var author=<value>
        #[arg(long)]
        author: Option<String>,
        /// 非交互模式，缺失变量且无默认值时报错
        #[arg(long)]
        non_interactive: bool,
        /// 覆盖已存在文件
        #[arg(long)]
        force: bool,
        /// 只展示将创建的路径，不写入文件
        #[arg(long)]
        dry_run: bool,
    },
    /// 列出所有可用模板
    List,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Template { action } => match action {
            TemplateAction::Init {
                template,
                name,
                output,
                vars,
                author,
                non_interactive,
                force,
                dry_run,
            } => commands::init_template(
                template,
                name,
                output,
                vars,
                author,
                non_interactive,
                force,
                dry_run,
            ),
            TemplateAction::List => commands::list_templates(),
        },
    }
}
