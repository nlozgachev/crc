use crate::file_ops::{create_file, get_new_file_paths};
use std::collections::HashMap;

use crate::{
    dir_ops::{create_component_dir, get_component_dir},
    placeholder::replace_placeholder,
    templates::embed_templates,
};

pub fn create_component_hashmap<T>(index: T, view: T, styles: T) -> HashMap<&'static str, T> {
    return HashMap::from([("index", index), ("view", view), ("styles", styles)]);
}

pub fn setup_component_contents(
    templates: HashMap<&'static str, String>,
    component_name: &str,
) -> HashMap<&'static str, String> {
    let mut hmap: HashMap<&'static str, String> = HashMap::new();

    for (k, template) in templates.into_iter() {
        hmap.insert(k, replace_placeholder(&template, component_name));
    }

    hmap
}

pub fn create_component(path_string: &str, component_name: &str) {
    create_component_dir(path_string, component_name);

    let out_dir: String = get_component_dir(path_string, component_name);
    let templates: HashMap<&'static str, String> = embed_templates();
    let component_contents: HashMap<&'static str, String> =
        setup_component_contents(templates, component_name);
    let file_paths = get_new_file_paths(&out_dir, component_name, &component_contents);

    for (_, v) in file_paths.into_iter() {
        create_file(&v.file_path, &v.content);
    }
}
