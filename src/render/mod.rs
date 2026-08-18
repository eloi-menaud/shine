use std::{path::PathBuf, process::exit, str::ParseBoolError};

use iced::{Task, widget::{button, column}};
use log::{debug, info};
use roxmltree::{Document, Node};

use crate::{Cli, parser::{build_xml_error, tags::{Tag, col::Col, void, window::Window}}, shell_session::Shell};


#[derive(Debug,Clone)]
pub enum Message {
    Render,
    Callback(String,String)
}


pub struct State {
    pub builder_path: PathBuf,
    pub ui: Tag,
    pub title: String,
    pub shell_session: Shell
}
impl State {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Render => {
                self.build();
            },
            Message::Callback(name,action)=>{
                info!("> callback {name}");
                self.shell_session.exec(action).unwrap();
                debug!("Building SXML schema");
                self.build();
            }
        }
    }

    pub fn view(&self) -> iced::widget::Container<Message>{
        info!("Rendering UI");
        iced::widget::container(self.ui.render()).into()
    }

    pub fn build(&mut self){
        info!("\nComputing SXML schema");
        self.shell_session.exec(format!("./{}",self.builder_path.to_string_lossy().to_string())).unwrap();
        let doc = std::fs::read_to_string(&self.shell_session.dist_file).unwrap();
        info!("═══ Computed XML ═══\n{}\n════════",doc.trim());
        info!("Parsing dist XML file to SXML schema");
        let xml = Document::parse(&doc).unwrap_or_else(|err| {
            let e = build_xml_error(&doc,err.pos(),&err.to_string(),"");
            eprintln!("{e}");
            std::process::exit(1);
        });
        let root = xml.root_element();
        info!("Parsing SXML schema to UI");
        let window = Window::from_node_ctx(root, &mut Vec::new());
        let window = match window{
            Ok(w) => w,
            Err(err) => {
                let pos = xml.text_pos_at(err.get_range().start);
                let e = build_xml_error(&doc,pos,&err.to_string(),&format!("\nError path : {}",err.get_path()));
                eprintln!("{e}");
                std::process::exit(1);
            },
        };
        self.ui = window.child;
        self.title = window.title;
    }
    
    pub fn new(builder_path: PathBuf) -> Self {
        State{
            builder_path: builder_path,
            title: String::new(),
            ui: Tag::Void(void::Void{}),
            shell_session: {
                let mut s = Shell::new().unwrap();
                s.init_env().unwrap();
                s
            }
        }
    }


}

