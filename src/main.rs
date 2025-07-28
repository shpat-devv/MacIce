use fs_utils::file_handler::file_handler_mod::FileHandler;
use fs_utils::csv_hanlder::csv_handler_mod::CsvHandler;

mod fs_utils;

const CSV_PATH: &str = "paths.csv";
fn main() {
    let mut file_handler = FileHandler::new();
    let csv_handler = CsvHandler::new(String::from(CSV_PATH));

    file_handler.finder();
    file_handler.check();
    
    csv_handler.read(String::from("fsfasfasdf"));
}
