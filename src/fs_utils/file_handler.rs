pub mod file_handler_mod {
    use std::collections::HashMap;
    use std::fs::File;
    use std::path::PathBuf;
    use std::io::{stdin};    
    use whoami;
    pub struct FileHandler{
        pub paths: HashMap<String, String>,
        pub check_valid: bool,
    }

    impl FileHandler{
        pub fn new() -> FileHandler{
            let file_handler = FileHandler { paths: HashMap::new(), check_valid: (false) };
            file_handler
        }

        pub fn finder(&self, possible_path: String) -> String{
            let username = whoami::username();
            let full_path = format!("/Users/{}/{}", username, possible_path);
            let file = PathBuf::from(&full_path);

            if file.is_file(){
                full_path  
            } else {
                let mut user_input = String::new();

                while user_input.trim().is_empty() {
                    println!("Please input a valid path: ");
                    stdin()
                        .read_line(&mut user_input)
                        .expect("Couldn't read file, please try again:");
                }
                user_input.trim().to_string()
            }
        }

        pub fn check(&mut self){
            for file in &self.paths{
                let (key, value) = file;
                println!("checking for {} at path: {:?}", key, value);

                let file = File::open(value);

                let _file = match file {
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