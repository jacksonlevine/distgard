
use std::{fs::{File, OpenOptions}, io::{Read, Write}, path::Path};

use bevy::prelude::*;

use crate::rlencode::{rldecode_chunk, rlencode_chunk};

pub struct LevelSaver {
    
}







impl LevelSaver {
    pub fn loadpos(seed: u32, cpos: &IVec2) {
        println!("Loading {}", cpos.to_string());
        let paths = path!((seed.to_string().replace("-", "N").replace("1", "A") + "/") + cpos.to_string().replace("-", "N").replace("[", "C").replace("]", "C").replace(",", "X").replace(" ", "").as_str());
        let path = Path::new(&paths);
        match OpenOptions::new().read(true).open(path) {
            Ok(mut file) => {
                let mut string = String::new();
                match file.read_to_string(&mut string) {
                    Ok(_) => {
                        println!("File exists: {}", string);
                        rldecode_chunk(string, cpos);
                    }
                    Err(e) => {
                        println!("Failed to read file {}, {}", paths, e);
                        info!("Failed to read file {}, {}", paths, e);
                    }
                }
                
            }
            Err(e) => {

            }
        }
    }

    pub fn savepos(seed: u32, cpos: &IVec2) {
        println!("Saving: {}", cpos.to_string());
        let paths = path!((seed.to_string().replace("-", "N").replace("1", "A") + "/") + cpos.to_string().replace("-", "N").replace("[", "C").replace("]", "C").replace(",", "X").replace(" ", "").as_str());
        let path = Path::new(&paths);

        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();

        println!("about to encode chunk");
        let savestring = rlencode_chunk(cpos);
        println!("encoded chunk");
        match File::create(path) {
            Ok(mut file) => {
                match file.write_all(savestring.as_bytes()) {
                    Ok(_) => {

                    }
                    Err(e) => {
                        println!("Failed to write file {}, {}", paths, e);
                        info!("Failed to write file {}, {}", paths, e);
                    }
                }
            }
            Err(e) => {
                println!("Failed to create file {}, {}", paths, e);
                info!("Failed to create file {}, {}", paths, e);
            }
        }
        
    }

    // pub fn unloadpos(cpos: &IVec2) {

    // }
}