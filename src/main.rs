#![forbid(unsafe_code)]

mod cli;
mod file_utils;

use self::{cli::*, file_utils::*};
use std::path::Path;

fn create_component(component_name: &String) {
    let out_dir = get_component_dir(component_name);

    create_component_dir(component_name);

    let component_contents_view =
        &include_str!("templates/COMPONENT.view.tsx").replace("COMPONENT", component_name);
    let component_contents_styles =
        &include_str!("templates/COMPONENT.styles.ts").replace("COMPONENT", component_name);
    // let component_contents_store: &String =
    //     &include_str!("templates/COMPONENT.store.ts").replace("COMPONENT", component_name);
    // let component_contents_stories =
    //     &include_str!("templates/COMPONENT.stories.tsx").replace("COMPONENT", component_name);

    let fname_view = &format!("{out_dir}/{component_name}.view.tsx");
    let fname_styles = &format!("{out_dir}/{component_name}.styles.ts");

    let component_view_path_new = Path::new(fname_view);
    let component_styles_path_new = Path::new(fname_styles);

    create_file(component_view_path_new, component_contents_view);
    create_file(component_styles_path_new, component_contents_styles);
}

// #[derive(Debug)]
// struct ComponentFile {
//     filename: String,
//     content: String,
// }

// impl ComponentFile {
//     fn new(filename: &str, content: &str) -> Self {
//         Self {
//             filename: filename.to_string(),
//             content: content.to_string(),
//         }
//     }
// }

fn main() {
    // let mut files_collection: Vec<ComponentFile> = Vec::new();

    // for file in fs::read_dir("./src/templates").unwrap() {
    //     let filename = file.unwrap().file_name().to_str().unwrap().to_string();
    //     let content = "content";
    //     files_collection.push(ComponentFile::new(filename.as_str(), content));

    //     println!("{}", files_collection[0].content);
    // }

    // println!("{:#?}", files_collection);

    let cli = Cli::parse();
    if let Some(path) = cli.path.to_str() {
        let component = cli.name;
        println!("Value for component: {component}");
        println!("Value for path: {path}");
        create_component(&component)
    }
}
