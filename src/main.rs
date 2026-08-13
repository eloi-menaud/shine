use std::path::{Component, PathBuf};
use clap::error::ContextValue::Strings;
use log::LevelFilter;
use roxmltree::Document;
use serde::Deserialize;
use std::io::Write;

mod parser;

use parser::tags::t;

use crate::parser::tags::t::T;



fn main() {

    let xml_data = r#"<t>Bonjour le monde</t>"#;
    
    // 1. Parse le document XML en arbre roxmltree
    let doc = Document::parse(xml_data).unwrap();

    // 2. Récupère le nœud racine <t>
    let root = doc.root_element();

    // 3. Conversion idiomatique grâce à try_into()
    let t_obj: t::T = T::into_node_ctx(root, &mut Vec::new()).unwrap(); //(&root)

    // Affichage de l'objet instancié
    println!("{:#?}", t_obj);

}
