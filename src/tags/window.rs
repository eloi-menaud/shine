use iced::widget::column;
use serde::Deserialize;

use crate::{
    parser::{self, ParseError, try_parse_to_tag},
    tags::Tag,
};

#[derive(Clone)]
pub struct Window {
    pub child: Tag,
}

impl Window {
    pub fn render(&self) -> iced::Element<crate::renderer::Message> {
        self.child.render()
    }

    pub fn parse(
        node: roxmltree::Node<'_, '_>,
        path: &mut Vec<String>,
    ) -> Result<Window, ParseError> {
        let child = parser::extract_children__exactly_one(node, path)?;
        Ok(Window {
            child: try_parse_to_tag(child, path)?,
        })
    }
}
