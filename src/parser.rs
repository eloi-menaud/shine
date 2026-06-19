use crate::tags::{self, Tag, window::Window};
use roxmltree::{Document, Node, TextPos};
use std::collections::{HashMap, HashSet};
use thiserror::Error;

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
    AttributeSpecifiedMultipleTime {
        attribute_name: String,
        tag_path: Vec<String>,
    },

    #[error("Invalid attribute '{attribute_name}' in .{}", tag_path.join("."))]
    InvalidAttribute {
        attribute_name: String,
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

    #[error("Unexpected content in .{} : found '{type_name}' at index {position}", tag_path.join("."))]
    InvalidContent {
        tag_path: Vec<String>,
        type_name: String,
        position: u8,
    },
}

/// Parse raw xml to Window tag
pub fn parse(raw_xml: String) -> Result<Window, ParseError> {
    let doc = Document::parse(&raw_xml).map_err(|xml_err| {
        let pos = xml_err.pos();
        let lines: Vec<&str> = raw_xml.lines().collect();

        let target_idx = pos.row.saturating_sub(1);

        let start = target_idx.saturating_sub(2) as usize;
        let end = (target_idx + 3).min(lines.len() as u32) as usize;

        let mut context = String::new();

        context.push_str("     ╎\n");
        for (i, line_content) in lines[start..end].iter().enumerate() {
            let current_line_num = start + i + 1;

            context.push_str(&format!("{:>4} | {}\n", current_line_num, line_content));

            if current_line_num == (pos.row) as usize {
                let padding = " ".repeat((pos.col as usize).saturating_sub(1));
                context.push_str(&format!("{:>4}   {}\x1b[31m{}\x1b[0m\n", "", padding, "^"));
            }
        }

        context.push_str("     ╎\n");
        ParseError::InvalidXML {
            err: xml_err,
            context: context.trim_end().to_string(),
        }
    })?;
    try_parse_window(doc.root_element())
}

/// Extracts node attributes into a HashMap,
/// returning an error if not allowed or duplicate attributes are found
pub fn extract_attributes<'a, 'input>(
    node: Node<'a, 'input>,
    allowed_attributes_names: HashSet<&str>,
    path: &Vec<String>,
) -> Result<HashMap<String, String>, ParseError> {
    // check for invalid attribute name
    if let Some(invalid_attribute_name) = node
        .attributes()
        .find(|att| !(allowed_attributes_names.contains(att.name())))
    {
        return Err(ParseError::InvalidAttribute {
            attribute_name: invalid_attribute_name.name().to_string(),
            tag_path: path.clone(),
        });
    }

    // get all attributes
    let mut res: HashMap<String, String> = HashMap::new();
    for att in node.attributes().into_iter() {
        if None == res.get(att.name()) {
            res.insert(att.name().to_string(), att.value().to_string());
        } else {
            return Err(ParseError::AttributeSpecifiedMultipleTime {
                attribute_name: att.name().to_string(),
                tag_path: path.clone(),
            });
        }
    }

    Ok(res)
}

/// Extracts all child nodes, returning an error if the node contains non-element content (skip comments).
fn extract_children<'a, 'input>(
    node: Node<'a, 'input>,
    path: &Vec<String>,
) -> Result<Vec<Node<'a, 'input>>, ParseError> {
    let mut res = Vec::new();
    for (idx, child) in node.children().clone().enumerate() {
        if child.is_pi() {
            return Err(ParseError::InvalidContent {
                tag_path: path.clone(),
                type_name: "processing instruction".to_string(),
                position: idx as u8,
            });
        } else if child.is_text() {
            if !child
                .text()
                .unwrap_or_default()
                .chars()
                .all(|c| c.is_whitespace())
            {
                return Err(ParseError::InvalidContent {
                    tag_path: path.clone(),
                    type_name: "text".to_string(),
                    position: idx as u8,
                });
            } else {
                continue;
            }
        }
        res.push(child);
    }
    Ok(res)
}

/// Extracts all child nodes,
/// returning an error if no children are found or if non-element content is present.
pub fn extract_children__at_least_one<'a, 'input>(
    node: Node<'a, 'input>,
    path: &Vec<String>,
) -> Result<Vec<Node<'a, 'input>>, ParseError> {
    let children = extract_children(node, path)?;
    if children.len() == 0 {
        Err(ParseError::MissingChild {
            tag_path: {
                let mut p = path.clone();
                p.push(node.tag_name().name().to_string());
                p
            },
        })
    } else {
        Ok(children)
    }
}

/// Extracts a single child node,
/// returning an error if zero or more than one child is present.
pub fn extract_children__exactly_one<'a, 'input>(
    node: Node<'a, 'input>,
    path: &Vec<String>,
) -> Result<Node<'a, 'input>, ParseError> {
    let children = extract_children__at_least_one(node, path)?;

    if children.len() > 1 {
        Err(ParseError::MoreThanOneChild {
            tag_path: {
                let mut p = path.clone();
                p.push(node.tag_name().name().to_string());
                p
            },
        })
    } else {
        Ok(children
            .into_iter()
            .next()
            .expect("must be Some regarding logic (because of extract_tags__at_least_one)"))
    }
}

/// Extracts the node's text content,
/// returning an error if any non-text content is found.
pub fn extract_text__only_text<'a, 'input>(
    node: Node<'a, 'input>,
    path: &Vec<String>,
) -> Result<String, ParseError> {
    let children = node.children();

    let not_text_tag = children
        .clone()
        .enumerate()
        .find(|(_index, child)| !child.is_text() && !child.is_comment());

    match not_text_tag {
        Some((idx, child)) => Err(ParseError::InvalidContent {
            tag_path: path.clone(),
            type_name: if child.is_pi() {
                "processing instruction"
            } else {
                "xml tag"
            }
            .to_string(),
            position: idx as u8,
        }),
        None => Ok(match children.clone().next() {
            Some(child) => child.text().unwrap_or_default().to_string(),
            None => String::new(),
        }),
    }
}

/// Parses a generic XML node into a Tag structure.
pub fn try_parse_to_tag<'a, 'input>(
    node: Node<'a, 'input>,
    path: &mut Vec<String>,
) -> Result<Tag, ParseError> {
    path.push(node.tag_name().name().to_string());
    match node.tag_name().name() {
        "t" => Ok(tags::t::T::parse(node, path)?.into()),
        "btn" => Ok(tags::btn::Btn::parse(node, path)?.into()),
        "col" => Ok(tags::row::Row::parse(node, path)?.into()),
        "row" => Ok(tags::col::Col::parse(node, path)?.into()),
        // "none" => Ok(tags::_none::None::parse(node, path)?.into()),
        tag_name => Err(ParseError::InvalidTag {
            tag_name: tag_name.to_string(),
            tag_path: path.clone(),
        }),
    }
}

/// Parses a generic XML node into a Window structure.
pub fn try_parse_window<'a, 'input>(root: Node<'a, 'input>) -> Result<Window, ParseError> {
    let path = &mut vec!["window".to_string()];
    match root.tag_name().name() {
        "window" => Ok(Window::parse(root, path)?.into()),
        tag_name => Err(ParseError::InvalidRootTag {
            tag_name: tag_name.to_string(),
        }),
    }
}
