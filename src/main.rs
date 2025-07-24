use std::collections::HashMap;
use fs_utils::file_handler::file_handler_mod::FileHandler;
use fs_utils::finder;
use fs_utils::finder::finder_mod;
use std::path::PathBuf;
use std::io::stdin;

mod fs_utils;

fn main() {
    let mut file_handler = FileHandler::new();
    file_handler.finder();
    file_handler.check();
    
}
