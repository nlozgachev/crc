use std::collections::HashMap;

use crate::component::create_component_hashmap;

pub fn embed_templates() -> HashMap<&'static str, String> {
    return create_component_hashmap(
        include_str!("templates/index.ts").to_string(),
        include_str!("templates/COMPONENT.view.tsx").to_string(),
        include_str!("templates/COMPONENT.styles.ts").to_string(),
    );
}
