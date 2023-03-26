use std::{env, fs, fs::File, io::Write, path::Path};

pub fn get_component_dir(component_name: &String) -> String {
    let path = env::current_dir().unwrap().display().to_string();
    format!("{path}/{component_name}")
}

pub fn create_component_dir(component_name: &String) {
    fs::create_dir(get_component_dir(component_name)).unwrap();
}

pub fn create_file(p: &Path, contents: &String) {
    let mut file = match File::create(&p) {
        Err(why) => panic!("couldn't create {}: {}", p.display(), why),
        Ok(file) => file,
    };

    match file.write_all(contents.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", p.display(), why),
        Ok(_) => println!("successfully wrote to {}", p.display()),
    }
}
