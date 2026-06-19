use std::{path::PathBuf, sync::OnceLock};

use iced::{Element, Size};
use log::{debug, info, trace};
use thiserror::Error;

use crate::{
    parser::parse,
    shell_session::{Shell, ShellError},
    tags::window::Window,
};

use std::sync::Mutex;

static SHELL: OnceLock<Mutex<Shell>> = OnceLock::new();

pub struct State {
    _exec_ui_file: PathBuf,
    window: Window,
    error: Option<UiRuntimeError>,
}

#[derive(Error, Debug)]
pub enum UiRuntimeError {
    #[error("Failed to exec onclick function : {0}")]
    Onclick(#[from] ShellError),
}

#[derive(Debug, Clone)]
pub enum Message {
    Onclick(String),
}

fn view(state: &State) -> Element<Message> {
    if let Some(err) = state.error.as_ref() {
        eprintln!("\x1b[0;31mRuntime Error:\n{}\x1b[0;0m", err)
    }

    state.window.render().into()
}

fn generate(exec_ui_file: &PathBuf, shell: &mut Shell) -> Window {
    debug!("\n═══ Re-generating ui ═══");

    shell
        .exec(exec_ui_file.to_str().unwrap_or_default().to_string())
        .expect("Failed to generate ui.xml");

    let xml = std::fs::read_to_string(&shell.dist_file)
        .expect("Failed to fetch generated ui")
        .trim()
        .to_string();

    trace!("─── generated xml \n{}\n───",xml);

    let window: Window = parse(xml).map_err(|e| e.to_string()).unwrap_or_else(|err| {
        println!(
            "\n\x1b[31mError:\x1b[0m {}\nEntire content of the generated XML can be found at \x1b[4;37m{}\x1b[0;37m",
            err,
            shell.dist_file.to_string_lossy()
        );
        std::process::exit(1);
    });

    window
}

fn update(state: &mut State, message: Message) {
    let mut shell = SHELL
        .get()
        .expect("Static SHELL must be initialised")
        .lock()
        .expect("Failed to lock SHELL");

    info!("-> Event: {message:?}");
    match message {
        Message::Onclick(callback) => {
            if let Err(err) = shell.exec(callback) {
                state.error = Some(err.into());
            };
        }
    }

    state.window = generate(&state._exec_ui_file, &mut shell);
}

pub fn run_app(exec_ui_file: PathBuf) -> iced::Result {
    let mut shell = Shell::new().expect("Failed to create shell session");
    shell
        .init_env()
        .expect("Failed to initialise shell session environment");

    let initial_window = generate(&exec_ui_file, &mut shell);

    let app = iced::application(
        move || State {
            error: None,
            window: initial_window.clone(),
            _exec_ui_file: exec_ui_file.clone(),
        },
        update,
        view,
    );

    SHELL
        .set(Mutex::new(shell))
        .expect("SHELL already initialized");

    app.window(iced::window::Settings {
        fullscreen: false,
        resizable: false,
        size: Size::new(500.0, 500.0),
        max_size: Some(Size::new(500.0, 500.0)),
        ..iced::window::Settings::default()
    })
    .level(iced::window::Level::AlwaysOnTop)
    .run()
}
