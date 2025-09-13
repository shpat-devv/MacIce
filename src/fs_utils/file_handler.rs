pub mod file_handler_mod{
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::io::{stdin};    
    use whoami;
    pub struct FileHandler{
        pub paths: HashMap<String, String>,
    }
    impl FileHandler{
        pub fn new() -> FileHandler{
            let file_handler = FileHandler { paths: HashMap::new() };
            file_handler
        }

        pub fn finder(&mut self, program: String, possible_path: String) -> String{
            let username = whoami::username();
            let full_path = format!("/Users/{}/{}", username, possible_path);
            let file = PathBuf::from(&full_path);

            if file.is_file(){
                self.paths.insert(program, full_path.clone());
                println!("inserted {:?}", self.paths);
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
    }
}