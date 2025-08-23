use std::ffi::OsStr;

use colored::Colorize;

fn list_directory_contents(dir_color: colored::Color) -> std::io::Result<()> {

    let current_dir = std::env::current_dir()?;
    let dir = std::fs::read_dir(current_dir)?;

    let mut directories = Vec::new();
    let mut files = Vec::new();

    for dir_entry in dir {

        let entry = dir_entry?;
        let file_name = entry.file_name();
        let file_type = entry.file_type()?;

        if file_type.is_dir() {

            directories.push(file_name);
        }
        else if file_type.is_file() {

            files.push(file_name);
        }
    }

    directories.sort();
    files.sort();

    for os_string in directories {
            
        match os_string.to_str() {
            None => {},
            Some(v) => {
                
                let mut dir_name = String::from(v);
                dir_name.push('/');
                println!("{}", dir_name.color(dir_color));
            }
        };
    }
    for os_string in files {
            
        match os_string.to_str() {
            None => {},
            Some(name) => {
                
                println!("{}", name);
            }
        };
    }

    Ok(())
}

fn main() {

    let dir_color = colored::Color::BrightCyan;
    match list_directory_contents(dir_color) {
        Ok(_v) => {},
        Err(_e) => println!("Failed to read directory")
    }
}