use crate::{chunk::USERDATAMAPANDMISCMAP,*};

use dashmap::DashMap;
use sled::Db;
use bevy::{prelude::*, utils::HashMap};

// #[derive(Clone)]
// pub struct UserDataMapAndMiscMap(pub Db);
#[derive(Clone)]
pub struct UserDataMapAndMiscMap(pub Db, pub DashMap<IVec3, u32>);



impl UserDataMapAndMiscMap {
    pub fn get(&self, vec: &IVec3) -> Option<u32> {
        get_udm_entry(vec, self)
        
    }

    pub fn insert(&self, vec: IVec3, block: u32) {
        put_udm_entry(&vec, block, self);
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

pub fn get_udm_entry(key: &IVec3, udm: &UserDataMapAndMiscMap) -> Option<u32>
{

    match udm.1.get(key) {
        Some(id) => Some(*id.value()),
        None => None,
    }
}

pub fn put_udm_entry(key: &IVec3, block: u32, udm: &UserDataMapAndMiscMap) {
    println!("Inserting {} at {}", block.to_string(), key.to_string());
    udm.1.insert(*key, block);
}

pub fn put_misc_entry(key: &str, data: Vec<u8>) {
    put_entry(b"misc".to_vec(), key.to_string().as_bytes().to_vec(), data);
}

pub fn get_misc_entry(key: &str) -> Option<Vec<u8>> {
    return get_entry(b"misc".to_vec(), key.to_string().as_bytes().to_vec());
}