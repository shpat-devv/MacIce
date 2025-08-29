//handle different data types:
//json, lua, toml, regular
//accept themes as vectors

pub mod value_changer_mod{
    use std::collections::HashMap;
    pub struct ValueChanger{
        pub themes: Vec<Vec<String>>,
    }

    impl ValueChanger{
        pub fn new(given_themes: Vec<Vec<String>>) -> ValueChanger {
            let value_changer = ValueChanger { themes: given_themes };
            value_changer
        }

        pub fn change_theme(&self, paths: HashMap<String, String>) {

        }

        fn change_json(value: String, path: String) {

        }
        fn change_lua(value: String, path: String) {

        }
        fn change_toml(value: String, path: String) {

        }
        fn change_txt(value: String, path: String) {

        }    
    }
}