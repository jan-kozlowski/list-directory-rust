use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct FileConfig {

    // hex color
    color: String,
    icon: char,
}

impl Default for FileConfig {

    fn default() -> Self {

        Self {
            color: "#90A4AE".to_string(),
            icon: '\u{ea7b}'
        }       
    }
}

impl FileConfig {

    pub fn new(hex: &str, _icon: char) -> Self {
        
        Self { 
            color: hex.to_string(),
            icon: _icon
        }
    }

    pub fn get_color(&self) -> colored::CustomColor {

        hex_to_color(&self.color)
    }

    pub fn get_icon(&self) -> char {

        self.icon
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProgramConfig {

    depth: u8,
    default_file_config: FileConfig,
    default_dir_config: FileConfig,
    file_configs: HashMap<String, FileConfig>,
}

impl Default for ProgramConfig {

    fn default() -> Self {
        
        Self {
            depth: 0,
            default_file_config: FileConfig::default(),
            default_dir_config: FileConfig::new("#DD9623", '\u{f4d4}'),
            file_configs: HashMap::new()
        }
    }
}

impl ProgramConfig {

    pub fn get_config(&self, extension: &str) -> &FileConfig {

        self.file_configs.get(extension)
            .unwrap_or(
        if extension.eq("directory") 
                    { &self.default_dir_config } 
                else 
                    { &self.default_file_config })
    }

    pub fn get_depth(&self) -> u8 {

        self.depth
    }
} 

fn hex_to_color(hex_color: &str) -> colored::CustomColor {

    let hex_value = hex_color.trim_start_matches("#");

    if hex_value.len() < 6 {

        return hex_to_color(&FileConfig::default().color);
    }

    let r = u8::from_str_radix(&hex_value[0..2], 16).unwrap_or(255);
    let g= u8::from_str_radix(&hex_value[2..4], 16).unwrap_or(255);
    let b = u8::from_str_radix(&hex_value[4..6], 16).unwrap_or(255);

    colored::CustomColor::new(r, g, b)
}