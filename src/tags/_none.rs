use iced::widget::text;
use serde::Deserialize;

use crate::parser::{ParseError, extract_text__only_text};



#[derive(Debug,Default, Deserialize,Clone)]
pub struct None{}
 

impl None {
    pub fn render(&self) -> iced::Element<crate::renderer::Message> {
        text(String::new()).into()
    }

    pub fn parse(node: roxmltree::Node<'_, '_>, path: &mut Vec<String>) -> Result<Self,ParseError>{
        let text = extract_text__only_text(node, path)?;
        Ok(Self{
            content: parser::extract_text__only_text(node, path)?

        })
    }
}
