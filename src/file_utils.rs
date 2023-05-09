use std::{env, fs, io::Write};

pub fn get_component_dir(component_name: &String) -> String {
    let path: String = match env::current_dir() {
        Err(why) => panic!(
            "couldn't get current directory for {}: {}",
            component_name, why
        ),
        Ok(dir) => dir.display().to_string(),
    };

    format!("{path}/{component_name}")
}

pub fn create_component_dir(component_name: &String) {
    match fs::create_dir(get_component_dir(component_name)) {
        Err(why) => panic!("couldn't create {}: {}", component_name, why),
        Ok(file) => file,
    };
}

pub fn create_file(p: &String, contents: &String) {
    let mut file: fs::File = match fs::File::create(&p) {
        Err(why) => panic!("couldn't create {}: {}", p, why),
        Ok(file) => file,
    };

    match file.write_all(contents.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", p, why),
        Ok(_) => println!("successfully wrote to {}", p),
    }
}
