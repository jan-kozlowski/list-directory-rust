use std::{ffi::OsStr, fmt::format, io::Read, path::PathBuf};
use colored::Colorize;

mod config;
use config::*;

/*
    if file begins with '.', returns the complete file name
    for example, .gitignore will return .gitignore
*/
fn get_extension_from_string(file_name: &OsStr) -> &OsStr {

    let extension = std::path::Path::new(file_name).extension().unwrap_or(file_name);
    extension
}

fn list_directory_contents(config: &ProgramConfig, current_dir: PathBuf, depth: u8) -> std::io::Result<()> {

    if depth > config.get_depth() {

        return Ok(());
    }

    let dir = std::fs::read_dir(&current_dir)?;
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
    
    // directories.sort();
    // files.sort();

    let directory_config = config.get_config("directory");

    for dir_name in directories {
            
        print(directory_config, &dir_name, depth);

        let new_dir = format!("{}/{}", current_dir.display(), dir_name.display());

        let _ = list_directory_contents(config, PathBuf::from(new_dir), depth + 1);
    }

    for file_name in files {
            
        let extension = get_extension_from_string(&file_name).to_str()
                                    .unwrap_or("default").to_string();
        let file_config = config.get_config(&extension);

        print(file_config, &file_name, depth);
    }

    Ok(())
}
fn main() {

    let program_config = get_config_data();
    let current_dir = std::env::current_dir().unwrap_or(PathBuf::from("./"));

    match list_directory_contents(&program_config, current_dir, 0) {
        Ok(_v) => {},
        Err(e) => println!("Failed to read directory: {}", e)
    }
}

fn print(file_config: &FileConfig, file_name: &OsStr, depth: u8) {

    let color = file_config.get_color();
    let line = if depth == 0 
    {
        format!(" {}  {}", file_config.get_icon(), file_name.display())
    } else {
        let s = "--".repeat(depth as usize);
        format!(" |{} {}  {}", s, file_config.get_icon(), file_name.display())
    };

    println!("{}", line.custom_color(color));
}

fn get_config_data() -> ProgramConfig {
    
    let args: Vec<String> = std::env::args().collect();
    let config_path = args.get(2);  
    match config_path {
        None => ProgramConfig::default(),
        Some(path) => {

            let read = read_config_file(path);
            match read {
                Err(e) => {

                    println!("error reading config file: {}", e);
                    ProgramConfig::default()
                },
                Ok(v) => {

                    match toml::from_str(v.as_str()) {
                        Err(e) => {
                            println!("error parsing config file: {}", e);
                            ProgramConfig::default()
                        },
                        Ok(v) => v
                    }
                }
            }
        }
    }
}

fn read_config_file(config_path: &str) -> std::io::Result<String> {

    let mut config_file = std::fs::File::open(config_path)?;
    let mut buffer = String::new();
    config_file.read_to_string(&mut buffer)?;

    Ok(buffer)
}