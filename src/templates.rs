use std::collections::HashMap;

use crate::component::create_component_hashmap;

pub fn embed_templates() -> HashMap<&'static str, &'static str> {
    return create_component_hashmap(
        include_str!("templates/index.ts"),
        include_str!("templates/COMPONENT.view.tsx"),
        include_str!("templates/COMPONENT.styles.ts"),
    );
}
