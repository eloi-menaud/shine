use std::{default, ops::Range};

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

    // #[error("Missing required attribute '{attribute_name}'")]
    // MissingAttribute {
    //     attribute_name: String,
    //     tag_path: Vec<String>,
    //     range: Range<usize>
    // },

    #[error("Attribute '{attribute_name}' specified multiple times")]
    DuplicatedAttribute {
        attribute_name: String,
        tag_path: Vec<String>,
        range: Range<usize>
    },

    #[error("Invalid attribute '{attribute_name}'")]
    InvalidAttributeName {
        attribute_name: String,
        tag_path: Vec<String>,
        range: Range<usize>
    },

    #[error("Invalid value for attribute '{attribute_name}' : {reason}")]
    InvalidAttributeValue {
        attribute_name: String,
        reason: String,
        tag_path: Vec<String>,
        range: Range<usize>
    },

    // #[error("Missing mandatory child element")]
    // MissingChild { tag_path: Vec<String>, range: Range<usize> },

    #[error("Invalid tag name '{tag_name}'")]
    InvalidTag {
        tag_name: String,
        tag_path: Vec<String>,
        range: Range<usize>
    },

    #[error("Root tag must be 'window', found '{tag_name}'")]
    InvalidRootTag { tag_name: String, range: Range<usize> },

    #[error("Too many children : expected 1")]
    MoreThanOneChild { tag_path: Vec<String>, range: Range<usize> },

    // #[error("Missing content for {}", .tag_path.join("."))]
    // MissingContent { tag_path: Vec<String>, range: Range<usize> },

    // #[error("Unexpected content : found '{child_tag_name}' at index {position}")]
    // InvalidChild {
    //     tag_path: Vec<String>,
    //     child_tag_name: String,
    //     position: u8,
    //     range: Range<usize>
    // },

}
impl ParseError {
    pub fn get_range(&self) -> Range<usize>{
        match self {
            ParseError::InvalidXML { err, context } => Range::default(),
            ParseError::DuplicatedAttribute { attribute_name, tag_path, range } => range.clone(),
            ParseError::InvalidAttributeName { attribute_name, tag_path, range } => range.clone(),
            ParseError::InvalidAttributeValue { attribute_name, reason, tag_path, range } => range.clone(),
            ParseError::InvalidTag { tag_name, tag_path, range } => range.clone(),
            ParseError::InvalidRootTag { tag_name, range } => range.clone(),
            ParseError::MoreThanOneChild { tag_path, range } => range.clone(),
        }
    }
    pub fn get_path(&self) -> String{
        match self {
            ParseError::InvalidXML { err, context } => String::new(),
            ParseError::DuplicatedAttribute { attribute_name, tag_path, range } => format!("{} attribut '{attribute_name}'", tag_path.join(".")),
            ParseError::InvalidAttributeName { attribute_name, tag_path, range } => format!("{} attribut '{attribute_name}'", tag_path.join(".")),
            ParseError::InvalidAttributeValue { attribute_name, reason, tag_path, range } => format!("{} attribut '{attribute_name}'", tag_path.join(".")),
            ParseError::InvalidTag { tag_name, tag_path, range } => format!("{}.{tag_name}", tag_path.join(".")),
            ParseError::InvalidRootTag { tag_name, range } => "{tag_name} (root)".to_string(),
            ParseError::MoreThanOneChild { tag_path, range } => tag_path.join("."),
        }
    }
}






pub fn build_xml_error(
    raw_xml: &str,
    pos: roxmltree::TextPos,
    reason: &str,
    info: &str,
) -> String {
    let lines: Vec<&str> = raw_xml.lines().collect();
    let error_line_idx = (pos.row as usize).saturating_sub(1);
    let start_idx = error_line_idx.saturating_sub(3);

    let context_lines: String = lines
        .iter()
        .enumerate()
        .skip(start_idx)
        .take(error_line_idx - start_idx + 1)
        .map(|(i, line)| format!("{:>4} | {}\n", i + 1, line))
        .collect();
    
    let cursor = format!("{:>4}   {}^", "", " ".repeat((pos.col as usize).saturating_sub(1)));

    format!("{context_lines}\x1b[31m{cursor} {reason}\x1b[0m\n{info}")
}
