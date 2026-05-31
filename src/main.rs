use clap::{Parser, Subcommand};

use crate::{dialog::info::InfoArgs, theme::parsing::Theme};

mod dialog;
mod theme;
mod types;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Info(InfoArgs)
}



fn main() {
    let cli = Cli::parse();
    
    Theme::default().init_static();
    
    match cli.command {
        Commands::Info(args) => dialog::info::render(args).unwrap(),
    }
}