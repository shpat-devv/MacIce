pub mod io_utils {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{self, Read};
    pub struct FileHandler{
        pub paths: HashMap<String, String>,
        pub check_valid: bool,
    }

    impl FileHandler{
        pub fn new(input_paths: HashMap<String, String>) -> FileHandler{
            let file_handler = FileHandler { paths: input_paths, check_valid: (false) };
            file_handler
        }

        pub fn check(mut self){
            for file in &self.paths{
                let (key, value) = file;
                println!("checking for {} at path: {}", key, value);

                let file = File::open(value);

                let file = match file {
                    Ok(file) => file,
                    Err(error) => panic!("Couldn't open file. {:?}", error)
                };

                println!("{} file path valid, checking next path..", key);
                println!("-----------------------------------------------------");
            }
            self.check_valid = true;
        }
    }
}