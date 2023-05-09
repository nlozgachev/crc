#![forbid(unsafe_code)]

mod cli;
mod file_utils;
mod string_utils;

use self::{cli::*, file_utils::*, string_utils::*};
use std::path::Path;

#[derive(Debug)]
struct ComponentFile {
    filename: String,
    content: String,
}

impl ComponentFile {
    fn new(filename: &String, content: &String) -> Self {
        Self {
            filename: filename.to_string(),
            content: content.to_string(),
        }
    }
}

fn create_component(component_name: &String) {
    let out_dir: String = get_component_dir(component_name);

    create_component_dir(component_name);

    let component_contents_index: String =
        replace_placeholder(include_str!("templates/index.ts"), component_name);
    let component_contents_view: String =
        replace_placeholder(include_str!("templates/COMPONENT.view.tsx"), component_name);
    let component_contents_styles: String = replace_placeholder(
        include_str!("templates/COMPONENT.styles.ts"),
        component_name,
    );
    // const component_contents_store: &String =
    //     &include_str!("templates/COMPONENT.store.ts").replace("COMPONENT", component_name);
    // const component_contents_stories =
    //     &include_str!("templates/COMPONENT.stories.tsx").replace("COMPONENT", component_name);

    let fname_view: &String = &format!("{out_dir}/{component_name}.view.tsx");
    let fname_styles: &String = &format!("{out_dir}/{component_name}.styles.ts");
    let fname_index: &String = &format!("{out_dir}/index.ts");

    let component_paths: Vec<ComponentFile> = vec![
        ComponentFile::new(
            &Path::new(fname_index).display().to_string(),
            &component_contents_index,
        ),
        ComponentFile::new(
            &Path::new(fname_view).display().to_string(),
            &component_contents_view,
        ),
        ComponentFile::new(
            &Path::new(fname_styles).display().to_string(),
            &component_contents_styles,
        ),
    ];

    for component_data in component_paths {
        create_file(&component_data.filename, &component_data.content);
    }
}

fn main() {
    let cli: Cli = Cli::parse();
    let path: String = cli.path;
    let component: String = cli.name;
    println!("Value for component: {component}");
    println!("Value for path: {path}");
    create_component(&component)
}
