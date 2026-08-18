use std::{path::PathBuf};

use clap::Parser;

use crate::render::State;
use std::io::Write;


mod parser;
mod render;
mod shell_session;




#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Cli {

    /// Path to the builder executable file 
    #[arg(value_name = "PATH")]
    path: PathBuf,

    // /// Verbose mode
    // #[arg(short,long)]
    // verbose: bool,

    // /// Tracing mode
    // #[arg(long)]
    // tracing: bool,
}




fn main() -> iced::Result {
    let cli = Cli::parse();


    
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Off)
        .filter_module(env!("CARGO_PKG_NAME"), log::LevelFilter::Info)
        .format(|buf, record| {
            writeln!(buf, "{}", record.args())
        })
        .init();

    let path = cli.path;

    iced::application(
            move || {
            let mut state = State::new(path.clone());
            state.build();
            state
        },
        State::update,
        State::view
    )
    .title(move |s: &State| s.title.clone())
    .run()


}

 