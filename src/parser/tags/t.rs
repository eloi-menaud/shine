

use iced::{alignment::{Horizontal, Vertical}};
use roxmltree::Node;
use crate::parser::{ParseError, extract_attribute, tags::check_duplicate_attributes};




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


define_struct_with_names!{
    #[derive(Debug)]
    pub struct T {
        pub content: String,
        pub horizontal_align: Horizontal,
        pub vertical_align: Vertical
    }
}


impl T {
    pub fn into_node_ctx<'a, 'input>(node: Node<'a, 'input>, path: &mut Vec<String>) -> Result<Self, ParseError> {

        path.push(String::from("t"));
        
        check_duplicate_attributes(node.attributes()).map_err(|e| ParseError::DuplicatedAttribute { attribute_name: e.to_string(), tag_path: path.clone() })?;
        if let Some(invalid_attr) = node.attributes().find(|attr| !T::ALL_FIELDS.contains(&attr.name())){
            return Err(ParseError::InvalidAttributeName { attribute_name: invalid_attr.name().to_string(), tag_path: path.clone() });
        }
        
                
        Ok(T{
            content: node.text().unwrap_or("").to_string(),
            horizontal_align: extract_attribute::<Horizontal>(node,T::horizontal_align, path.clone())?.unwrap_or(Horizontal::Left),
            vertical_align: extract_attribute::<Vertical>(node,T::vertical_align, path.clone())?.unwrap_or(Vertical::Top),
        })
    }
}


