use std::fs::File;
use std::io::{self, Read};


pub fn lua_writer(&path: String, key: &str, value: &str) -> Result<confirm, err>{
    let mut file = File::open(path);

}

pub fn toml_writer(&path: String, key: &str, value: &str) -> Result<confirm, err>{
    let mut file = File::open(path);
    
}

pub fn css_writer(&path: String, key: &str, value: &str) -> Result<confirm, err>{
    let mut file = File::open(path);
    
}

pub fn json_writer(&path: String, key: &str, value: &str) -> Result<confirm, err>{
    let mut file = File::open(path);
    
}

pub fn text_writer(&path: String, key: &str, value: &str) -> Result<confirm, err>{
    let mut file = File::open(path);
    
}