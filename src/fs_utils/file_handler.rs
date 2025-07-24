pub mod file_handler_mod {
    use std::collections::HashMap;
    use std::fs::File;
    use std::path::PathBuf;
    use std::io::{stdin};    
    use whoami;
    pub struct FileHandler{
        pub paths: HashMap<String, PathBuf>,
        pub check_valid: bool,
    }

    impl FileHandler{
        pub fn new() -> FileHandler{
            let file_handler = FileHandler { paths: HashMap::new(), check_valid: (false) };
            file_handler
        }

        pub fn finder(&mut self){
            let username = whoami::username();

            let files_to_check: Vec<(&'static str, &'static str)> = vec![
                ("yabai", ".config/yabai/yabairc"),
                ("sketchybar", ".config/sketchybar/colors.lua"),
                ("alacritty", ".config/alacritty/alacritty.toml"),
                ("code", "Library/Application Support/Code/User/settings.json"),
                ("firefox", "Library/Application Support/Firefox/Profiles/vct3x7mf.DownToneUi/chrome/DownToneUI/_globals.css"),
            ];


            for (key, path) in files_to_check{
                let full_path = format!("/Users/{}/{}", username, path);
                let file = PathBuf::from(&full_path);
                if file.is_file(){
                    self.paths.insert(String::from(key), file);
                } else {
                    let mut user_input = String::new();
                    while user_input.is_empty(){
                        println!("Please input {} path:", key);
                        stdin()
                            .read_line(&mut user_input)
                            .expect("Couldn't read file, please try again:");
                    }
                    self.paths.insert(String::from(key), PathBuf::from(user_input));
                }
            }

        }

        pub fn check(&mut self){
            for file in &self.paths{
                let (key, value) = file;
                println!("checking for {} at path: {:?}", key, value);

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