mod fs_utils; 

use std::collections::HashMap;
use fs_utils::utils::io_utils::FileHandler;


fn main() {
    let paths = HashMap::from([
        (String::from("yabai"), String::from("/Users/uglyprincess/.config/yabai/yabairc")),
        (String::from("sketchybar"), String::from("/Users/uglyprincess/.config/sketchybar/colors.lua")),
        (String::from("alacritty"), String::from("/Users/uglyprincess/.config/alacritty/alacritty.toml")),
        (String::from("vscode"), String::from("/Users/uglyprincess/Library/Application Support/Code/User/settings.json")),
        (String::from("firefox"), String::from("/Users/uglyprincess/Library/Application Support/Firefox/Profiles/vct3x7mf.DownToneUi/chrome/DownToneUI/_globals.css")),
        (String::from("spicetify"), String::from("/Users/uglyprincess/.config/spicetify")),
    ]);
     

    let mut file_handler = FileHandler::new(paths);
    file_handler.check();
}
