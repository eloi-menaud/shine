use std::vec;

use iced::{
    Alignment::Center,
    Background, Color,
    widget::{button, column, text},
};
use serde::Deserialize;

use crate::{
    parser::{self, ParseError, extract_attributes, try_parse_to_tag},
    renderer::Message,
    tags::Tag,
    utils::hex_color::HexColor,
};

#[derive(Clone)]
pub struct Btn {
    child: Box<Tag>,
    onclick: String,
}

impl Btn {
    pub fn render(&self) -> iced::Element<crate::renderer::Message> {
        button(self.child.render())
            .on_press(Message::Onclick(self.onclick.clone()))
            .into()
    }

    pub fn parse(
        node: roxmltree::Node<'_, '_>,
        path: &mut Vec<String>,
    ) -> Result<Self, ParseError> {
        
        let attributes = extract_attributes(node, vec!["onclick"].into_iter().collect(), path)?;

        let onclick = attributes.get("onclick").cloned().unwrap_or_default();

        Ok(Self {
            child: Box::new(try_parse_to_tag(
                parser::extract_children__exactly_one(node, path)?,
                path,
            )?),
            onclick: onclick,
        })
    }
}
