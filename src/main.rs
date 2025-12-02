// main.rs: CLI entrypoint that delegates to command modules.

use anyhow::Result;
use clap::Parser;

use keyboard_recorder::{
    clt::{Cli, Command as CliCommand},
    commands::{
        clear::run as clear_command,
        daemon::{start_daemon, stop_daemon},
        query::run as query_command,
        recorder::run as run_recorder,
    },
};

// 主函数：解析命令行并分发到 Start/Stop/Daemon。
// Start 在 Windows 上以后台进程方式启动；Stop 停止后台进程；Daemon 在前台运行记录器或作为后台进程的实际执行入口。
fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        CliCommand::Start => start_daemon(),
        CliCommand::Stop => stop_daemon(),
        CliCommand::Daemon => run_recorder(),
        CliCommand::Query(args) => query_command(args),
        CliCommand::Clear(args) => clear_command(args),
    }
}
