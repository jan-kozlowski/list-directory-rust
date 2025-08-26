use std::{ffi::OsStr, io::{Read, Write}};
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

fn list_directory_contents(config: &ProgramConfig) -> std::io::Result<()> {

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

    // directories.sort();
    // files.sort();

    let directory_config = config.get_config("directory");

    for os_string in directories {
            
        print(directory_config, os_string.into_string().unwrap_or(String::new()));
    }

    for os_string in files {
            
        let extension = get_extension_from_string(&os_string).to_str()
                                    .unwrap_or("default").to_string();
        let file_config = config.get_config(&extension);

        print(file_config, os_string.into_string().unwrap_or(String::new()));
    }

    Ok(())
}
fn main() {

    let program_config = get_config_data();

    match list_directory_contents(&program_config) {
        Ok(_v) => {},
        Err(e) => println!("Failed to read directory: {}", e)
    }
}

fn save_config(config: &ProgramConfig) -> std::io::Result<()> {

    let res = toml::to_string(&config).unwrap_or_default();
    print!("{}", res);
    let mut f = std::fs::File::create("./src/theme.toml")?;
    f.write_all(res.as_bytes())?;
    // let mut f = std::fs::File::options().write(true).open("src/theme.toml")?;
    // f.write(res.as_bytes())?;
    // let y = std::fs::write("/themes/default.toml", res);
    
    Ok(())
}

fn print(file_config: &FileConfig, file_name: String) {

    let color = file_config.get_color();
    let icon_string = file_config.get_icon().to_string().custom_color(color);
    let file_name_string = file_name.custom_color(color);
    println!("{}  {}", icon_string, file_name_string);
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