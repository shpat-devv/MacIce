use fs_utils::file_handler::file_handler_mod::FileHandler;
use fs_utils::csv_hanlder::csv_handler_mod::CsvHandler;

mod fs_utils;

fn main() {
    let mut file_handler = FileHandler::new();
    file_handler.finder();
    file_handler.check();
    
}
