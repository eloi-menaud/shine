


use iced::{Element, alignment::{Horizontal, Vertical}, widget::text};
use roxmltree::Node;
use crate::{define_struct_with_names, parser::{ParseError, tags::{Tag::{self, Void}, check_duplicate_attributes, extract_attribute, extract_children, void}, types::Pixel}, render::Message};







define_struct_with_names!{
    #[derive(Debug)]
    pub struct Window {
        pub child: Tag,
        pub title: String
    }
}


impl Window {
    pub fn from_node_ctx<'a, 'input>(node: Node<'a, 'input>, path: &mut Vec<String>) -> Result<Self, ParseError> {
        if node.tag_name().name() != "window" {
            return Err(ParseError::InvalidRootTag { tag_name: node.tag_name().name().to_string(), range: node.range() })
        }
        path.push(String::from("window"));
        
        // check_duplicate_attributes(node.attributes()).map_err(|e| ParseError::DuplicatedAttribute { attribute_name: e.to_string(), tag_path: path.clone(), range: node.range() })?;
        if let Some(invalid_attr) = node.attributes().find(|attr| !Self::ALL_FIELDS.contains(&attr.name())){
            return Err(ParseError::InvalidAttributeName { attribute_name: invalid_attr.name().to_string(), tag_path: path.clone(), range: node.range() });
        }

        let children = extract_children(&node).into_iter().map(|node| Tag::from_node_ctx(node, path) ).collect::<Result<Vec<Tag>,ParseError>>()?;
        if children.len() > 1 {
            return Err(ParseError::MoreThanOneChild{
                tag_path: path.clone(),
                range: node.range(),
            })
        }
        
        Ok(Self{
            child: extract_children(&node).into_iter().map(|node| Tag::from_node_ctx(node, path) ).collect::<Result<Vec<Tag>,ParseError>>()?.into_iter().next().unwrap_or(Void(void::Void{})),
            title: node.attribute(Self::title).unwrap_or("Shine app").to_string()
        })
    }

    pub fn render<'a>(&self) -> Element<'a, Message> {
        iced::widget::container(self.child.render())
            .into()
    }

}


