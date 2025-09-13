pub mod value_changer_mod{
    use std::collections::HashMap;

    enum FileType {
        Text,
        Lua,
        Toml,
        Json,
    }

    pub struct ValueChanger{
        pub themes: HashMap<String, Vec<String>>
    }
    impl ValueChanger{
        pub fn new() -> ValueChanger {
            let dark_theme = vec![
                String::from("0x333333"), // fg
                String::from("0x333333"), // bg
                String::from("0x757575"), // border
                String::from("Dark Modern"), // theme
            ];

            let light_theme = vec![
                String::from("0x111111"), // fg
                String::from("0xFFFFFF"), // bg
                String::from("0x444444"), // border
                String::from("Light Modern"), // theme
            ];

            let mut available_themes = HashMap::new();
            
            available_themes.insert(String::from("dark"), dark_theme);
            available_themes.insert(String::from("light"), light_theme);

            let value_changer = ValueChanger { themes: available_themes };
            value_changer
        }

        pub fn change_theme(&self, paths: &HashMap<String, String>, theme: String) {
            //paths order: yabai, sketchybar, alacritty, vscode
            for path in paths{
                if path.0 == "yabai"{
                    self.change_txt(&self.themes.get(path.0), path.1);
                }
            }
        }

        fn change_txt(&self, value: &Option<&Vec<String>>, path: &String) {
            match value {
                Some(value) => {
                    for (i, item) in value.iter().enumerate(){
                        println!("a lil loopy: {i}, {item}")
                    }
                } 
                None => {
                    println!("shi doesn't exist");
                }
            }
        }  
        fn change_lua(&self, value: String, path: String) {

        }
        fn change_toml(&self, value: String, path: String) {

        }
        fn change_json(&self, value: &str, path: &str) {

        }
    }
}