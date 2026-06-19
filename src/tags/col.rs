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
pub struct Col {
    children: Vec<Tag>,
}

impl Col {
    pub fn render(&self) -> iced::Element<crate::renderer::Message> {
        let mut column = widget::Column::new();

        for elem in self.children.iter() {
            column = column.push(elem.render());
        }
        column.into()
    }

    pub fn parse(
        node: roxmltree::Node<'_, '_>,
        path: &mut Vec<String>,
    ) -> Result<Self, ParseError> {
        
        let tags = parser::extract_children__at_least_one(node, path)?
            .into_iter().enumerate()
            .map(move |(idx,child)| {
                let p =  &mut path.clone();
                p.push(format!("[{idx}]"));
                try_parse_to_tag(child, p)
            })
            .collect::<Result<Vec<Tag>, ParseError>>()?;
        Ok(Self { children: tags })
    }
}
