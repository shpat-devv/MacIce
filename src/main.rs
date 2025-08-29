use fs_utils::file_handler::file_handler_mod::FileHandler;
use fs_utils::csv_hanlder::csv_handler_mod::CsvHandler;

mod fs_utils;

use std::io;

const CSV_PATH: &str = "paths.csv";

const PROGRAMS: [&str; 5] = [
    "yabai",
    "sketchybar",
    "alacritty",
    "code", //vs code
    "firefox"
];
const POSSIBLE_PATHS: [&str; 5] = [
    ".config/yabai/yabairc",
    ".config/sketchybar/colors.lua",
    ".config/alacritty/alacritty.toml",
    "Library/Application Support/Code/User/settings.json",
    "Library/Application Support/Firefox/Profiles/vct3x7mf.DownToneUi/chrome/DownToneUI/_globals.css",
];
fn main() {
    let mut csv_handler: CsvHandler = CsvHandler::new(String::from(CSV_PATH));
    let mut file_handler: FileHandler = FileHandler::new();

    let mut program_index = 0;

    for program in PROGRAMS{
        println!("checking {program}");
        if !csv_handler.read(String::from(program)) {
            let possible_path = String::from(POSSIBLE_PATHS[program_index]);
            let missing_path = file_handler.finder(possible_path); //if the possible path doesn't work it prompts the user for input
            csv_handler.write(&String::from(program), &missing_path);
        }
        program_index += 1;
    }
}
