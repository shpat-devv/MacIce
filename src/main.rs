use fs_utils::file_handler::file_handler_mod::FileHandler;
use fs_utils::csv_handler::csv_handler_mod::CsvHandler;
use fs_utils::value_changer::value_changer_mod::ValueChanger;

use enums::path_return::return_enum_mod::ReturnPath;

use std::io::stdin;

mod fs_utils;
mod enums;

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
    let theme_changer: ValueChanger = ValueChanger::new();

    let mut selected_theme: String = String::new();

    //finding and reading all the paths
    let mut program_index = 0;
    for program in PROGRAMS{
        println!("checking {program}");
        let string_program = String::from(program); //this could probably have a better solution
        /* 
        if csv_handler.read(&string_program) == ReturnPath(bool) {
            let possible_path = String::from(POSSIBLE_PATHS[program_index]);
            let missing_path = &file_handler.finder(string_program, possible_path); //if the possible path doesn't work it prompts the user for input
            csv_handler.write(&String::from(program), &missing_path);
        } else {
            file_handler.paths.insert(string_program.copy(), csv_handler.)
        }
        */
        match ReturnPath{
            
        }
        program_index += 1;
    }

    println!("Please select one of the following themes:");
    for theme in &theme_changer.themes{
        println!("{}", theme.0);
    }
     
    while selected_theme.trim().is_empty(){
        stdin()
            .read_line(&mut selected_theme)
            .expect("Couldn't read line...");
    }

    theme_changer.change_theme(&file_handler.paths, selected_theme);
}
