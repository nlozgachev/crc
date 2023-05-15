use std::{collections::HashMap, fs, io::Write, process};

use crate::component::create_component_hashmap;

pub struct ComponentFile {
    pub file_path: String,
    pub content: String,
}

impl ComponentFile {
    fn new(file_path: &str, content: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
            content: content.to_string(),
        }
    }
}

pub fn create_file(p: &String, contents: &String) {
    let mut file: fs::File = match fs::File::create(&p) {
        Err(why) => {
            eprintln!("Couldn't create {}: {}", p, why);
            process::exit(1);
        }
        Ok(file) => file,
    };

    match file.write_all(contents.as_bytes()) {
        Err(why) => {
            eprintln!("Couldn't write to {}: {}", p, why);
            process::exit(1);
        }
        Ok(_) => println!("Successfully created {}", p),
    }
}

pub fn get_output_filepath(out_dir: &str, component_name: &str, file_id: &str) -> Option<String> {
    let file_paths: HashMap<&'static str, String> = create_component_hashmap(
        format!("{out_dir}/index.ts"),
        format!("{out_dir}/{component_name}.view.tsx"),
        format!("{out_dir}/{component_name}.styles.ts"),
    );

    return file_paths.get(file_id).cloned();
}

pub fn get_new_file_paths(
    out_dir: &str,
    component_name: &str,
    contents: &HashMap<&'static str, String>,
) -> HashMap<&'static str, ComponentFile> {
    let mut out: HashMap<&'static str, ComponentFile> = HashMap::new();

    for (k, contents) in contents {
        match get_output_filepath(out_dir, component_name, &k) {
            Some(path) => {
                out.insert(k, ComponentFile::new(&path, contents));
            }
            None => {
                eprintln!("Couldn't find filepath for {}", k);
                process::exit(1);
            }
        }
    }

    return out;
}
