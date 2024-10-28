use crate::{chunk::USERDATAMAPANDMISCMAP,*};

use sled::Db;
use bevy::{prelude::*, utils::HashMap};

#[derive(Clone)]
pub struct UserDataMapAndMiscMap(pub Db);

impl UserDataMapAndMiscMap {
    pub fn get(&self, vec: &IVec3) -> Option<u32> {
        get_udm_entry(vec)
    }

    pub fn insert(&self, vec: IVec3, block: u32) {
        put_udm_entry(&vec, block);
    }
}


pub fn key_to_bytes(key: &IVec3) -> Result<Vec<u8>, borsh::io::Error> {
    borsh::to_vec(&[key.x, key.y, key.z])
}

pub fn prefix_key(prefix: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    let mut fullkey = prefix;
    fullkey.extend(key);
    fullkey
}

pub fn get_entry(prefix: Vec<u8>, key: Vec<u8>) -> Option<Vec<u8>> {
    match unsafe { &USERDATAMAPANDMISCMAP } {
        Some(db) => {
            let db = db.0.clone();
            match db.get(prefix_key(prefix, key)) {
                Ok(value) => {
                    match value {
                        Some(value) => {
                            Some(value.to_vec())
                        }
                        None => {
                            None
                        }
                    }
                }
                Err(e) => {
                    None
                }
            }
        }
        None => {
            None
        }
    }
}



pub fn put_entry(prefix: Vec<u8>, key: Vec<u8>, value: Vec<u8>) {
    match unsafe { &USERDATAMAPANDMISCMAP } {
        Some(db) => {
            let db = db.0.clone();
            match db.insert(prefix_key(prefix, key), value) {
                Ok(_) => {
                    //println!("Inserted key");
                }
                Err(e) => {
                    println!("Error inserting key: {}", e);
                }
            };
        }
        None => {

        }
    }
}

pub fn get_udm_entry(key: &IVec3) -> Option<u32>
{
    return get_entry(b"udm".to_vec(), key_to_bytes(key).unwrap())
            .map(|v| u32::from_le_bytes(v.try_into().unwrap()));
}



pub fn put_udm_entry(key: &IVec3, block: u32) {
    put_entry(b"udm".to_vec(), key_to_bytes(key).unwrap(), block.to_le_bytes().to_vec());
}

pub fn put_misc_entry(key: &str, data: Vec<u8>) {
    put_entry(b"misc".to_vec(), key.to_string().as_bytes().to_vec(), data);
}

pub fn get_misc_entry(key: &str) -> Option<Vec<u8>> {
    return get_entry(b"misc".to_vec(), key.to_string().as_bytes().to_vec());
}