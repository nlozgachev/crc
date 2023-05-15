use std::{fs, path, process};

fn get_dir_path(path: &str) -> String {
    return path::Path::new(&path).display().to_string();
}

pub fn get_component_dir(path_string: &str, component_name: &str) -> String {
    let path = get_dir_path(path_string);

    return format!("{path}/{component_name}");
}

pub fn create_component_dir(path_string: &str, component_name: &str) {
    match fs::create_dir_all(get_component_dir(path_string, component_name)) {
        Err(why) => {
            eprintln!("Couldn't create directory for {}: {}", component_name, why);
            process::exit(1);
        }
        Ok(file) => file,
    };
}
