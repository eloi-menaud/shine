use iced::{
    Alignment::Center,
    Color,
    keyboard::key,
    widget::{
        self, button,
        text::{self, Style},
    },
};
use serde::{Deserialize, Serialize};

use crate::{
    parser::{self, ParseError},
    tags::Tag,
    utils::hex_color::HexColor,
};

#[derive(Debug, Deserialize, Clone)]
pub struct T {
    /// text to display
    #[serde(rename = "$value")]
    content: String,
}

impl T {
    pub fn render(&self) -> iced::Element<crate::renderer::Message> {
        widget::text(&self.content).into()
    }

    pub fn parse(
        node: roxmltree::Node<'_, '_>,
        path: &mut Vec<String>,
    ) -> Result<Self, ParseError> {
        
        Ok(Self {
            content: parser::extract_text__only_text(node, path)?,
        })
    }
}
