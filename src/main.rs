use std::{collections::HashMap, ffi::OsStr, io::Read, process::exit, str::FromStr};

use colored::Colorize;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct FileConfig {

    // r, g, b
    color: [u8; 3],
    icon: char,
}

impl FileConfig {

    pub fn get_color(&self) -> colored::CustomColor {

        colored::CustomColor::new(self.color[0], self.color[1], self.color[2])
    }
}

#[derive(Serialize, Deserialize)]
struct ProgramConfig {

    file_configs: HashMap<String, FileConfig>,
}

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

    let default_dir_config = &FileConfig{
        color: [144, 164, 174],
        icon: ''
    };

    let default_file_config = &FileConfig{
        color: [144, 164, 174],
        icon: ''
    };

    let directory_config = config.file_configs.get("directory").unwrap_or(default_dir_config);

    for os_string in directories {
            
        let color = directory_config.get_color();
        print(directory_config.get_color(), directory_config.icon, os_string.into_string().unwrap_or(String::new()));
    }
    for os_string in files {
            
        let extension = get_extension_from_string(&os_string).to_str()
                                    .unwrap_or("none").to_string();
        let file_config = config.file_configs.get(&extension).unwrap_or(default_file_config);

        print(file_config.get_color(), file_config.icon, os_string.into_string().unwrap_or(String::new()));
    }

    Ok(())
}
fn main() {

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {

        exit(1);
    } 

    let config_path = &args[2];  

    let read = read_config_data(config_path);
    match read {
        Err(_e) => println!("error reading file"),
        Ok(v) => {

            let program_config: ProgramConfig = serde_json::from_str(v.as_str()).unwrap();
            
            match list_directory_contents(&program_config) {
                Ok(_v) => {},
                Err(_e) => println!("Failed to read directory")
            }
        }
    }
}

fn print(color: colored::CustomColor, icon: char, file_name: String) {

    let icon_string = icon.to_string().custom_color(color);
    let file_name_string = file_name.custom_color(color);
    println!("{}  {}", icon_string, file_name_string);
}

fn serialize(pc: &ProgramConfig) -> serde_json::Result<()> {

    let fc = &pc.file_configs;

    for x in fc {

        let config = x.1;
        let color = colored::CustomColor::new(
            config.color[0], config.color[1], config.color[2]);
        println!("{}:", x.0);
        println!("  icon: {}", String::from(config.icon).custom_color(color));
    }

    let s = serde_json::to_string(&pc)?;
    println!("{}", s);

    Ok(())
}

fn read_config_data(config_path: &str) -> std::io::Result<String> {

    let mut config_file = std::fs::File::open(config_path)?;
    let mut buffer = String::new();
    config_file.read_to_string(&mut buffer)?;

    Ok(buffer)
}