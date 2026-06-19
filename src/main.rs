use std::path::{Component, PathBuf};
use log::LevelFilter;
use serde::Deserialize;
use std::io::Write;

mod parser;
mod renderer;
mod shell_session;
mod tags;
mod utils;

use crate::{
    shell_session::Shell,
    tags::{Tag, window::Window},
};

use clap::{Parser, Subcommand, ValueEnum};

#[derive(clap::Parser)]
#[command(name = "shine", version)]
#[command(group(
    clap::ArgGroup::new("log_level")
        .args(["verbose", "tracing"])
        .required(false)
))]
struct Cli {
    /// Path of the executable file that generate the UI xml
    #[arg(long, default_value = "./ui.sh", value_name("PATH"))]
    ui_exec_file: PathBuf,

    /// Verbose mode
    #[arg(short,long)]
    verbose: bool,

    /// Tracing mode
    #[arg(long)]
    tracing: bool,
}

fn main() -> iced::Result {
    let cli = Cli::parse();


    let log_level = if cli.tracing {
        LevelFilter::Trace
    } else if cli.verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    
    env_logger::Builder::new()
        .filter_level(LevelFilter::Off)
        .filter_module(env!("CARGO_PKG_NAME"), log_level)
        .format(|buf, record| {
            writeln!(buf, "{}", record.args())
        })
        .init();
    
    renderer::run_app(cli.ui_exec_file)
}
