use std::collections::HashSet;

use roxmltree::Attributes;

pub mod t;

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