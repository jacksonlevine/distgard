use std::{collections::HashMap, fs::{self, File}, io::{Read, Write}, path::Path, str::FromStr};

use bevy::math::IVec3;
use once_cell::sync::Lazy;
use uuid::Uuid;

use serde;
use tracing::info;


pub static mut MY_MULTIPLAYER_UUID: Lazy<Uuid> = Lazy::new(|| Uuid::new_v4());

pub static mut LAST_ENTERED_SERVERADDRESS: Lazy<String> = Lazy::new(|| String::from(""));


use serde::{Serialize, Deserialize};
use serde_json::{self, from_str, to_string_pretty};

#[derive(Serialize, Deserialize)]
pub struct MiscellaneousSettingsData {
    pub mouse_sense: f32,
    pub music_vol: f32,
    pub sound_vol: f32,
    #[serde(with = "vectorize")]
    pub keybinds: HashMap<i32, String>,
    #[serde(with = "vectorize")]
    pub mousebinds: HashMap<String, String>,
    #[serde(default = "support_controller_default")]
    pub controllersupport: bool,
    #[serde(with = "vectorize", default = "singleplayer_worlds_default")]
    pub singleplayer_worlds: HashMap<usize, u32>,
    #[serde(with = "vectorize", default = "worldnames_default")]
    pub world_names: HashMap<usize, String>,
    #[serde(default = "brightness_default")]
    pub brightness: f32

}

pub fn support_controller_default() -> bool {
    true
}

pub fn brightness_default() -> f32 {
    0.0
}

pub fn singleplayer_worlds_default() -> HashMap<usize, u32> {
    HashMap::new()
}

pub fn waypoints_default() -> HashMap<String, IVec3> {
    HashMap::new()
}

pub fn worldnames_default() -> HashMap<usize, String> {
    let mut h= HashMap::new();
    h.insert(0, "World 1".into());
    h.insert(1, "World 2".into());
    h.insert(2, "World 3".into());
    h.insert(3, "World 4".into());
    h.insert(4, "World 5".into());
    h
}

pub static mut MISCSETTINGS: Lazy<MiscellaneousSettingsData> = Lazy::new(|| MiscellaneousSettingsData {
    mouse_sense: 0.25,
    music_vol: 1.0,
    sound_vol: 1.0,
    keybinds: HashMap::from([
        (glfw::Key::Escape.get_scancode().unwrap(), "Exit/Menu".into()),
        (glfw::Key::W.get_scancode().unwrap(), "Forward".into()),
        (glfw::Key::A.get_scancode().unwrap(), "Left".into()),
        (glfw::Key::S.get_scancode().unwrap(), "Backward".into()),
        (glfw::Key::D.get_scancode().unwrap(), "Right".into()),

        (glfw::Key::C.get_scancode().unwrap(), "Craft".into()),
        
        (glfw::Key::Space.get_scancode().unwrap(), "Jump/Swim/Climb Up".into()),
        (glfw::Key::LeftShift.get_scancode().unwrap(), "Sprint".into()),
        (glfw::Key::LeftControl.get_scancode().unwrap(), "Crouch".into()),

        (glfw::Key::Num0.get_scancode().unwrap(), "Fov Up".into()),
        (glfw::Key::Num9.get_scancode().unwrap(), "Fov Down".into()),
    ]),
    mousebinds: HashMap::from([
        ("Button2".into(), "Place/Use".into()),
        ("Button1".into(), "Break/Attack".into()),

    ]),
    controllersupport: true,
    singleplayer_worlds: HashMap::new(),
    world_names: HashMap::new(),
    brightness: 0.0
} );

pub fn save_misc() {
    unsafe {
        let misc_settings = &*MISCSETTINGS;
        let json = to_string_pretty(misc_settings).expect("Failed to serialize MISCSETTINGS");
        let mut file = File::create("misc").expect("Failed to create file");
        file.write_all(json.as_bytes()).expect("Failed to write data to file");
    }
}

pub fn load_misc() {
    if Path::new("misc").exists() {
        let mut file = File::open("misc").expect("Failed to open file");
        let mut json = String::new();
        file.read_to_string(&mut json).expect("Failed to read file");
        let mut loaded_settings: MiscellaneousSettingsData = from_str(&json).expect("Failed to deserialize JSON");
        if !loaded_settings.keybinds.contains_key(&glfw::Key::B.get_scancode().unwrap()) {
            loaded_settings.keybinds.insert(glfw::Key::B.get_scancode().unwrap(), "Build Mode Toggle".into());
        }

        unsafe {
            *MISCSETTINGS = loaded_settings;
            save_misc();
        }
    }
}

pub fn save_lesa() {
    let mmu = "lesa";

    // Always create or overwrite the file
    let mut file = File::create(mmu).unwrap();
    unsafe {
        file.write_all(LAST_ENTERED_SERVERADDRESS.to_string().as_bytes()).unwrap();
    }
    info!("UUID file created or overwritten.");
}

pub fn load_or_initialize_statics() {
    
    let mmu = "mmu";

    if Path::new(mmu).exists() {
        // If the file exists, read its contents
        let contents = fs::read_to_string(mmu).unwrap();
        let uuid = Uuid::from_str(&contents).unwrap();
        unsafe {
            (*MY_MULTIPLAYER_UUID) = uuid;
        }
        info!("UUID file loaded, its {uuid}.");
    } else {
        // If the file doesn't exist, create and write to it
        let mut file = File::create(mmu).unwrap();
        unsafe {
            file.write_all(MY_MULTIPLAYER_UUID.to_string().as_bytes()).unwrap();
        }
        unsafe {
            info!("UUID file created and written to, with {}.", *MY_MULTIPLAYER_UUID);
        }
        
    }

    let mmu = "lesa";

    if Path::new(mmu).exists() {
        // If the file exists, read its contents
        let contents = fs::read_to_string(mmu).unwrap();
        unsafe {
            (*LAST_ENTERED_SERVERADDRESS) = contents;
        }
        info!("UUID file loaded.");
    } else {
        // If the file doesn't exist, create and write to it
        let mut file = File::create(mmu).unwrap();
        unsafe {
            file.write_all(LAST_ENTERED_SERVERADDRESS.to_string().as_bytes()).unwrap();
        }
        info!("UUID file created and written to.");
    }



}
