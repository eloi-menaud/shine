use std::collections::HashSet;

use iced::Element;
use roxmltree::{Attributes, Node};

use crate::{parser::{ParseError, tags::Tag::Void, types::AttrFromStr}, render::Message};

pub mod t;
pub mod col;
pub mod row;
pub mod window;
pub mod void;
pub mod bttn;

#[derive(Debug)]
pub enum Tag {
    T(t::T),
    Row(row::Row),
    Col(col::Col),
    Bttn(bttn::Bttn),
    Void(void::Void)
}
impl Tag {
    pub fn from_node_ctx(node: Node, path: &mut Vec<String>) -> Result<Self,ParseError> {
        Ok(match node.tag_name().name() {
            "t" => Self::T( t::T::from_node_ctx(node, path)? ),
            "row" => Self::Row( row::Row::from_node_ctx(node, path)? ),
            "col" => Self::Col( col::Col::from_node_ctx(node, path)? ),
            "bttn" => Self::Bttn( bttn::Bttn::from_node_ctx(node, path)? ),
            invalid_tag_name => return Err(ParseError::InvalidTag { tag_name: invalid_tag_name.to_string(), tag_path: path.clone(), range: node.range() })
        })
    }

    pub fn render<'a>(&self) -> Element<'a, Message> {
        match self {
            Tag::T(t) => t.render(),
            Tag::Row(row) => row.render(),
            Tag::Col(col) => col.render(),
            Tag::Bttn(bttn) => bttn.render(),
            Tag::Void(void) => void.render()
        }
    }
}
impl Default for Tag{
    fn default() -> Self {
        Void(void::Void{})
    }
}


/// Get children but remove all node with empty tag name
/// 
/// when the source XML containes \n,\s,\t the AST stock those as node
/// instead of ignoring those formating chars. 
pub fn extract_children<'a, 'd>(node: &'a Node<'a, 'd>) -> Vec<Node<'a, 'd>> {
    node.children()
        .filter(|n| !n.tag_name().name().is_empty())
        .collect()
}

pub fn extract_attribute<T: AttrFromStr>(node: Node, name: &str, path: &Vec<String>) -> Result<Option<T>,ParseError> {
    match node.attribute(name){
        Some(attr_value) => T::attr_from_str(attr_value)
            .map(|val| Some(val))
            .map_err(|err| ParseError::InvalidAttributeValue {
                attribute_name: name.to_string(),
                reason: err.to_string(),
                tag_path: path.clone(), range: node.range()
            }),
        None => Ok(None)
    }
}



pub fn check_duplicate_attributes<'a, 'input>(
    attributes: Attributes<'a, 'input>,
) -> Result<(), &'input str> {
    let mut seen = HashSet::new();
    for attr in attributes {
        let key = attr.name(); 

        if !seen.insert(key) {
            return Err(key);
        }
    }

    Ok(())
}



#[macro_export]
macro_rules! define_struct_with_names {
    (
        $(#[$meta:meta])*
        pub struct $struct_name:ident {
            $(pub $field_name:ident : $field_type:ty),* $(,)?
        }
    ) => {
        // 1. On génère la struct normale
        $(#[$meta])*
        pub struct $struct_name {
            $(pub $field_name : $field_type),*
        }

        // 2. On génère un bloc `impl` avec le nom de chaque champ sous forme de constante
        impl $struct_name {
            $(
                pub const $field_name: &'static str = stringify!($field_name);
            )*

            /// Tableau contenant TOUS les noms de champs (super pratique pour valider les inconnus !)
            pub const ALL_FIELDS: &'static [&'static str] = &[
                $( stringify!($field_name) ),*
            ];
        }
    };
}