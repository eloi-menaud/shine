use iced::{
    Alignment::Center,
    widget::{self, button, text},
};
use serde::Deserialize;

use crate::{
    parser::{self, ParseError, try_parse_to_tag},
    tags::Tag,
};


#[derive(Clone)]
pub struct Row {
    children: Vec<Tag>,
}

impl Row {
    pub fn render(&self) -> iced::Element<crate::renderer::Message> {
        let mut row = widget::Row::new();

        for elem in self.children.iter() {
            row = row.push(elem.render());
        }
        row.into()
    }

    pub fn parse(
        node: roxmltree::Node<'_, '_>,
        path: &mut Vec<String>,
    ) -> Result<Self, ParseError> {
        let tags = parser::extract_children__at_least_one(node, path)?
            .into_iter().enumerate()
            .map(move |(idx,child)| {
                let p =  &mut path.clone();
                p.push("{idx}".to_string());
                try_parse_to_tag(child, p)
            })
            .collect::<Result<Vec<Tag>, ParseError>>()?;
        Ok(Self { children: tags })
    }
}
