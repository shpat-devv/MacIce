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

        pub fn check(&mut self){
            for file in &self.paths{
                
            }
        }
    }
}