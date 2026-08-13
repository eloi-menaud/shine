use std::default;

use roxmltree::Node;
use thiserror::Error;

pub mod tags;
pub mod types;


#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid XML: {err} :\n{}", context)]
    InvalidXML {
        err: roxmltree::Error,
        context: String,
    },

    #[error("Missing required attribute '{attribute_name}' in .{}", tag_path.join("."))]
    MissingAttribute {
        attribute_name: String,
        tag_path: Vec<String>,
    },

    #[error("Attribute '{attribute_name}' specified multiple times in .{}", tag_path.join("."))]
    DuplicatedAttribute {
        attribute_name: String,
        tag_path: Vec<String>,
    },

    #[error("Invalid attribute '{attribute_name}' in .{}", tag_path.join("."))]
    InvalidAttributeName {
        attribute_name: String,
        tag_path: Vec<String>,
    },

    #[error("Invalid value for attribute '{attribute_name}' in .{} : {reason}", tag_path.join("."))]
    InvalidAttributeValue {
        attribute_name: String,
        reason: String,
        tag_path: Vec<String>,
    },

    #[error("Missing mandatory child element in .{}", tag_path.join("."))]
    MissingChild { tag_path: Vec<String> },

    #[error("Invalid tag name '{tag_name}' in .{}", tag_path[0..tag_path.len() - 1].join("."))]
    InvalidTag {
        tag_name: String,
        tag_path: Vec<String>,
    },

    #[error("Root tag must be 'window', found '{tag_name}'")]
    InvalidRootTag { tag_name: String },

    #[error("Too many children in .{} : expected 1", tag_path.join("."))]
    MoreThanOneChild { tag_path: Vec<String> },

    #[error("Missing content for {}", .tag_path.join("."))]
    MissingContent { tag_path: Vec<String> },

    #[error("Unexpected content in .{} : found '{child_tag_name}' at index {position}", tag_path.join("."))]
    InvalidChild {
        tag_path: Vec<String>,
        child_tag_name: String,
        position: u8,
    },

}





pub trait AttrFromStr: Sized {
    fn attr_from_str(s: &str,) -> Result<Self, &str>;
}

pub fn extract_attribute<T: AttrFromStr>(node: Node, name: &str, path: Vec<String>) -> Result<Option<T>,ParseError> {
    match node.attribute(name){
        Some(attr_value) => T::attr_from_str(attr_value)
            .map(|val| Some(val))
            .map_err(|err| ParseError::InvalidAttributeValue {
                attribute_name: name.to_string(),
                reason: err.to_string(),
                tag_path: path.clone()
            }),
        None => Ok(None)
    }
}


