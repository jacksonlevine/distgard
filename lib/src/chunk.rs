use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::fs::File;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use std::thread;
use std::time::Duration;

use bevy::math::U16Vec3;
use borsh::BorshSerialize;
use dashmap::DashMap;

pub static CHUNKPOSDEFAULT: i32 = 999999;

use bevy::prelude::*;
use gl::types::GLuint;
// use lockfree::queue::Queue;
use num_enum::FromPrimitive;
use once_cell::sync::Lazy;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use sled::Db;
use rusqlite::params;
use rusqlite::Connection;

use std::sync::Arc;

use parking_lot::{Mutex, RwLock};

use noise::{NoiseFn, Perlin};

use crate::blockinfo::BLOCK_MARKED_FOR_DELETION;
use crate::camera::Camera;
use crate::chunkregistry::ChunkMemory;
use crate::chunkregistry::ChunkRegistry;
use crate::climates;
use crate::climates::get_climate;
use crate::climates::get_floor_block_based_on_climate;
use crate::climates::get_vox_mod_from_treetype;
use crate::climates::Elevation;
use crate::cube::Cube;
use crate::cube::CubeSide;

use crate::database::UserDataMapAndMiscMap;
use crate::everis::EverisVisitSpot;
use crate::everis::EVERIS_VISIT_QUEUE;
use crate::everis::EVERIS_KILLED_YET;
use crate::everis::MAX_LIGHTS;
use crate::everis::QUEUED_FOR_EVERIS;
#[cfg(feature = "audio")]
use crate::game::AUDIOPLAYER;

use crate::game::CHUNKSYS;
//use crate::game::CHUNKDRAWINGHERE;
use crate::game::CURRSEED;

// use crate::game::PLAYERCHUNKPOS;
use crate::game::PLAYERPOS;
use crate::game::SINGLEPLAYER;
use crate::game::VOXEL_MODELS;
use crate::game::WEATHERTYPE;
use crate::packedvertex::PackedVertex;
use crate::planetinfo::Planets;
use crate::shader::Shader;
use crate::specialblocks::awds::AwdsInfo;
use crate::specialblocks::chest::ChestInfo;
use crate::specialblocks::conveyor::ConveyorInfo;
use crate::specialblocks::crafttable::CraftTableInfo;
use crate::specialblocks::door::DoorInfo;
use crate::specialblocks::fence::FenceInfo;
use crate::specialblocks::jackolantern::JackolanternInfo;
use crate::specialblocks::ladder::LadderInfo;
use crate::specialblocks::tallgrass::TallGrassInfo;
use crate::specialblocks::torch::TorchInfo;
use crate::textureface::TextureFace;
use crate::textureface::ONE_OVER_16;
use crate::textureface::TEXTURE_WIDTH;
use crate::vechelp::ivec2length;
use crate::windowandkey::AM_I_A_FUCKING_SERVER;

use arrayvec::ArrayVec;

use tracing::info;

use crate::blockinfo::Blocks;
use crate::voxmodel::JVoxModel;

use std::io::Write;

pub type LightColor = U16Vec3;

//pub static check_for_intercepting: Lazy<Queue<IVec3>> = Lazy::new(|| Queue::new());

#[derive(Clone)]
pub struct LightRay {
    pub value: LightColor,
    pub origin:IVec3,
    pub directions: Vec<CubeSide>,
}

pub struct LightSegment {
    pub rays: ArrayVec<LightRay, 16>,
}

impl LightSegment {
    pub fn sum(&self) -> LightColor {
        let mut res = LightColor::ZERO;
        for ray in &self.rays {
            res += ray.value;
        }
        return res.min(LightColor::new(15, 15, 15));
    }
}

pub struct ChunkGeo {
    pub data32: Mutex<Vec<u32>>,
    pub data8: Mutex<Vec<u8>>,
    pub data8rgb: Mutex<Vec<u16>>,

    pub pos: Mutex<IVec2>,

    pub vbo32: gl::types::GLuint,
    pub vbo8: gl::types::GLuint,
    pub vbo8rgb: GLuint,

    pub tdata32: Mutex<Vec<u32>>,
    pub tdata8: Mutex<Vec<u8>>,
    pub tdata8rgb: Mutex<Vec<u16>>,

    pub tvbo32: gl::types::GLuint,
    pub tvbo8: gl::types::GLuint,
    pub tvbo8rgb: GLuint,

    pub vvbo: GLuint,
    pub uvvbo: GLuint,

    pub wvvbo: GLuint,
    pub wuvvbo: GLuint,

    pub vdata: Mutex<Vec<f32>>,
    pub uvdata: Mutex<Vec<f32>>,

    pub wvdata: Mutex<Vec<f32>>,
    pub wuvdata: Mutex<Vec<f32>>,
}
impl ChunkGeo {
    pub fn new() -> ChunkGeo {
        let mut vbo32: gl::types::GLuint = 0;
        let mut vbo8: gl::types::GLuint = 0;
        let mut tvbo32: gl::types::GLuint = 0;
        let mut tvbo8: gl::types::GLuint = 0;
        let mut vbo8rgb: GLuint = 0;
        let mut tvbo8rgb: GLuint = 0;

        let mut vvbo: gl::types::GLuint = 0;
        let mut uvvbo: gl::types::GLuint = 0;

        let mut wvvbo: gl::types::GLuint = 0;
        let mut wuvvbo: gl::types::GLuint = 0;
        #[cfg(feature = "glfw")]
        unsafe {
            gl::CreateBuffers(1, &mut vbo32);
            gl::CreateBuffers(1, &mut vbo8);
            gl::CreateBuffers(1, &mut tvbo32);
            gl::CreateBuffers(1, &mut tvbo8);

            gl::CreateBuffers(1, &mut vvbo);
            gl::CreateBuffers(1, &mut uvvbo);

            gl::CreateBuffers(1, &mut wvvbo);
            gl::CreateBuffers(1, &mut wuvvbo);

            gl::CreateBuffers(1, &mut vbo8rgb);
            gl::CreateBuffers(1, &mut tvbo8rgb);

            let error = gl::GetError();
            if error != gl::NO_ERROR {
                info!(
                    "OpenGL Error after creating chunk system buffers: {}",
                    error
                );
            }
        }

        let data32 = Mutex::new(Vec::new());
        let data8 = Mutex::new(Vec::new());
        let data8rgb = Mutex::new(Vec::new());

        let tdata32 = Mutex::new(Vec::new());
        let tdata8 = Mutex::new(Vec::new());
        let tdata8rgb = Mutex::new(Vec::new());
        let vdata = Mutex::new(Vec::new());
        let uvdata = Mutex::new(Vec::new());

        let wvdata = Mutex::new(Vec::new());
        let wuvdata = Mutex::new(Vec::new());

        ChunkGeo {
            data32,
            data8,
            data8rgb,
            pos: Mutex::new(IVec2 {
                x: CHUNKPOSDEFAULT,
                y: CHUNKPOSDEFAULT,
            }),
            vbo32,
            vbo8,
            vbo8rgb,
            tdata32,
            tdata8,
            tdata8rgb,
            tvbo32,
            tvbo8,
            tvbo8rgb,

            vvbo,
            uvvbo,

            wvvbo,
            wuvvbo,

            vdata,
            uvdata,

            wvdata,
            wuvdata,
        }
    }

    pub fn clear(&self) {
        self.data32.lock().clear();
        self.data8.lock().clear();
        self.tdata32.lock().clear();
        self.tdata8.lock().clear();

        self.vdata.lock().clear();
        self.uvdata.lock().clear();

        self.wvdata.lock().clear();
        self.wuvdata.lock().clear();

        self.data8rgb.lock().clear();
        self.tdata8rgb.lock().clear();
    }
    pub fn solids(&self) -> (&Mutex<Vec<u32>>, &Mutex<Vec<u8>>, &Mutex<Vec<u16>>) {
        return (&self.data32, &self.data8, &self.data8rgb);
    }
    pub fn transparents(&self) -> (&Mutex<Vec<u32>>, &Mutex<Vec<u8>>, &Mutex<Vec<u16>>) {
        return (&self.tdata32, &self.tdata8, &self.tdata8rgb);
    }
}

#[derive(Clone, Copy)]
pub struct ChunkFacade {
    pub geo_index: usize,
    pub used: bool,
    pub pos:IVec2,
}

pub const CH_W: i32 = 15;
pub const CH_H: i32 = 200;

pub struct ReadyMesh {
    pub geo_index: usize,
    pub newpos:IVec2,
    pub newlength: i32,
    pub newtlength: i32,
    pub newvlength: i32,
    pub newwvlength: i32,
}

impl ReadyMesh {
    pub fn new(
        index: usize,
        newpos: &IVec2,
        newlength: i32,
        newtlength: i32,
        newvlength: i32,
        newwvlength: i32,
    ) -> ReadyMesh {
        ReadyMesh {
            geo_index: index,
            newpos: *newpos,
            newlength,
            newtlength,
            newvlength,
            newwvlength,
        }
    }
}

pub struct AutomataChange {
    pub expectedhere: u32,
    pub spot: IVec3,
    pub changeto: u32,
}

impl AutomataChange {
    pub fn new(expectedhere: u32, spot: IVec3, changeto: u32) -> Self {
        Self {
            expectedhere,
            spot,
            changeto,
        }
    }
}

pub struct ACSet {
    pub count: usize,
    pub changes: [AutomataChange; 2],
}

impl ACSet {
    pub fn new(count: usize, changes: [AutomataChange; 2]) -> Self {
        Self { changes, count }
    }
}

pub static mut ALREADY_QUEUED_KEYS: Lazy<DashMap<IVec2, u8>> = Lazy::new(|| DashMap::new());


pub static mut AUTOMATA_QUEUED_CHANGES: Lazy<VecDeque<ACSet>> = Lazy::new(|| VecDeque::new());



//MEMBERS TAKEN OUT AS PER BEVY MIGRATION PLAN STEP 1

//pub static mut USERDATAMAP: Option<Arc<DashMap<IVec3, u32>>> = None;


pub static mut USERDATAMAPANDMISCMAP: Option<UserDataMapAndMiscMap> = None;




pub static mut NONUSERDATAMAP: Option<Arc<DashMap<IVec3, u32>>> = None;



pub static mut GIS_QUEUED: Lazy<DashMap<usize, bool>> = Lazy::new(|| DashMap::new());

pub static mut LIGHT_GIS_QUEUED: Lazy<DashMap<usize, bool>> = Lazy::new(|| DashMap::new());

pub struct ChunkSystem {
    pub chunks: Vec<Arc<Mutex<ChunkFacade>>>,
    pub geobank: Vec<Arc<ChunkGeo>>,
    pub takencare: Arc<DashMap<IVec2, ChunkFacade>>,
    pub finished_user_geo_queue: Arc<lockfree::queue::Queue<ReadyMesh>>,
    pub finished_geo_queue: Arc<lockfree::queue::Queue<ReadyMesh>>,
    pub user_rebuild_requests: lockfree::queue::Queue<usize>,
    pub gen_rebuild_requests: lockfree::queue::Queue<usize>,
    pub light_rebuild_requests: lockfree::queue::Queue<usize>,
    pub background_rebuild_requests: lockfree::queue::Queue<usize>,
    
    // pub userdatamap: Arc<DashMap<IVec3, u32>>,
    // pub nonuserdatamap: Arc<DashMap<IVec3, u32>>,
    
    pub justcollisionmap: DashMap<IVec3, u8>,
    pub radius: u8,
    pub perlin: Arc<RwLock<Perlin>>,
    pub chunk_memories: Mutex<ChunkRegistry>,
    pub planet_type: u8,

    pub headless: bool,
    pub hashadinitiallightpass: Arc<Mutex<HashMap<IVec2, bool>>>,
    pub lightmap: Arc<Mutex<HashMap<IVec3, LightSegment>>>,

    pub generated_chunks: Arc<DashMap<IVec2, bool>>,
}

impl ChunkSystem {
    
    pub fn new(radius: u8, seed: u32, noisetype: usize, headless: bool) -> ChunkSystem {

            if(unsafe { AM_I_A_FUCKING_SERVER }) {

                println!("IM A SERVER OR SINGLEPLAYER");
                unsafe {
                    //USERDATAMAP = Some(Arc::new(DashMap::new()));
                    println!("Opening db");
                    match sled::open(String::from("dgsaves/") + seed.to_string().as_str()) {
                        Ok(db) => {
                            println!("Opened db, assigning to USERDATAMAP");
                            USERDATAMAPANDMISCMAP = Some(UserDataMapAndMiscMap(db));
                            println!("Opened db");
                        }
                        Err(e) => {
                            println!("Error opening db: {}", e);
                        }
                    }
    
                    NONUSERDATAMAP = Some(Arc::new(DashMap::new()));
                }
            } else {
                unsafe {
                    println!("IM A CLIENT TRYING TO JOIN MULTIPLAYER");
                    //USERDATAMAP = Some(Arc::new(DashMap::new()));
                    println!("Opening db");
                    match sled::open(String::from("dgsavesmp/") + seed.to_string().as_str()) {
                        Ok(db) => {
                            println!("Opened db, assigning to USERDATAMAP");
                            USERDATAMAPANDMISCMAP = Some(UserDataMapAndMiscMap(db));
                            println!("Opened db");
                        }
                        Err(e) => {
                            println!("Error opening db: {}", e);
                        }
                    }
    
                    NONUSERDATAMAP = Some(Arc::new(DashMap::new()));
                }
            }
            


        let mut cs = ChunkSystem {
            chunks: Vec::new(),
            geobank: Vec::new(),
            takencare: Arc::new(DashMap::new()),
            finished_user_geo_queue: Arc::new(lockfree::queue::Queue::new()),
            finished_geo_queue: Arc::new(lockfree::queue::Queue::new()),
            user_rebuild_requests: lockfree::queue::Queue::new(),
            gen_rebuild_requests: lockfree::queue::Queue::new(),
            light_rebuild_requests: lockfree::queue::Queue::new(),
            background_rebuild_requests: lockfree::queue::Queue::new(),
           // userdatamap: Arc::new(DashMap::new()),
           // nonuserdatamap: Arc::new(DashMap::new()),
            justcollisionmap: DashMap::new(),
            radius,
            perlin: Arc::new(RwLock::new(Perlin::new(seed))),
            chunk_memories: Mutex::new(ChunkRegistry {
                memories: Vec::new(),
            }),
            planet_type: noisetype as u8,
            headless,
            hashadinitiallightpass: Arc::new(Mutex::new(HashMap::new())),
            lightmap: Arc::new(Mutex::new(HashMap::new())),
            generated_chunks: Arc::new(DashMap::new()),
        };

        // let directory_path = "assets/voxelmodels/";

        // for entry in WalkDir::new(directory_path) {
        //     let entry = entry.unwrap();
        //     if entry.file_type().is_file() {
        //         let path_str = entry.path().to_string_lossy().into_owned();
        //         let jv = JVoxModel::new(Box::leak(path_str.into_boxed_str()));
        //         //info!("{:#?}", jv.model);
        //         cs.voxel_models.push(jv);
        //     }
        // }
        if !cs.headless {
            for _ in 0..radius * 2 + 5 {
                for _ in 0..radius * 2 + 5 {
                    cs.chunks.push(Arc::new(Mutex::new(ChunkFacade {
                        geo_index: cs.geobank.len(),
                        used: false,
                        pos: IVec2 {
                            x: CHUNKPOSDEFAULT,
                            y: CHUNKPOSDEFAULT,
                        },
                    })));

                    cs.geobank.push(Arc::new(ChunkGeo::new()));

                    cs.chunk_memories
                        .lock()
                        .memories
                        .push(ChunkMemory::new(&cs.geobank[cs.geobank.len() - 1]));
                }
            }
        }

        //tracing::info!("Amount of chunkgeo buffers: {}", 4 * cs.geobank.len());

        cs
    }
    

    pub fn change_seed(&self, newseed: u32) {

        let new_perlin = Perlin::new(newseed);
        {
            let mut perlin_lock = self.perlin.write(); 
            *perlin_lock = new_perlin; 
        }
        println!("Replaced Perlin noise generator with a new seed: {}", newseed);

        if(unsafe { AM_I_A_FUCKING_SERVER }) {

            println!("IM A SERVER OR SINGLEPLAYER");
            unsafe {
                //USERDATAMAP = Some(Arc::new(DashMap::new()));
                println!("Opening db");
                match sled::open(String::from("dgsaves/") + newseed.to_string().as_str()) {
                    Ok(db) => {
                        println!("Opened db, assigning to USERDATAMAP");
                        USERDATAMAPANDMISCMAP = Some(UserDataMapAndMiscMap(db));
                        println!("Opened db");
                    }
                    Err(e) => {
                        println!("Error opening db: {}", e);
                    }
                }

                NONUSERDATAMAP = Some(Arc::new(DashMap::new()));
            }
        } else {
            unsafe {
                println!("Changing seed for a CLIENT TRYING TO JOIN MULTIPLAYER");
                //USERDATAMAP = Some(Arc::new(DashMap::new()));
                println!("Opening db");
                match sled::open(String::from("dgsavesmp/") + newseed.to_string().as_str()) {
                    Ok(db) => {
                        println!("Opened db, assigning to USERDATAMAP");
                        USERDATAMAPANDMISCMAP = Some(UserDataMapAndMiscMap(db));
                        println!("Opened db");
                    }
                    Err(e) => {
                        println!("Error opening db: {}", e);
                    }
                }

                NONUSERDATAMAP = Some(Arc::new(DashMap::new()));
            }
        }



        unsafe { CURRSEED.store(newseed, std::sync::atomic::Ordering::Relaxed); }
    }
    pub fn write_new_udm_entry(&self, spot:IVec3, block: u32) {
        let seed = unsafe { CURRSEED.load(std::sync::atomic::Ordering::Relaxed) };
        let table_name = format!("userdatamap_{}", seed);

        let conn = Connection::open("db").unwrap();

        // Insert userdatamap entries
        let mut stmt = conn
            .prepare(&format!(
                "INSERT OR REPLACE INTO {} (x, y, z, value) VALUES (?, ?, ?, ?)",
                table_name
            ))
            .unwrap();

        stmt.execute(params![spot.x, spot.y, spot.z, block])
            .unwrap();
    }

    pub fn collision_predicate(&self, vec:IVec3) -> bool {
        let isntwater = (self.blockat(vec.clone()) & Blocks::block_id_bits()) != 2;
        let isnttallgrass = (self.blockat(vec.clone()) & Blocks::block_id_bits()) != 23;

        return isntwater && isnttallgrass && self.blockat(vec.clone()) != 0
            || self.justcollisionmap.contains_key(&vec);
    }

    pub fn start_with_seed(_seed: u32) {}

    pub fn do_automata(&self, _cam: &Arc<Mutex<Camera>>) {
        pub const ODDBIT: u32 = 0b1000_0000_0000_0000_0000_0000_0000_0000;
        //let chunkslist = self.chunks.clone();

        let takencare = self.takencare.clone();

        let udm = unsafe {USERDATAMAPANDMISCMAP.as_ref().unwrap()}.clone();
        let nudm = unsafe {NONUSERDATAMAP.as_ref().unwrap()}.clone();
        let per = self.perlin.clone();
        //let cam = cam.clone();

        thread::spawn(move || {
            let mut rng = StdRng::from_entropy();

            let mut odd_frame: u32 = 0;

            let mut time: f32 = 0.0;
            
            let mut instant = std::time::Instant::now();

            loop {
                if instant.elapsed().as_secs_f32() < 5.0 {
                    thread::sleep(Duration::from_secs(5));
                }
                else{

                let psnap = unsafe { PLAYERPOS.snapshot() };

                let pcpos = ChunkSystem::spot_to_chunk_pos(&IVec3::new(
                    psnap.pos.0 as i32,
                    0,
                    psnap.pos.2 as i32,
                ));

                //let camposx = pcpos.x;
                //let camposz = pcpos.y;

                for i in -6..6 {
                    for j in -6..6 {
                        let poshere = pcpos + IVec2::new(i, j);
                        match takencare.get(&poshere) {
                            Some(c) => {
                                let c = c.value();
                                

                                let cdist = ivec2length(&(c.pos - poshere)) as i32;

                                for i in 0..CH_W {
                                    for k in 0..CH_W {
                                        // let hit_block = false;
                                        for j in (0..CH_H).rev() {
                                            let spot =IVec3 {
                                                x: ((c.pos.x) * CH_W) + i,
                                                y: j,
                                                z: (c.pos.y * CH_W) + k,
                                            };

                                            
                                           


                                            if rng.gen_range(0..((cdist*2)+10)) == 0 {
                                            
                                            let temp = ChunkSystem::_temp_noise(&per.read(), spot) as f32;
                                           // let hum = ChunkSystem::_humidity_noise(&per.read(), spot) as f32;
                                            //let climate = get_climate(temp, hum);
                                                    

                                            let combined =
                                                Self::_blockat(&nudm, &udm, &per.read(), spot);
                                            let block = combined & Blocks::block_id_bits();
                                            // let flags = combined & Blocks::block_flag_bits();
                                            unsafe {
                                                //println!("weathertype: {}", WEATHERTYPE);
                                                if true {
                                                    //WEATHERTYPE == 1.0 {

                                                    if (combined & BLOCK_MARKED_FOR_DELETION) != 0 {
                                                        AUTOMATA_QUEUED_CHANGES.push_back(ACSet::new(
                                                            1,
                                                            [AutomataChange::new(
                                                                combined,
                                                                spot,
                                                                0,
                                                            ),
                                                            AutomataChange::new(
                                                                block,
                                                                spot,
                                                                0,
                                                            )],
                                                        ));
                                                    }

                                                    match block {
                                                        3 => {
                                                            if WEATHERTYPE == 1.0 {
                                                                if rng.gen_range(0..100) == 9 {
                                                                    //println!("Pushin one");
                                                                    AUTOMATA_QUEUED_CHANGES.push_back(
                                                                        ACSet::new(1, [
                                                                            AutomataChange::new(
                                                                                block, spot, 48,
                                                                            ),
                                                                            AutomataChange::new(
                                                                                block, spot, 48,
                                                                            ),
                                                                        ])
                                                                    );
                                                                }
                                                            }
                                                            

                                                            //break;
                                                        }
                                                        2 => {

                                                            let waterfall = || {
                                                                let belowspot =
                                                                        spot + IVec3::new(0, -1, 0);
                                                                    let belowcombined = Self::_blockat(
                                                                        &nudm,
                                                                        &udm,
                                                                        &per.read(),
                                                                        belowspot,
                                                                    );
                                                                    let belowblock = belowcombined
                                                                        & Blocks::block_id_bits();

                                                                    if belowblock == 0 {
                                                                        AUTOMATA_QUEUED_CHANGES
                                                                            .push_back(ACSet::new(2, [
                                                                            AutomataChange::new(
                                                                                2 | (ODDBIT
                                                                                    * (odd_frame)),
                                                                                spot,
                                                                                0,
                                                                            ),
                                                                            AutomataChange::new(
                                                                                0,
                                                                                belowspot,
                                                                                2 | (ODDBIT
                                                                                    * (1 - odd_frame)),
                                                                            ),
                                                                        ]));
                                                                    }
                                                            };

                                                            if combined
                                                                == (2u32 | (ODDBIT * odd_frame))
                                                            {
                                                                if WEATHERTYPE == 1.0 {
                                                                    let abovespot =
                                                                        spot + IVec3::new(0, 1, 0);
                                                                    let abovecombined = Self::_blockat(
                                                                        &nudm,
                                                                        &udm,
                                                                        &per.read(),
                                                                        abovespot,
                                                                    );
                                                                    let aboveblock = abovecombined
                                                                        & Blocks::block_id_bits();

                                                                    if aboveblock == 0 {
                                                                        if rng.gen_range(0..100) == 9 {
                                                                            AUTOMATA_QUEUED_CHANGES
                                                                                .push_back(ACSet::new(1, [
                                                                                    AutomataChange::new(
                                                                                        2 | (ODDBIT
                                                                                            * (odd_frame)), spot, 51,
                                                                                    ),
                                                                                    AutomataChange::new(
                                                                                        2 | (ODDBIT
                                                                                            * (odd_frame)), spot, 51,
                                                                                    ),
                                                                                ]));
                                                                        }
                                                                    } else {
                                                                        waterfall();
                                                                    }
                                                                } else {
                                                                    waterfall();
                                                                }
                                                                
                                                            }
                                                        }
                                                        7 => {
                                                            if WEATHERTYPE == 1.0 {
                                                                let abovespot =
                                                                    spot + IVec3::new(0, 1, 0);
                                                                let abovecombined = Self::_blockat(
                                                                    &nudm,
                                                                    &udm,
                                                                    &per.read(),
                                                                    abovespot,
                                                                );
                                                                let aboveblock = abovecombined
                                                                    & Blocks::block_id_bits();

                                                                if aboveblock == 0 {
                                                                    if rng.gen_range(0..100) == 9 {
                                                                        AUTOMATA_QUEUED_CHANGES
                                                                            .push_back(ACSet::new(1, [
                                                                                AutomataChange::new(
                                                                                    7, spot, 50,
                                                                                ),
                                                                                AutomataChange::new(
                                                                                    7, spot, 50,
                                                                                ),
                                                                            ]));
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        51 => {
                                                            if WEATHERTYPE != 1.0 && temp > 0.0 {
                                                                
                                                                if rng.gen_range(0..100) == 9 {
                                                                    AUTOMATA_QUEUED_CHANGES
                                                                        .push_back(ACSet::new(1, [
                                                                            AutomataChange::new(
                                                                                51, spot, 2 | (ODDBIT * (1 - odd_frame)),
                                                                            ),
                                                                            AutomataChange::new(
                                                                                51, spot, 2 | (ODDBIT * (1 - odd_frame)),
                                                                            ),
                                                                        ]));
                                                                }
                                                                
                                                            }
                                                        }
                                                        22 => {
                                                            let abovespot =
                                                                spot + IVec3::new(0, 1, 0);
                                                            let abovecombined = Self::_blockat(
                                                                &nudm,
                                                                &udm,
                                                                &per.read(),
                                                                abovespot,
                                                            );
                                                            let aboveblock = abovecombined
                                                                & Blocks::block_id_bits();

                                                            if aboveblock == 0 {
                                                                if rng.gen_range(0..10) == 9 {
                                                                    AUTOMATA_QUEUED_CHANGES
                                                                        .push_back(ACSet::new(1, [
                                                                            AutomataChange::new(
                                                                                0, abovespot, 22,
                                                                            ),
                                                                            AutomataChange::new(
                                                                                0, abovespot, 22,
                                                                            ),
                                                                        ]) );
                                                                }
                                                            }
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                            }





                                        }


                                        }
                                    }
                                }
                            }
                            None => {}
                        }
                    }
                }

                thread::sleep(Duration::from_secs_f32(0.5));
                odd_frame = 1 - odd_frame;
            }
            }
        });
    }

    pub fn exit(&mut self) {
        if !self.headless {
            for cg in &self.geobank {
                #[cfg(feature = "glfw")]
                unsafe {
                    gl::DeleteBuffers(1, &cg.vbo32);
                    gl::DeleteBuffers(1, &cg.tvbo32);
                    gl::DeleteBuffers(1, &cg.vbo8);
                    gl::DeleteBuffers(1, &cg.tvbo8);
                }
            }
        }

        info!("After deleting buffers");
        self.chunks.clear();
        info!("After clearing chunks");
        self.geobank.clear();
        info!("After clearing geobank");
        self.chunk_memories.lock().memories.clear();
        info!("After clearing memories");
        self.takencare.clear();
        info!("After clearing takencare");
        while let Some(_) = self.finished_geo_queue.pop() {}
        while let Some(_) = self.finished_user_geo_queue.pop() {}
        while let Some(_) = self.user_rebuild_requests.pop() {}
        while let Some(_) = self.gen_rebuild_requests.pop() {}
        while let Some(_) = self.background_rebuild_requests.pop() {}
        info!("After that whole popping thing");
        //self.userdatamap.clear();
        //self.nonuserdatamap.clear();
        self.justcollisionmap.clear();
        info!("After clearing the next 3 things");
    }

    pub fn reset(&mut self, radius: u8, seed: u32, noisetype: usize) {
        info!("Start of reset func");

        self.radius = radius;
        *(self.perlin.write()) = Perlin::new(seed);

        self.planet_type = noisetype as u8;
        unsafe { CURRSEED.store(seed, std::sync::atomic::Ordering::Relaxed) };

        info!("After setting currentseed");

        if !self.headless {
            for _ in 0..radius * 2 + 5 {
                for _ in 0..radius * 2 + 5 {
                    self.chunks.push(Arc::new(Mutex::new(ChunkFacade {
                        geo_index: self.geobank.len(),
                        used: false,
                        pos: IVec2 {
                            x: CHUNKPOSDEFAULT,
                            y: CHUNKPOSDEFAULT,
                        },
                    })));

                    self.geobank.push(Arc::new(ChunkGeo::new()));
                    self.chunk_memories
                        .lock()
                        .memories
                        .push(ChunkMemory::new(&self.geobank[self.geobank.len() - 1]));
                }
            }
        }

        info!("After making new chunk stuff");
    }

    
    pub fn spot_to_chunk_pos(spot: &IVec3) ->IVec2 {
        return IVec2 {
            x: (spot.x as f32 / CH_W as f32).floor() as i32,
            y: (spot.z as f32 / CH_W as f32).floor() as i32,
        };
    }
    pub fn spot_to_chunk_pos_bevyvec3(spot: &bevy::prelude::Vec3) ->IVec2 {
        return IVec2 {
            x: (spot.x as f32 / CH_W as f32).floor() as i32,
            y: (spot.z as f32 / CH_W as f32).floor() as i32,
        };
    }
    pub fn initial_rebuild_on_main_thread(
        csys: &Arc<ChunkSystem>,
        _shader: &Shader,
        campos: &Vec3,
    ) {
        // unsafe {
        //     gl::BindVertexArray(shader.vao);
        //     gl::UseProgram(shader.shader_id);
        // }

        let user_cpos = IVec2 {
            x: (campos.x / CH_W as f32).floor() as i32,
            y: (campos.z / CH_W as f32).floor() as i32,
        };

        let mut neededspots = Vec::new();

        for i in -(csys.radius as i32)..(csys.radius as i32) {
            for k in -(csys.radius as i32)..(csys.radius as i32) {
                let this_spot = IVec2 {
                    x: user_cpos.x + i as i32,
                    y: user_cpos.y + k as i32,
                };
                neededspots.push(this_spot);
            }
        }

        for (index, cpos) in neededspots.iter().enumerate() {
            csys.move_and_rebuild(index, *cpos);
        }

        // let mut genstuff = true;
        // while genstuff {
        //     match csys.gen_rebuild_requests.pop() {
        //         Some(index) => {
        //             csys.rebuild_index(index, true);
        //         }
        //         None => {
        //             genstuff = false;
        //         }
        //     }
        // }

        // let mut more_in_queue = true;
        // while more_in_queue {
        //     match csys.finished_user_geo_queue.pop() {
        //         Some(ready) => {
        //             //info!("Some user queue");
        //            // info!("Weird!");

        //             let bankarc = csys.geobank[ready.geo_index].clone();

        //             let mut cmemlock = csys.chunk_memories.lock();

        //             cmemlock.memories[ready.geo_index].length = ready.newlength;
        //             cmemlock.memories[ready.geo_index].tlength = ready.newtlength;
        //             cmemlock.memories[ready.geo_index].pos = ready.newpos;
        //             cmemlock.memories[ready.geo_index].used = true;

        //             // info!("Received update to {} {} {} {}", ready.newlength, ready.newtlength, ready.newpos.x, ready.newpos.y);
        //             // info!("New cmemlock values: {} {} {} {} {}", cmemlock.memories[ready.geo_index].length, cmemlock.memories[ready.geo_index].tlength, cmemlock.memories[ready.geo_index].pos.x, cmemlock.memories[ready.geo_index].pos.y, cmemlock.memories[ready.geo_index].used);
        //             // //if num == 0 { num = 1; } else { num = 0; }
        //             //bankarc.num.store(num, std::sync::atomic::Ordering::Release);
        //             // if num == 0 {
        //             //     bankarc.num.store(1, Ordering::Relaxed);
        //             //     num = 1;
        //             // } else {
        //             //     bankarc.num.store(0, Ordering::Relaxed);
        //             //     num = 0;
        //             // };

        //             // let v32 = cmemlock.memories[ready.geo_index].vbo32;
        //             // let v8 = cmemlock.memories[ready.geo_index].vbo8;
        //             // let tv32 = cmemlock.memories[ready.geo_index].tvbo32;
        //             // let tv8 = cmemlock.memories[ready.geo_index].tvbo8;

        //             // WorldGeometry::bind_geometry(v32, v8, true, shader, bankarc.solids());
        //             // WorldGeometry::bind_geometry(
        //             //     tv32,
        //             //     tv8,
        //             //     true,
        //             //     shader,
        //             //     bankarc.transparents(),
        //             // );
        //         }
        //         None => {
        //             more_in_queue = false;
        //         }
        //     }
        // }
    }
    pub fn queue_geoindex_rerender(&self, geo_index: usize, user_power: bool, light: bool) {
        if light {
            match unsafe { &LIGHT_GIS_QUEUED.get(&geo_index) } {
                Some(_) => {}
                None => {
                    unsafe { &LIGHT_GIS_QUEUED.insert(geo_index, true) };
                    self.light_rebuild_requests.push(geo_index);
                }
            }
        } else {
            match user_power {
                true => {
                    match unsafe { &GIS_QUEUED.get(&geo_index) } {
                        Some(_) => {}
                        None => {
                            unsafe { &GIS_QUEUED.insert(geo_index, true) };
                            self.user_rebuild_requests.push(geo_index);
                        }
                    }
                }
                false => {
                    match unsafe { &GIS_QUEUED.get(&geo_index) } {
                        Some(_) => {}
                        None => {
                            println!("BACKGROUND REBUILD REQUEST PUSHED");
                            info!("BACKGROUND REBUILD REQUEST PUSHED");
                            unsafe { &GIS_QUEUED.insert(geo_index, true) };
                            self.background_rebuild_requests.push(geo_index);
                        }
                    }
                }
            }
        }
    }
    pub fn queue_rerender(&self, spot:IVec3, user_power: bool, light: bool) {
        let chunk_key = &Self::spot_to_chunk_pos(&spot);
        match self.takencare.get(chunk_key) {
            Some(cf) => {
                self.queue_geoindex_rerender(cf.geo_index, user_power, light);
            }
            None => {}
        }
    }
    pub fn queue_rerender_with_key(&self, chunk_key: IVec2, user_power: bool, light: bool) {
        match self.takencare.get(&chunk_key) {
            Some(cf) => self.queue_geoindex_rerender(cf.geo_index, user_power, light),
            None => {}
        }
    }
    pub fn set_block_and_queue_rerender(
        &self,
        spot:IVec3,
        block: u32,
        neighbors: bool,
        user_power: bool,
        automata: bool,
    ) {
        let existingblock = self.blockat(spot);

        self.set_block(spot, block, user_power);

        let blockislight = Blocks::is_light(block & Blocks::block_id_bits());
        let blockwaslight = Blocks::is_light(existingblock &    Blocks::block_id_bits());

        let light = blockislight || blockwaslight;

        // if !light {
        //     check_for_intercepting.push(spot);
        // }

        if !light && !automata && !Blocks::is_transparent(block & Blocks::block_id_bits()) {
            //If not light still check if it intercepts any lights, we will need to update.

            let mut implicated = HashSet::new();
            for i in Cube::get_neighbors() {
                match self.lightmap.lock().get(&(*i + spot)) {
                    Some(k) => {
                        for ray in &k.rays {
                            let chunkofthisraysorigin = ChunkSystem::spot_to_chunk_pos(&ray.origin);
                            // match self.takencare.get(&chunkofthisraysorigin) {
                            //     Some(chunk) => {
                            //         implicated.insert(chunk.geo_index);
                            //     }
                            //     None => {

                            //     }
                            // }
                            implicated.insert(chunkofthisraysorigin);
                        }
                    }
                    None => {}
                }
            }

            for i in implicated {
                self.queue_rerender_with_key(i, true, true);
            }
        }

        if neighbors {
            let mut neighbs: HashSet<IVec2> = HashSet::new();

            for i in Cube::get_neighbors() {
                let thisspot = spot + *i;
                neighbs.insert(ChunkSystem::spot_to_chunk_pos(&thisspot));
            }
            for i in neighbs {
                let here = i;
                self.queue_rerender_with_key(here, user_power, light);
            }
        } else {
            self.queue_rerender(spot, user_power, light);
        }
    }

    pub fn set_block_and_queue_rerender_no_sound(
        &self,
        spot:IVec3,
        block: u32,
        neighbors: bool,
        user_power: bool,
        automata: bool,
    ) {
        let existingblock = self.blockat(spot);

        self.set_block_no_sound(spot, block, user_power);

        let blockislight = Blocks::is_light(block & Blocks::block_id_bits());
        let blockwaslight = Blocks::is_light(existingblock & Blocks::block_id_bits());

        let light = blockislight || blockwaslight;

        // if !light {
        //     check_for_intercepting.push(spot);
        // }

        if !light && !automata && !Blocks::is_transparent(block & Blocks::block_id_bits()) {
            //If not light still check if it intercepts any lights, we will need to update.

            let mut implicated = HashSet::new();
            for i in Cube::get_neighbors() {
                match self.lightmap.lock().get(&(*i + spot)) {
                    Some(k) => {
                        for ray in &k.rays {
                            let chunkofthisraysorigin = ChunkSystem::spot_to_chunk_pos(&ray.origin);
                            // match self.takencare.get(&chunkofthisraysorigin) {
                            //     Some(chunk) => {
                            //         implicated.insert(chunk.geo_index);
                            //     }
                            //     None => {

                            //     }
                            // }
                            implicated.insert(chunkofthisraysorigin);
                        }
                    }
                    None => {}
                }
            }

            for i in implicated {
                self.queue_rerender_with_key(i, true, true);
            }
        }

        if neighbors {
            let mut neighbs: HashSet<IVec2> = HashSet::new();

            for i in Cube::get_neighbors() {
                let thisspot = spot + *i;
                neighbs.insert(ChunkSystem::spot_to_chunk_pos(&thisspot));
            }
            for i in neighbs {
                let here = i;
                self.queue_rerender_with_key(here, user_power, light);
            }
        } else {
            self.queue_rerender(spot, user_power, light);
        }
    }

    pub fn set_block(&self, spot:IVec3, block: u32, user_power: bool) {
        let udm = unsafe {USERDATAMAPANDMISCMAP.as_ref().unwrap()};
        let nudm = unsafe {NONUSERDATAMAP.as_ref().unwrap()};
        match user_power {
            true => {
                //info!("Has user power, set block to {block}");
                udm.insert(spot, block);
            }
            false => {
                //info!("Non user power");
                nudm.insert(spot, block);
            }
        }
        if !self.headless {
            if block == 0 {
                let wastherebits = self.blockat(spot) & Blocks::block_id_bits();
                #[cfg(feature = "audio")]
                unsafe {
                    let _ = AUDIOPLAYER.play_next_in_series(
                        Blocks::get_place_series(wastherebits),
                        &Vec3::new(spot.x as f32, spot.y as f32, spot.z as f32),
                        &Vec3::ZERO,
                        0.5,
                    );
                }
            } else {
                #[cfg(feature = "audio")]
                unsafe {
                    let _ = AUDIOPLAYER.play_next_in_series(
                        Blocks::get_place_series(block & Blocks::block_id_bits()),
                        &Vec3::new(spot.x as f32, spot.y as f32, spot.z as f32),
                        &Vec3::ZERO,
                        0.5,
                    );
                }
            }
        }
    }

    pub fn set_block_no_sound(&self, spot:IVec3, block: u32, user_power: bool) {
        let udm = unsafe {USERDATAMAPANDMISCMAP.as_ref().unwrap()};
        let nudm = unsafe {NONUSERDATAMAP.as_ref().unwrap()};
        match user_power {
            true => {
                //info!("Has user power, set block to {block}");
                udm.insert(spot, block);
            }
            false => {
                //info!("Non user power");
                nudm.insert(spot, block);
            }
        }
    }
    pub fn move_and_rebuild(&self, index: usize, cpos:IVec2) {
        //info!("MBeing asked to move and rebuild to {} {}", cpos.x, cpos.y);

        let tc = self.takencare.clone();

        if !tc.contains_key(&cpos) {
            let chunkgeoarc = self.geobank[index].clone();

            if tc.contains_key(&chunkgeoarc.pos.lock()) {
                tc.remove(&chunkgeoarc.pos.lock());
            }

            // unsafe {
            //     CHUNKDRAWINGHERE.remove(&chunkgeoarc.pos.lock());
            // }

            // let mut chunkgeolock = chunkgeoarc.geos[num as usize].lock();
            // chunkgeolock.pos = cpos;
            // drop(chunkgeolock);

            // if num == 0 { num = 1; } else { num = 0; }

            chunkgeoarc.pos.lock().clone_from(&cpos);
            let lo = chunkgeoarc.pos.lock();

            let mut chunklock = self.chunks[index].lock();

            chunklock.pos = cpos;

            drop(chunklock);

            //info!("Chunkgeoarc pos set to {} {}", lo.x, lo.y);

            //#[cfg(feature="structures")]
            self.generate_chunk(&lo);

            let hashadlock = self.hashadinitiallightpass.lock();
            let mut light = false;
            if !hashadlock.contains_key(&cpos) {
                light = true;
            }
            drop(hashadlock);




            /*
            Ah, this is why background_rebuild_requests didnt seem to do anything
            The background rebuilds are just done on the spot, no request queue used.

             */


            self.rebuild_index(index, false, light);
        } else {
            info!("This path");
            let ind = tc.get(&cpos).unwrap().geo_index;

            let hashadlock = self.hashadinitiallightpass.lock();
            let mut light = false;
            if !hashadlock.contains_key(&cpos) {
                light = true;
            }
            drop(hashadlock);
            self.rebuild_index(ind, false, light);
        }
    }

    pub fn depropagate_light_origin(&self, origin:IVec3, imp: &mut HashSet<IVec2>) {
        //info!("Starting depropagating light origin");
        let mut stack: Vec<IVec3> = Vec::new();

        stack.push(origin);

        let lmarc = self.lightmap.clone();
        let mut lmlock = lmarc.lock();

        while !stack.is_empty() {
            let spot = stack.pop().unwrap();

            let chunkcoordoforigin = Self::spot_to_chunk_pos(&origin);

            let chunkcoordhere = Self::spot_to_chunk_pos(&spot);

            if chunkcoordoforigin != chunkcoordhere {
                imp.insert(chunkcoordhere);
            }

            match lmlock.get_mut(&spot) {
                Some(k) => {
                    let mut i = 0;
                    while i < k.rays.len() {
                        if k.rays[i].origin == origin {
                            let directions = k.rays[i].directions.clone();
                            //info!("There are {} directions", directions.len());
                            k.rays.remove(i);
                            let neighbs = Cube::get_neighbors();
                            for dir in directions {
                                stack.push(neighbs[dir as usize] + spot);
                            }
                        } else {
                            i += 1;
                        }
                    }
                }
                None => {}
            }
        }
        //info!("Got to end of depropagating light origin");
    }
    pub fn propagate_light_origin(
        &self,
        spot:IVec3,
        origin:IVec3,
        value: LightColor,
        imp: &mut HashSet<IVec2>,
    ) {
        //info!("Starting propagating light origin");
        let mut stack: Vec<(LightColor,IVec3)> = Vec::new();
        let mut visited: HashSet<IVec3> = HashSet::new();

        stack.push((value, spot));
        visited.insert(spot);

        let lmarc = self.lightmap.clone();

        let mut lmlock = lmarc.lock();

        while !stack.is_empty() {
            //info!("Stack size is now {}", stack.len());
            let n = stack.pop().unwrap();

            let blockbitshere = self.blockat(n.1);

            //info!("Got the block bits here: {}", blockbitshere);
            let blockidhere = blockbitshere & Blocks::block_id_bits();

            let goinghere = blockidhere == 0
                || Blocks::is_transparent(blockidhere)
                || Blocks::is_semi_transparent(blockidhere)
                || n.1 == origin;

            //info!("Goinghere: {}", goinghere);
            if !goinghere {
                let inner_light_seg;

                match lmlock.get_mut(&n.1) {
                    Some(k2) => inner_light_seg = k2,
                    None => {
                        lmlock.insert(n.1, LightSegment { rays: ArrayVec::new() });
                        inner_light_seg = lmlock.get_mut(&n.1).unwrap();
                    }
                }

                let _my_ray_here = match inner_light_seg.rays.iter_mut().find(|r| r.origin == origin)
                {
                    Some(k) => {
                        if k.value.x < n.0.x {
                            k.value.x = n.0.x;
                        }
                        if k.value.y < n.0.y {
                            k.value.y = n.0.y;
                        }
                        if k.value.z < n.0.z {
                            k.value.z = n.0.z;
                        }
                        k
                    }
                    None => {
                        match inner_light_seg.rays.try_push(LightRay {
                            value: n.0,
                            origin,
                            directions: Vec::new(),
                        }) {
                            Ok(_) => {

                            }
                            Err(_) => {

                            }
                        };
                        inner_light_seg.rays.last_mut().unwrap()
                    }
                };
                // dropping a reference does nothing
                // drop(my_ray_here);
                // drop(inner_light_seg);
            } else {
                let chunkcoordoforigin = Self::spot_to_chunk_pos(&origin);
                let chunkcoordhere = Self::spot_to_chunk_pos(&n.1);

                //info!("Chunk coord of origin: {:?}", chunkcoordoforigin);
                //info!("Chunk coord here: {:?}", chunkcoordhere);

                if chunkcoordoforigin != chunkcoordhere {
                    imp.insert(chunkcoordhere);
                }

                let inner_light_seg;

                match lmlock.get_mut(&n.1) {
                    Some(k2) => inner_light_seg = k2,
                    None => {
                        lmlock.insert(n.1, LightSegment { rays: ArrayVec::new() });
                        inner_light_seg = lmlock.get_mut(&n.1).unwrap();
                    }
                }

                let _my_ray_here = match inner_light_seg.rays.iter_mut().find(|r| r.origin == origin)
                {
                    Some(k) => {
                        if k.value.x < n.0.x {
                            k.value.x = n.0.x;
                        }
                        if k.value.y < n.0.y {
                            k.value.y = n.0.y;
                        }
                        if k.value.z < n.0.z {
                            k.value.z = n.0.z;
                        }
                        k
                    }
                    None => {
                        match inner_light_seg.rays.try_push(LightRay {
                            value: n.0,
                            origin,
                            directions: Vec::new(),
                        }) {
                            Ok(_) => {

                            }
                            Err(_) => {

                            }
                        };
                        inner_light_seg.rays.last_mut().unwrap()
                    }
                };
                // dropping a reference does nothing
                // drop(my_ray_here);
                // drop(inner_light_seg);

                if n.0.x > 1 || n.0.y > 1 || n.0.z > 1 {
                    let neighbs = Cube::get_neighbors();
                    for (index, neigh) in neighbs.iter().enumerate() {
                        let next = n.1 + *neigh;

                        //info!("Checking neighbor at index {}: {:?}", index, next);

                        let existing_new_light_seg;

                        let mut existing_next_value = LightColor::ZERO;

                        match lmlock.get(&next) {
                            Some(k2) => {
                                existing_new_light_seg = k2;

                                for (_ind, ray) in
                                    &mut existing_new_light_seg.rays.iter().enumerate()
                                {
                                    if ray.origin == origin {
                                        existing_next_value = ray.value; //Either this or we'll consider it zero
                                    }
                                }
                            }
                            None => {}
                        }

                        let reducedvalue = LightColor::new(
                            (n.0.x as i32 - ((n.0.x as f32 / 3.0) as i32).max(1)).max(0) as u16,
                            (n.0.y as i32 - ((n.0.y as f32 / 3.0) as i32).max(1)).max(0) as u16,
                            (n.0.z as i32 - ((n.0.z as f32 / 3.0) as i32).max(1)).max(0) as u16,
                        );

                        if !visited.contains(&next)
                            || existing_next_value.x < reducedvalue.x
                            || existing_next_value.y < reducedvalue.y
                            || existing_next_value.z < reducedvalue.z
                        {
                            //info!("Pushing next onto stack: {:?}", next);

                            stack.push((reducedvalue, next));
                            visited.insert(next);

                            let inner_light_seg = lmlock.get_mut(&n.1).unwrap();

                            match inner_light_seg
                                .rays
                                .iter_mut()
                                .find(|r| r.origin == origin) {
                                    Some(my_ray_here) => {
                                        if !my_ray_here
                                            .directions
                                            .contains(&CubeSide::from_primitive(index))
                                        {
                                            my_ray_here.directions.push(CubeSide::from_primitive(index));
                                        }
                                    }
                                    None => {
                                        
                                    }
                                };

                            
                        }
                    }
                }
            }
        }
        //info!("Got to end of propagating light origin");
    }

    pub fn lightpass_on_chunk(&self, pos:IVec2) {
        //println!("Doing lightpass on chunk! {} {}", pos.x, pos.y);

        let hashadarc = self.hashadinitiallightpass.clone();
        let mut hashadlock = hashadarc.lock();

        hashadlock.insert(pos, true);

        drop(hashadlock);

        let mut implicated: HashSet<IVec2> = HashSet::new();

        let mut lightsources: HashMap<IVec3, u32> = HashMap::new();
        //let mut lightsources: HashSet<(IVec3, u32)> = HashSet::new();

        let mut existingsources: HashSet<IVec3> = HashSet::new();

        let mut highest = 0;

        let lmarc = self.lightmap.clone();

        for x in 0..CH_W {
            for z in 0..CH_W {
                for y in 0..CH_H {
                    let blockcoord = IVec3::new(pos.x * CH_W + x, y, pos.y * CH_W + z);
                    let lmlock = lmarc.lock();
                    match lmlock.get(&blockcoord) {
                        Some(k) => {
                            for ray in &k.rays {
                                let chunkcoord_of_origin = Self::spot_to_chunk_pos(&ray.origin);

                                if chunkcoord_of_origin == pos {
                                    let originweremoving = ray.origin;

                                    existingsources.insert(ray.origin);

                                    let id = self.blockat(originweremoving);
                                    let cleanid = id & Blocks::block_id_bits();
                                    
                                    

                                    if Blocks::is_light(cleanid) {

                                        //if there are no neighbor light sources
                                        if !lightsources.contains_key(&(originweremoving + IVec3::new(1, 0, 0)))
                                        && !lightsources.contains_key(&(originweremoving + IVec3::new(-1, 0, 0)))
                                        && !lightsources.contains_key(&(originweremoving + IVec3::new(0, 1, 0)))
                                        && !lightsources.contains_key(&(originweremoving + IVec3::new(0, -1, 0)))
                                        && !lightsources.contains_key(&(originweremoving + IVec3::new(0, 0, 1)))
                                        && !lightsources.contains_key(&(originweremoving + IVec3::new(0, 0, -1)))
                                        {
                                            if blockcoord.y > highest {
                                                highest = blockcoord.y;
                                            }
                                            lightsources.insert(originweremoving, cleanid);
                                        }
                                    }
                                }
                            }
                        }
                        None => {}
                    }

                    let id = self.blockat(blockcoord);
                    let cleanid = id & Blocks::block_id_bits();
                    if Blocks::is_light(cleanid) {
                        //if there are no neighbor light sources
                        if !lightsources.contains_key(&(blockcoord + IVec3::new(1, 0, 0)))
                        && !lightsources.contains_key(&(blockcoord + IVec3::new(-1, 0, 0)))
                        && !lightsources.contains_key(&(blockcoord + IVec3::new(0, 1, 0)))
                        && !lightsources.contains_key(&(blockcoord + IVec3::new(0, -1, 0)))
                        && !lightsources.contains_key(&(blockcoord + IVec3::new(0, 0, 1)))
                        && !lightsources.contains_key(&(blockcoord + IVec3::new(0, 0, -1)))
                        {
                            if blockcoord.y > highest {
                                highest = blockcoord.y;
                            }
                            lightsources.insert(blockcoord, cleanid);
                        }
                    }
                }
            }
        }

        if lightsources.len() > MAX_LIGHTS {
            let key = bevy::math::IVec2::new(pos.x, pos.y);
            if !QUEUED_FOR_EVERIS.contains_key(&key) {
                QUEUED_FOR_EVERIS.insert(key, true);
                unsafe { EVERIS_KILLED_YET = false };
                unsafe { EVERIS_VISIT_QUEUE.push(EverisVisitSpot{ spot:key, highestlight: highest}) };
            }
            
        }

        for source in existingsources {
            self.depropagate_light_origin(source, &mut implicated);
        }

        for (source, id) in lightsources {
            self.propagate_light_origin(
                source,
                source,
                Blocks::get_light_color(id),
                &mut implicated,
            );
        }

        //info!("Implicated number: {}", implicated.len());

        for c in implicated.iter() {
            match self.takencare.get(&c) {
                Some(cf) => {
                    self.user_rebuild_requests.push(cf.geo_index);
                }
                None => {}
            }
        }

        //info!("Got to end of lightpass");
    }

    pub fn rebuild_index(&self, index: usize, user_power: bool, light: bool) {
        self._rebuild_index(index, user_power, light);
    }
    pub fn _rebuild_index(&self, index: usize, user_power: bool, light: bool) {
        //info!("Rebuilding!");
        let chunkarc = self.chunks[index].clone();
        let mut chunklock = chunkarc.lock();
        chunklock.used = true;

        let chunklock = chunklock.clone();

        static mut RNG: Lazy<StdRng> = Lazy::new(|| StdRng::from_entropy());

        if light {
            self.lightpass_on_chunk(chunklock.pos);
        }

        let doorbottomuvs = DoorInfo::get_door_uvs(TextureFace::new(11, 0));
        let doortopuvs = DoorInfo::get_door_uvs(TextureFace::new(11, 1));

        let geobankarc = self.geobank[index].clone();
        // if num == 0 { num = 1; } else { num = 0; }

        geobankarc.clear();

        let mut memo: HashMap<IVec3, u32> = HashMap::new();

        let mut data32 = geobankarc.data32.lock();
        let mut data8 = geobankarc.data8.lock();
        let mut tdata32 = geobankarc.tdata32.lock();
        let mut tdata8 = geobankarc.tdata8.lock();

        let mut vdata = geobankarc.vdata.lock();
        let mut uvdata = geobankarc.uvdata.lock();

        let mut wvdata = geobankarc.wvdata.lock();
        let mut wuvdata = geobankarc.wuvdata.lock();

        let mut data8rgb = geobankarc.data8rgb.lock();
        let mut tdata8rgb = geobankarc.tdata8rgb.lock();

        let mut weatherstoptops: HashMap<IVec2, i32> = HashMap::new();
        let mut tops: HashMap<IVec2, i32> = HashMap::new();

        for i in 0..CH_W {
            for k in 0..CH_W {
                let mut hit_block: bool;
                for j in (0..CH_H).rev() {
                    let spot =IVec3 {
                        x: (chunklock.pos.x * CH_W) + i,
                        y: j,
                        z: (chunklock.pos.y * CH_W) + k,
                    };
                    let combined = self.blockatmemo(spot, &mut memo);
                    let block = combined & Blocks::block_id_bits();
                    let flags = combined & Blocks::block_flag_bits();
                    // if self.justcollisionmap.contains_key(&spot) {
                    //     for (indie, neigh) in Cube::get_neighbors().iter().enumerate() {
                    //         let cubeside = CubeSide::from_primitive(indie);
                    //         let side = Cube::get_side(cubeside);
                    //                 let mut packed32: [u32; 6] = [0, 0, 0, 0, 0, 0];
                    //                 let mut packed8: [u8; 6] = [0, 0, 0, 0, 0, 0];

                    //                 let texcoord = Blocks::get_tex_coords(1, cubeside);
                    //                 for (ind, v) in side.chunks(4).enumerate() {

                    //                     static AMB_CHANGES: [u8; 4] = [
                    //                         0, 3, 6, 10
                    //                     ];

                    //                     let amb_spots: &[IVec3; 3] = Cube::get_amb_occul_spots(cubeside, ind as u8);

                    //                     let amb_change = amb_spots.iter()
                    //                                               .map(|vec| self.blockatmemo(*vec + spot, &mut memo))
                    //                                               .filter(|&result| result != 0)
                    //                                               .count();

                    //                     let base_light: i32 = v[3] as i32 - AMB_CHANGES[amb_change] as i32; // Perform calculations as i32
                    //                     let adjusted_light: i32 = if hit_block {
                    //                         base_light - 3
                    //                     } else {
                    //                         base_light
                    //                     };
                    //                     let clamped_light: u8 = adjusted_light.clamp(0, 15) as u8; // Clamp in i32 context, then cast to u8

                    //                     let pack = PackedVertex::pack(
                    //                         i as u8 + v[0],
                    //                         j as u8 + v[1],
                    //                         k as u8 + v[2],
                    //                         ind as u8,
                    //                         clamped_light,
                    //                         0,
                    //                         texcoord.0,
                    //                         texcoord.1,
                    //                     );
                    //                     packed32[ind] = pack.0;
                    //                     packed8[ind] = pack.1;
                    //                 }

                    //                 tdata32.extend_from_slice(packed32.as_slice());
                    //                 tdata8.extend_from_slice(packed8.as_slice());
                    //     }
                    // }
                    if block != 0 {
                        let isgrass = if block == 3 { 1u8 } else { 0u8 };

                        if !weatherstoptops.contains_key(&IVec2 { x: i, y: k }) {
                            weatherstoptops.insert(IVec2 { x: i, y: k }, spot.y);
                        }

                        match block {
                            19 => {
                                let direction = Blocks::get_direction_bits(flags);
                                let open = DoorInfo::get_door_open_bit(flags);
                                let opposite = DoorInfo::get_opposite_door_bits(flags);

                                let mut modelindex: i32;
                                if opposite == 1 {
                                    modelindex = direction as i32 - open as i32;
                                    if modelindex < 0 {
                                        modelindex = 3;
                                    }
                                } else {
                                    modelindex = (direction as i32 + open as i32) % 4;
                                }

                                let doortop = DoorInfo::get_door_top_bit(flags);

                                let _blocklightval = 0.0;

                                let lmlock = self.lightmap.lock();
                                let blocklighthere = match lmlock.get(&spot) {
                                    Some(k) => k.sum(),
                                    None => LightColor::ZERO,
                                };

                                let packedrgb = PackedVertex::pack_rgb(
                                    blocklighthere.x,
                                    blocklighthere.y,
                                    blocklighthere.z,
                                );

                                let prgb: u32 =
                                    0b0000_0000_0000_0000_0000_0000_0000_0000 | (packedrgb) as u32;
                                drop(lmlock);

                                for vert in
                                    DoorInfo::door_model_from_index(modelindex as usize).chunks(5)
                                {
                                    vdata.extend_from_slice(&[
                                        vert[0] + spot.x as f32,
                                        vert[1] + spot.y as f32,
                                        vert[2] + spot.z as f32,
                                        f32::from_bits(prgb),
                                        vert[4],
                                    ])
                                }

                                if doortop != 0 {
                                    uvdata.extend_from_slice(&doortopuvs);
                                } else {
                                    uvdata.extend_from_slice(&doorbottomuvs);
                                }
                            }
                            20 => {
                                let direction = Blocks::get_direction_bits(flags);

                                let modelindex: i32 = direction as i32;

                                let _blocklightval = 0.0;

                                let lmlock = self.lightmap.lock();
                                let blocklighthere = match lmlock.get(&spot) {
                                    Some(k) => k.sum(),
                                    None => LightColor::ZERO,
                                };

                                let packedrgb = PackedVertex::pack_rgb(
                                    blocklighthere.x,
                                    blocklighthere.y,
                                    blocklighthere.z,
                                );

                                let prgb: u32 =
                                    0b0000_0000_0000_0000_0000_0000_0000_0000 | (packedrgb) as u32;
                                drop(lmlock);

                                for vert in
                                    LadderInfo::ladder_model_from_index(modelindex as usize).chunks(5)
                                {
                                    vdata.extend_from_slice(&[
                                        vert[0] + spot.x as f32,
                                        vert[1] + spot.y as f32,
                                        vert[2] + spot.z as f32,
                                        f32::from_bits(prgb),
                                        vert[4],
                                    ])
                                }

                                uvdata.extend_from_slice(&LadderInfo::get_ladder_uvs());
                            }
                            80 => {
                                let direction = Blocks::get_direction_bits(flags);

                                let modelindex: i32 = direction as i32;

                                let _blocklightval = 0.0;

                                let lmlock = self.lightmap.lock();
                                let blocklighthere = match lmlock.get(&spot) {
                                    Some(k) => k.sum(),
                                    None => LightColor::ZERO,
                                };

                                let packedrgb = PackedVertex::pack_rgb(
                                    blocklighthere.x,
                                    blocklighthere.y,
                                    blocklighthere.z,
                                );

                                let prgb: u32 =
                                    0b0000_0000_0000_0000_0000_0000_0000_0000 | (packedrgb) as u32;

                                    let innerrgb = PackedVertex::pack_rgb(
                                        14,
                                        14,
                                        14,
                                    );
    
                                    let innerrgb: u32 =
                                        0b0000_0000_0000_0000_0000_0000_0000_0000 | (innerrgb) as u32;


                                drop(lmlock);

                                for vert in
                                    JackolanternInfo::jackolantern_model_from_index(modelindex as usize).chunks(5)
                                {
                                    vdata.extend_from_slice(&[
                                        vert[0] + spot.x as f32,
                                        vert[1] + spot.y as f32,
                                        vert[2] + spot.z as f32,
                                        f32::from_bits(prgb),
                                        vert[4],
                                    ])
                                }

                                uvdata.extend_from_slice(&JackolanternInfo::get_jackolantern_uvs());


                                for vert in
                                    JackolanternInfo::inner_bit_from_index(modelindex as usize).chunks(5)
                                {
                                    vdata.extend_from_slice(&[
                                        vert[0] + spot.x as f32,
                                        vert[1] + spot.y as f32,
                                        vert[2] + spot.z as f32,
                                        f32::from_bits(innerrgb),
                                        vert[4],
                                    ])
                                }

                                uvdata.extend_from_slice(&JackolanternInfo::inside_part_uvs());

                            }
                            45 => {
                                let direction = Blocks::get_direction_bits(flags);

                                let modelindex: i32 = direction as i32;

                                let _blocklightval = 0.0;

                                let lmlock = self.lightmap.lock();
                                let blocklighthere = match lmlock.get(&spot) {
                                    Some(k) => k.sum(),
                                    None => LightColor::ZERO,
                                };

                                let packedrgb = PackedVertex::pack_rgb(
                                    blocklighthere.x,
                                    blocklighthere.y,
                                    blocklighthere.z,
                                );

                                let prgb: u32 =
                                    0b0000_0000_0000_0000_0000_0000_0000_0000 | (packedrgb) as u32;
                                drop(lmlock);

                                for vert in ConveyorInfo::conveyor_model_from_index(modelindex as usize)
                                    .chunks(5)
                                {
                                    vdata.extend_from_slice(&[
                                        vert[0] + spot.x as f32,
                                        vert[1] + spot.y as f32,
                                        vert[2] + spot.z as f32,
                                        f32::from_bits(prgb),
                                        vert[4],
                                    ])
                                }

                                uvdata.extend_from_slice(&ConveyorInfo::get_conveyor_uvs());
                            }
                            52 => {
                                
                                let _blocklightval = 0.0;

                                let lmlock = self.lightmap.lock();
                                let blocklighthere = match lmlock.get(&spot) {
                                    Some(k) => k.sum(),
                                    None => LightColor::ZERO,
                                };

                                let packedrgb = PackedVertex::pack_rgb(
                                    blocklighthere.x,
                                    blocklighthere.y,
                                    blocklighthere.z,
                                );

                                let prgb: u32 =
                                    0b0000_0000_0000_0000_0000_0000_0000_0000 | (packedrgb) as u32;
                                drop(lmlock);

                                for vert in AwdsInfo::awds_model_from_index(0)
                                    .chunks(5)
                                {
                                    vdata.extend_from_slice(&[
                                        vert[0] + spot.x as f32,
                                        vert[1] + spot.y as f32,
                                        vert[2] + spot.z as f32,
                                        f32::from_bits(prgb),
                                        vert[4],
                                    ])
                                }

                                uvdata.extend_from_slice(&AwdsInfo::get_awds_uvs());
                            }
                            63 => {
                                //fence
                                let _blocklightval = 0.0;

                                let lmlock = self.lightmap.lock();
                                let blocklighthere = match lmlock.get(&spot) {
                                    Some(k) => k.sum(),
                                    None => LightColor::ZERO,
                                };

                                

                                let packedrgb = PackedVertex::pack_rgb(
                                    blocklighthere.x,
                                    blocklighthere.y,
                                    blocklighthere.z,
                                );

                                let prgb: u32 =
                                    0b0000_0000_0000_0000_0000_0000_0000_0000 | (packedrgb) as u32;
                                drop(lmlock);

                                for vert in FenceInfo::fence_model_from_combined(combined)
                                    .chunks(5)
                                {
                                    vdata.extend_from_slice(&[
                                        vert[0] + spot.x as f32,
                                        vert[1] + spot.y as f32,
                                        vert[2] + spot.z as f32,
                                        f32::from_bits(prgb),
                                        vert[4],
                                    ])
                                }

                                uvdata.extend_from_slice(&FenceInfo::fence_uvs_from_combined(combined));
                            }
                            49 => {
                                let direction = Blocks::get_direction_bits(flags);

                                let modelindex: i32 = direction as i32;

                                let _blocklightval = 0.0;

                                let lmlock = self.lightmap.lock();
                                let blocklighthere = match lmlock.get(&spot) {
                                    Some(k) => k.sum(),
                                    None => LightColor::ZERO,
                                };

                                let packedrgb = PackedVertex::pack_rgb(
                                    blocklighthere.x,
                                    blocklighthere.y,
                                    blocklighthere.z,
                                );

                                let prgb: u32 =
                                    0b0000_0000_0000_0000_0000_0000_0000_0000 | (packedrgb) as u32;
                                drop(lmlock);

                                for vert in
                                    TorchInfo::torch_model_from_index(modelindex as usize).chunks(5)
                                {
                                    vdata.extend_from_slice(&[
                                        vert[0] + spot.x as f32,
                                        vert[1] + spot.y as f32,
                                        vert[2] + spot.z as f32,
                                        f32::from_bits(prgb),
                                        vert[4],
                                    ])
                                }

                                uvdata.extend_from_slice(&TorchInfo::get_torch_uvs());
                            }
                            21 => {
                                let direction = Blocks::get_direction_bits(flags);

                                let modelindex: i32 = direction as i32;

                                let _blocklightval = 0.0;

                                let lmlock = self.lightmap.lock();
                                let blocklighthere = match lmlock.get(&spot) {
                                    Some(k) => k.sum(),
                                    None => LightColor::ZERO,
                                };

                                let packedrgb = PackedVertex::pack_rgb(
                                    blocklighthere.x,
                                    blocklighthere.y,
                                    blocklighthere.z,
                                );

                                let prgb: u32 =
                                    0b0000_0000_0000_0000_0000_0000_0000_0000 | (packedrgb) as u32;
                                drop(lmlock);

                                for vert in
                                    ChestInfo::chest_model_from_index(modelindex as usize).chunks(5)
                                {
                                    vdata.extend_from_slice(&[
                                        vert[0] + spot.x as f32,
                                        vert[1] + spot.y as f32,
                                        vert[2] + spot.z as f32,
                                        f32::from_bits(prgb),
                                        vert[4],
                                    ])
                                }

                                uvdata.extend_from_slice(&ChestInfo::get_chest_uvs());
                            }
                            23 => {
                                let modelindex: i32 = 0;

                                let _blocklightval = 0.0;

                                let lmlock = self.lightmap.lock();
                                let blocklighthere = match lmlock.get(&spot) {
                                    Some(k) => k.sum(),
                                    None => LightColor::ZERO,
                                };

                                let packedrgb = PackedVertex::pack_rgb(
                                    blocklighthere.x,
                                    blocklighthere.y,
                                    blocklighthere.z,
                                );

                                let prgb: u32 =
                                    0b0000_0000_0000_0000_0000_0000_0000_0000 | (packedrgb) as u32;
                                drop(lmlock);

                                for vert in
                                    TallGrassInfo::tallgrass_model_from_index(modelindex as usize)
                                        .chunks(5)
                                {
                                    vdata.extend_from_slice(&[
                                        vert[0] + spot.x as f32,
                                        vert[1] + spot.y as f32,
                                        vert[2] + spot.z as f32,
                                        f32::from_bits(prgb),
                                        vert[4],
                                    ])
                                }

                                uvdata.extend_from_slice(&TallGrassInfo::get_tallgrass_uvs());
                            }
                            31 => {
                                let direction = Blocks::get_direction_bits(flags);

                                let modelindex: i32 = direction as i32;

                                let _blocklightval = 0.0;

                                let lmlock = self.lightmap.lock();
                                let blocklighthere = match lmlock.get(&spot) {
                                    Some(k) => k.sum(),
                                    None => LightColor::ZERO,
                                };

                                let packedrgb = PackedVertex::pack_rgb(
                                    blocklighthere.x,
                                    blocklighthere.y,
                                    blocklighthere.z,
                                );

                                let prgb: u32 =
                                    0b0000_0000_0000_0000_0000_0000_0000_0000 | (packedrgb) as u32;
                                drop(lmlock);

                                for vert in
                                    CraftTableInfo::craft_table_model_from_index(modelindex as usize)
                                        .chunks(5)
                                {
                                    vdata.extend_from_slice(&[
                                        vert[0] + spot.x as f32,
                                        vert[1] + spot.y as f32,
                                        vert[2] + spot.z as f32,
                                        f32::from_bits(prgb),
                                        vert[4],
                                    ])
                                }

                                uvdata.extend_from_slice(&CraftTableInfo::get_craft_table_uvs());
                            }
                            _ =>  {
                                {
                                    if Blocks::is_transparent(block) || Blocks::is_semi_transparent(block) {
                                        for (indie, neigh) in Cube::get_neighbors().iter().enumerate() {
                                            let neighspot = spot + *neigh;
                                            let neigh_block = self.blockatmemo(neighspot, &mut memo)
                                                & Blocks::block_id_bits();
                                            let cubeside = CubeSide::from_primitive(indie);
                                            let neigh_semi_trans = Blocks::is_semi_transparent(neigh_block);
                                            let water_bordering_transparent = block == 2
                                                && neigh_block != 2
                                                && Blocks::is_transparent(neigh_block);
        
                                            let lmlock = self.lightmap.lock();
        
                                            let blocklighthere = match lmlock.get(&neighspot) {
                                                Some(k) => k.sum(),
                                                None => LightColor::ZERO,
                                            };
        
                                            // if blocklighthere != 0 {
                                            //     info!("Block light here: {}", blocklighthere);
                                            // }
                                            drop(lmlock);
        
                                            hit_block = match tops.get(&IVec2 {
                                                x: i + neigh.x,
                                                y: k + neigh.z,
                                            }) {
                                                Some(t) => *t > j + neigh.y,
                                                None => false,
                                            };
        
                                            if neigh_block == 0
                                                || neigh_semi_trans
                                                || water_bordering_transparent
                                            {
                                                let side = Cube::get_side(cubeside);
                                                let mut packed32: [u32; 6] = [0, 0, 0, 0, 0, 0];
                                                let mut packed8: [u8; 6] = [0, 0, 0, 0, 0, 0];
                                                let mut packed8rgb: [u16; 6] = [0, 0, 0, 0, 0, 0];
        
                                                let texcoord = Blocks::get_tex_coords(block, cubeside);
                                                for (ind, v) in side.chunks(4).enumerate() {
                                                    static AMB_CHANGES: [u8; 4] = [0, 3, 6, 10];
        
                                                    let amb_spots: &[IVec3; 3] =
                                                        Cube::get_amb_occul_spots(cubeside, ind as u8);
        
                                                    let amb_change = amb_spots
                                                        .iter()
                                                        .map(|vec| self.blockatmemo(*vec + spot, &mut memo))
                                                        .filter(|&result| {
                                                            let res = result & Blocks::block_id_bits();
                                                             res != 0 && !Blocks::is_semi_transparent(res) 
                                                            })
                                                        .count();
        
                                                    let base_light: i32 =
                                                        v[3] as i32 - AMB_CHANGES[amb_change] as i32; 
                                                    let adjusted_light: i32 = if hit_block {
                                                        base_light - 3
                                                    } else {
                                                        base_light
                                                    };
                                                    let clamped_light: u8 =
                                                        adjusted_light.clamp(0, 15) as u8; 
        
                                                    let pack = PackedVertex::pack(
                                                        i as u8 + v[0],
                                                        j as u8 + v[1],
                                                        k as u8 + v[2],
                                                        ind as u8,
                                                        clamped_light,
                                                        isgrass, 
                                                        texcoord.0,
                                                        texcoord.1,
                                                    );
        
                                                    let packedcolor = PackedVertex::pack_rgb(
                                                        blocklighthere.x,
                                                        blocklighthere.y,
                                                        blocklighthere.z,
                                                    );
        
                                                    packed32[ind] = pack.0;
                                                    packed8[ind] = pack.1;
                                                    packed8rgb[ind] = packedcolor;
                                                }
        
                                                if block == 2 && cubeside == CubeSide::TOP {
                                                    let texcoord = Blocks::get_tex_coords(block, cubeside);
                                                    for (ind, v) in side.chunks(4).enumerate().rev() {
                                                        static AMB_CHANGES: [u8; 4] = [0, 3, 6, 10];
        
                                                        let amb_change = 0;
        
                                                        let base_light: i32 =
                                                            v[3] as i32 - AMB_CHANGES[amb_change] as i32; // Perform calculations as i32
                                                        let adjusted_light: i32 = if hit_block {
                                                            base_light - 3
                                                        } else {
                                                            base_light
                                                        };
                                                        let clamped_light: u8 =
                                                            adjusted_light.clamp(0, 15) as u8; // Clamp in i32 context, then cast to u8
        
                                                        let pack = PackedVertex::pack(
                                                            i as u8 + v[0],
                                                            j as u8 + v[1],
                                                            k as u8 + v[2],
                                                            ind as u8,
                                                            clamped_light,
                                                            isgrass,
                                                            texcoord.0,
                                                            texcoord.1,
                                                        );
        
                                                        let packedcolor = PackedVertex::pack_rgb(
                                                            blocklighthere.x,
                                                            blocklighthere.y,
                                                            blocklighthere.z,
                                                        );
        
                                                        tdata32.push(pack.0);
                                                        tdata8.push(pack.1);
                                                        tdata8rgb.push(packedcolor);
                                                    }
                                                }
        
                                                tdata32.extend_from_slice(packed32.as_slice());
                                                tdata8.extend_from_slice(packed8.as_slice());
                                                tdata8rgb.extend_from_slice(packed8rgb.as_slice());
                                            } else {
                                                tops.insert(
                                                   IVec2 {
                                                        x: i + neigh.x,
                                                        y: k + neigh.z,
                                                    },
                                                    j + neigh.y,
                                                );
                                            }
                                        }
                                    } else {
                                        for (indie, neigh) in Cube::get_neighbors().iter().enumerate() {
                                            let neighspot = spot + *neigh;
                                            let neigh_block = self.blockatmemo(neighspot, &mut memo)
                                                & Blocks::block_id_bits();
        
                                            let cubeside = CubeSide::from_primitive(indie);
                                            let neighbor_transparent = Blocks::is_transparent(neigh_block)
                                                || Blocks::is_semi_transparent(neigh_block);
        
                                            hit_block = match tops.get(&IVec2 {
                                                x: i + neigh.x,
                                                y: k + neigh.z,
                                            }) {
                                                Some(t) => *t > j + neigh.y,
                                                None => false,
                                            };
        
                                            let lmlock = self.lightmap.lock();
        
                                            let blocklighthere = match lmlock.get(&neighspot) {
                                                Some(k) => k.sum(),
                                                None => LightColor::ZERO,
                                            };
                                            // if blocklighthere != 0 {
                                            //     info!("Block light here: {}", blocklighthere);
                                            // }
        
                                            drop(lmlock);
        
                                            if neigh_block == 0 || neighbor_transparent {
                                                let side = Cube::get_side(cubeside);
                                                let mut packed32: [u32; 6] = [0, 0, 0, 0, 0, 0];
                                                let mut packed8: [u8; 6] = [0, 0, 0, 0, 0, 0];
                                                let mut packed8rgb: [u16; 6] = [0, 0, 0, 0, 0, 0];
        
                                                let texcoord = Blocks::get_tex_coords(block, cubeside);
                                                for (ind, v) in side.chunks(4).enumerate() {
                                                    static AMB_CHANGES: [u8; 4] = [0, 3, 6, 10];
        
                                                    let amb_spots: &[IVec3; 3] =
                                                        Cube::get_amb_occul_spots(cubeside, ind as u8);
        
                                                    let amb_change = amb_spots
                                                        .iter()
                                                        .map(|vec| self.blockatmemo(*vec + spot, &mut memo))
                                                        .filter(|&result| {
                                                            let res = result & Blocks::block_id_bits();
                                                             res != 0 && !Blocks::is_semi_transparent(res) 
                                                            })
                                                        .count();
        
                                                    let base_light: i32 =
                                                        v[3] as i32 - AMB_CHANGES[amb_change] as i32; // Perform calculations as i32
                                                    let adjusted_light: i32 = if hit_block {
                                                        base_light - 3
                                                    } else {
                                                        base_light
                                                    };
                                                    let clamped_light: u8 =
                                                        adjusted_light.clamp(0, 15) as u8; // Clamp in i32 context, then cast to u8
        
                                                    let pack = PackedVertex::pack(
                                                        i as u8 + v[0],
                                                        j as u8 + v[1],
                                                        k as u8 + v[2],
                                                        ind as u8,
                                                        clamped_light,
                                                        isgrass, //TEMPORARY UNUSED
                                                        texcoord.0,
                                                        texcoord.1,
                                                    );
                                                    let packedcolor = PackedVertex::pack_rgb_w_norm(
                                                        blocklighthere.x,
                                                        blocklighthere.y,
                                                        blocklighthere.z,
                                                        cubeside as u16
                                                    );
        
                                                    packed32[ind] = pack.0;
                                                    packed8[ind] = pack.1;
                                                    packed8rgb[ind] = packedcolor;
                                                }
        
                                                data32.extend_from_slice(packed32.as_slice());
                                                data8.extend_from_slice(packed8.as_slice());
                                                data8rgb.extend_from_slice(packed8rgb.as_slice());
        
                                                if Blocks::is_semi_transparent(neigh_block) {
                                                    tops.insert(
                                                       IVec2 {
                                                            x: i + neigh.x,
                                                            y: k + neigh.z,
                                                        },
                                                        j + neigh.y,
                                                    );
                                                }
                                            } else {
                                                tops.insert(
                                                   IVec2 {
                                                        x: i + neigh.x,
                                                        y: k + neigh.z,
                                                    },
                                                    j + neigh.y,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                //BEGIN ADD WEATHER PANES FOR RAIN/SNOW/ETC, nOT EVERY BLOCK

                let topy = match weatherstoptops.get(&IVec2 { x: i, y: k }) {
                    Some(top) => {
                        //println!("Found top {}", *top);
                        *top
                    }
                    None => 0,
                };

                if unsafe { RNG.gen_range(0..=10) > 9 } && topy < 115 {
                    //let mut rng = StdRng::from_entropy();

                    //spot xz top
                    let spoint: IVec3 =IVec3 {
                        x: (chunklock.pos.x * CH_W) + i,
                        y: topy,
                        z: (chunklock.pos.y * CH_W) + k,
                    };

                    //spot xz top
                    let spo = Vec3 {
                        x: (chunklock.pos.x * CH_W) as f32 + i as f32,
                        y: topy as f32,
                        z: (chunklock.pos.y * CH_W) as f32 + k as f32,
                    };

                    //LET BLOCKLIGHT AT TOP
                    //FADE BLOCKLIGHT WITH HIGHER Y IN SHADER
                    //OR JUST SET TO <AMB> AT TOP SO IT FADES UP NATURALLY FROM VERTEX SHADING

                    let lmlock = self.lightmap.lock();
                    let blocklighthere = match lmlock.get(&(spoint + IVec3::new(0, 1, 0))) {
                        Some(k) => k.sum(),
                        None => LightColor::ZERO,
                    };

                    let packedrgb = PackedVertex::pack_rgb(
                        blocklighthere.x,
                        blocklighthere.y,
                        blocklighthere.z,
                    );

                    let prgb: u32 = 0b0000_0000_0000_0000_0000_0000_0000_0000 | (packedrgb) as u32;
                    drop(lmlock);

                    let lightf32 = f32::from_bits(prgb);

                    let face = TextureFace::new(15, 0);

                    wvdata.extend_from_slice(&[
                        spo.x as f32 - 1.0,
                        spo.y as f32,
                        spo.z as f32,
                        lightf32, /*BLOCKLIGHT */
                        14.0,
                        spo.x as f32 + 2.0,
                        spo.y as f32,
                        spo.z as f32 + 2.0,
                        lightf32, /*BLOCKLIGHT */
                        14.0,
                        spo.x as f32 + 2.0,
                        115.0,
                        spo.z as f32 + 2.0,
                        0.0, /*BLOCKLIGHT */
                        14.0,
                        spo.x as f32 + 2.0,
                        115.0,
                        spo.z as f32 + 2.0,
                        0.0, /*BLOCKLIGHT */
                        14.0,
                        spo.x as f32 - 1.0,
                        115.0,
                        spo.z as f32,
                        0.0, /*BLOCKLIGHT */
                        14.0,
                        spo.x as f32 - 1.0,
                        spo.y as f32,
                        spo.z as f32,
                        lightf32, /*BLOCKLIGHT */
                        14.0,
                        spo.x as f32 + 2.0,
                        spo.y as f32,
                        spo.z as f32 + 2.0,
                        lightf32, /*BLOCKLIGHT */
                        14.0,
                        spo.x as f32 - 1.0,
                        spo.y as f32,
                        spo.z as f32,
                        lightf32, /*BLOCKLIGHT */
                        14.0,
                        spo.x as f32 - 1.0,
                        115.0,
                        spo.z as f32,
                        0.0, /*BLOCKLIGHT */
                        14.0,
                        spo.x as f32 - 1.0,
                        115.0,
                        spo.z as f32,
                        0.0, /*BLOCKLIGHT */
                        14.0,
                        spo.x as f32 + 2.0,
                        115.0,
                        spo.z as f32 + 2.0,
                        0.0, /*BLOCKLIGHT */
                        14.0,
                        spo.x as f32 + 2.0,
                        spo.y as f32,
                        spo.z as f32 + 2.0,
                        lightf32, /*BLOCKLIGHT */
                        14.0,
                        spo.x as f32 - 1.0,
                        spo.y as f32,
                        spo.z as f32 + 2.0,
                        lightf32, /*BLOCKLIGHT */
                        14.0,
                        spo.x as f32 + 2.0,
                        spo.y as f32,
                        spo.z as f32,
                        lightf32, /*BLOCKLIGHT */
                        14.0,
                        spo.x as f32 + 2.0,
                        115.0,
                        spo.z as f32,
                        0.0, /*BLOCKLIGHT */
                        14.0,
                        spo.x as f32 + 2.0,
                        115.0,
                        spo.z as f32,
                        0.0, /*BLOCKLIGHT */
                        14.0,
                        spo.x as f32 - 1.0,
                        115.0,
                        spo.z as f32 + 2.0,
                        0.0, /*BLOCKLIGHT */
                        14.0,
                        spo.x as f32 - 1.0,
                        spo.y as f32,
                        spo.z as f32 + 2.0,
                        lightf32, /*BLOCKLIGHT */
                        14.0,
                        spo.x as f32 + 2.0,
                        spo.y as f32,
                        spo.z as f32,
                        lightf32, /*BLOCKLIGHT */
                        14.0,
                        spo.x as f32 - 1.0,
                        spo.y as f32,
                        spo.z as f32 + 2.0,
                        lightf32, /*BLOCKLIGHT */
                        14.0,
                        spo.x as f32 - 1.0,
                        115.0,
                        spo.z as f32 + 2.0,
                        0.0, /*BLOCKLIGHT */
                        14.0,
                        spo.x as f32 - 1.0,
                        115.0,
                        spo.z as f32 + 2.0,
                        0.0, /*BLOCKLIGHT */
                        14.0,
                        spo.x as f32 + 2.0,
                        115.0,
                        spo.z as f32,
                        0.0, /*BLOCKLIGHT */
                        14.0,
                        spo.x as f32 + 2.0,
                        spo.y as f32,
                        spo.z as f32,
                        lightf32, /*BLOCKLIGHT */
                        14.0,
                    ]);

                    let randyoffset = unsafe { RNG.gen_range(0.0..1.0) };

                    wuvdata.extend_from_slice(&[
                        face.blx,
                        face.bly + randyoffset,
                        0.0,
                        0.0,
                        face.brx + ONE_OVER_16 * 2.0,
                        face.bry + randyoffset,
                        0.0,
                        0.0,
                        face.brx + ONE_OVER_16 * 2.0,
                        face.bly - TEXTURE_WIDTH * (115.0 - spo.y as f32) + randyoffset,
                        0.0,
                        0.0,
                        face.brx + ONE_OVER_16 * 2.0,
                        face.bly - TEXTURE_WIDTH * (115.0 - spo.y as f32) + randyoffset,
                        0.0,
                        0.0,
                        face.blx,
                        face.bly - TEXTURE_WIDTH * (115.0 - spo.y as f32) + randyoffset,
                        0.0,
                        0.0,
                        face.blx,
                        face.bly + randyoffset,
                        0.0,
                        0.0,
                        face.blx,
                        face.bly + randyoffset,
                        0.0,
                        0.0,
                        face.brx + ONE_OVER_16 * 2.0,
                        face.bry + randyoffset,
                        0.0,
                        0.0,
                        face.brx + ONE_OVER_16 * 2.0,
                        face.bly - TEXTURE_WIDTH * (115.0 - spo.y as f32) + randyoffset,
                        0.0,
                        0.0,
                        face.brx + ONE_OVER_16 * 2.0,
                        face.bly - TEXTURE_WIDTH * (115.0 - spo.y as f32) + randyoffset,
                        0.0,
                        0.0,
                        face.blx,
                        face.bly - TEXTURE_WIDTH * (115.0 - spo.y as f32) + randyoffset,
                        0.0,
                        0.0,
                        face.blx,
                        face.bly + randyoffset,
                        0.0,
                        0.0,
                        face.blx,
                        face.bly + randyoffset,
                        0.0,
                        0.0,
                        face.brx + ONE_OVER_16 * 2.0,
                        face.bry + randyoffset,
                        0.0,
                        0.0,
                        face.brx + ONE_OVER_16 * 2.0,
                        face.bly - TEXTURE_WIDTH * (115.0 - spo.y as f32) + randyoffset,
                        0.0,
                        0.0,
                        face.brx + ONE_OVER_16 * 2.0,
                        face.bly - TEXTURE_WIDTH * (115.0 - spo.y as f32) + randyoffset,
                        0.0,
                        0.0,
                        face.blx,
                        face.bly - TEXTURE_WIDTH * (115.0 - spo.y as f32) + randyoffset,
                        0.0,
                        0.0,
                        face.blx,
                        face.bly + randyoffset,
                        0.0,
                        0.0,
                        face.blx,
                        face.bly + randyoffset,
                        0.0,
                        0.0,
                        face.brx + ONE_OVER_16 * 2.0,
                        face.bry + randyoffset,
                        0.0,
                        0.0,
                        face.brx + ONE_OVER_16 * 2.0,
                        face.bly - TEXTURE_WIDTH * (115.0 - spo.y as f32) + randyoffset,
                        0.0,
                        0.0,
                        face.brx + ONE_OVER_16 * 2.0,
                        face.bly - TEXTURE_WIDTH * (115.0 - spo.y as f32) + randyoffset,
                        0.0,
                        0.0,
                        face.blx,
                        face.bly - TEXTURE_WIDTH * (115.0 - spo.y as f32) + randyoffset,
                        0.0,
                        0.0,
                        face.blx,
                        face.bly + randyoffset,
                        0.0,
                        0.0,
                    ]);

                }
            }
        }

        let rm = ReadyMesh::new(
            index,
            &chunklock.pos,
            data32.len() as i32,
            tdata32.len() as i32,
            vdata.len() as i32,
            wvdata.len() as i32,
        );
        let ugqarc = self.finished_user_geo_queue.clone();
        let gqarc = self.finished_geo_queue.clone();

        match user_power {
            true => {
                ugqarc.push(rm);
            }
            false => {
                gqarc.push(rm);
            }
        }

        let tc = self.takencare.clone();

        if !tc.contains_key(&chunklock.pos) {
            tc.insert(chunklock.pos, chunklock);
        }
    }

    pub fn stamp_here(
        &self,
        spot: &IVec3,
        model: &JVoxModel,
        implicated: Option<&mut HashSet<IVec2>>,
    ) {
        let mut local_implicated_chunks; // Declare a mutable local HashSet for when None is provided
        let implicated_chunks; // This will be the reference used throughout the function
        let mut implicated_provided = false;

        // Determine which HashSet to use
        match implicated {
            Some(hs) => {
                implicated_chunks = hs;
                implicated_provided = true;
            }
            None => {
                local_implicated_chunks = HashSet::new(); // Create a new HashSet when None is provided
                implicated_chunks = &mut local_implicated_chunks; // Use the local HashSet
            }
        };

        for i in &model.model.models {
            let size = i.size;
            for v in &i.voxels {
                let rearr_point = IVec3::new(
                    v.point.x as i32 - (size.x / 2) as i32,
                    v.point.z as i32,
                    v.point.y as i32 - (size.y / 2) as i32,
                );

                let c_pos = ChunkSystem::spot_to_chunk_pos(&(*spot + rearr_point));
                implicated_chunks.insert(c_pos);
                self.set_block_no_sound(
                    IVec3::new(
                        spot.x + rearr_point.x,
                        spot.y + rearr_point.y,
                        spot.z + rearr_point.z,
                    ),
                    (v.color_index.0).clamp(0, Blocks::get_texs_length() as u8) as u32,
                    false,
                )
            }
        }

        if !implicated_provided {
            for c in implicated_chunks.iter() {
                match self.takencare.get(&c) {
                    Some(cf) => {
                        self.gen_rebuild_requests.push(cf.geo_index);
                    }
                    None => {}
                }
            }
        }
    }

    pub fn is_illuminite(spot: &IVec3) -> bool {
        let cpos = ChunkSystem::spot_to_chunk_pos(spot);
        let seed: [u8; 32] = [
            (cpos.x % 255) as u8,
            (cpos.y % 255) as u8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ]; // This needs to be a fixed-size array of bytes (u8).

        // Create a new RNG instance with the seed.
        let mut rng = StdRng::from_seed(seed);

        static perlin: Lazy<Perlin> = Lazy::new(|| Perlin::new(0) );
        let res = Self::_ore_noise(&perlin, *spot) > 1.0 && rng.gen_range(0..200) > 190;
        // if res {
        //     println!("Illuminite at {:?}", spot);
        // }
        return res;
    }

    pub fn generate_chunk(&self, cpos: &IVec2) {
        // Seed for the RNG.
        let seed: [u8; 32] = [
            (cpos.x % 255) as u8,
            (cpos.y % 255) as u8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ]; // This needs to be a fixed-size array of bytes (u8).

        // Create a new RNG instance with the seed.
        let mut rng = StdRng::from_seed(seed);

        // // Generate some random numbers
        // let rand_number1: u32 = rng.gen();
        // let rand_number2: u32 = rng.gen();
        let mut implicated: HashSet<IVec2> = HashSet::new();

        let should_break = false;
        

        //let dim_floors = Planets::get_floor_blocks(self.planet_type as u32);

        //let dim_range = Planets::get_voxel_model_index_range(self.planet_type as u32);



   

        //Two rng per chunk!
        //let spot: u32 = rng.gen_range(0..(CW as u32 * CW as u32)*(CH-40) as u32);
        //let item: u32 = rng.gen_range(dim_range.0 as u32..dim_range.1 as u32);

        //let mut index = 0;

        for x in 0..CH_W {
            for z in 0..CH_W {
                for y in (0..CH_H - 40).rev() {

                    


                    let coord = IVec3::new(cpos.x * CH_W + x, y, cpos.y * CH_W + z);

                    let climate = get_climate(self.temp_noise(coord) as f32, self.humidity_noise(coord) as f32);
                    let floorblock = climates::get_floor_block_based_on_climate(
                        &climate
                    );
                    //if index == spot {
                    if floorblock == self.natural_blockat(coord) && self.natural_blockat(coord + IVec3::new(0, 1, 0)) == 0 {
                        let featnoise = self.feature_noise(IVec2 {
                            x: coord.x * 20,
                            y: coord.z * 20,
                        }) * 20.0;
                        if featnoise > 0.0 {
                            let elev = match y {
                                0..=60 => Elevation::Low,
                                61..=100 => Elevation::Mid,
                                101..=255 => Elevation::High,
                                _ => Elevation::Low,
                            };
                            let treetypes = climates::get_tree_types(climate);

                            let item = rng.gen_range(0..treetypes.len());

                            let treetype = &treetypes[item];
                            let voxmodels = get_vox_mod_from_treetype(*treetype);

                            match voxmodels {
                                Some(vox) => {

                                    let decidedvox = &vox[rng.gen_range(0..vox.len())];

                                    let decidedjvox = &unsafe { &VOXEL_MODELS }[(*decidedvox) as usize];

                                    let item2: u32 = rng.gen_range(0..128);

                                    if item2 >= 127 as u32
                                    {
                                        self.stamp_here(
                                            &coord,
                                            decidedjvox,
                                            Some(&mut implicated),
                                        );
                                    }
                                   
                                }
                                None => {}
                            }
                            
                        }

                        // should_break = true;
                        // break;
                    }
                    //}

                    //index += 1;
                }
                if should_break {
                    break;
                }
            }
            if should_break {
                break;
            }
        }

        for c in implicated.iter() {
            match self.takencare.get(&c) {
                Some(cf) => {
                    self.gen_rebuild_requests.push(cf.geo_index);
                }
                None => {}
            }
        }
    }
    fn mix(a: f64, b: f64, t: f64) -> f64 {
        a * (1.0 - t) + b * t
    }

    pub fn biome_noise(&self, spot:IVec2) -> f64 {
        return Self::_biome_noise(&self.perlin.read(), spot);
    }

    pub fn _biome_noise(perlin: &Perlin, spot:IVec2) -> f64 {
        const XZDIVISOR1: f64 = 100.35 * 4.0;

        let y = 20;

        let noise1 = f64::max(
            0.0,
            perlin.get([
                spot.x as f64 / XZDIVISOR1,
                y as f64,
                spot.y as f64 / XZDIVISOR1,
            ]),
        );

        noise1 * 10.0
    }
    pub fn temp_noise(&self, spot:IVec3) -> f64 {
        return Self::_temp_noise(&self.perlin.read(), spot);
    }
    pub fn _temp_noise(perlin: &Perlin, spot:IVec3) -> f64 {
        const XZDIVISOR1: f64 = 105.35 * 4.0;


        let noise1 = 
        //make sure temp noise is not the fucking same as humid noise lol
            perlin.get([
                (spot.x as f64 + 1000.0) / XZDIVISOR1,
                (spot.y as f64 + 1000.0) / XZDIVISOR1,
                spot.z as f64 / XZDIVISOR1,
            ]) ;

        //println!("Temp noise: {}", noise1);
       // println!("Temp noise: {}", noise1 * 10.0);
       ( noise1 * 2.0)  - ((spot.y as f64 - 60.0) / 500.0)
    }
    pub fn humidity_noise(&self, spot:IVec3) -> f64 {
        return Self::_humidity_noise(&self.perlin.read(), spot);
    }
    pub fn _humidity_noise(perlin: &Perlin, spot:IVec3) -> f64 {
        const XZDIVISOR1: f64 = 55.35 * 4.0;



        let noise1 =     perlin.get([
                spot.x as f64 / XZDIVISOR1,
                spot.y as f64 / XZDIVISOR1,
                spot.z as f64 / XZDIVISOR1,
            ]);

        //println!("Humidity noise: {}", noise1*10.0);
        (noise1 * 2.0 ) - ((spot.y as f64 - 60.0) / 500.0)
    }
    pub fn ore_noise(&self, spot:IVec3) -> f64 {
        return Self::_ore_noise(&self.perlin.read(), spot);
    }

    pub fn _ore_noise(perlin: &Perlin, spot:IVec3) -> f64 {
        const XYZDIVISOR: f64 = 15.53;

        let noise1 = f64::max(
            0.0,
            perlin.get([
                spot.x as f64 / XYZDIVISOR,
                spot.y as f64 / XYZDIVISOR,
                spot.z as f64 / XYZDIVISOR,
            ]),
        );

        noise1 * ((60.0 - spot.y as f64).max(0.0) / 7.0)
    }
    pub fn feature_noise(&self, spot:IVec2) -> f64 {
        return Self::_feature_noise(&self.perlin.read(), spot);
    }

    pub fn _feature_noise(perlin: &Perlin, spot:IVec2) -> f64 {
        const XZDIVISOR1: f64 = 45.35 * 4.0;

        let y = 20;

        let noise1 = f64::max(
            0.0,
            perlin.get([
                (spot.x as f64 + 200.0) / XZDIVISOR1,
                y as f64,
                spot.y as f64 / XZDIVISOR1,
            ]),
        );

        noise1
    }

    pub fn cave_noise(&self, spot:IVec3) -> f64 {
        return Self::_cave_noise(&self.perlin.read(), spot);
    }

    pub fn _cave_noise(perlin: &Perlin, spot:IVec3) -> f64 {
        const XZDIVISOR1: f64 = 25.35;

        let noise1 = f64::max(
            0.0,
            perlin.get([
                (spot.x as f64) / XZDIVISOR1,
                (spot.y as f64) / XZDIVISOR1,
                spot.z as f64 / XZDIVISOR1,
            ]),
        );

        noise1
    }

    pub fn noise_func(&self, spot:IVec3) -> f64 {
        return Self::_noise_func(&self.perlin.read(), spot);
    }

    pub fn _noise_func(perlin: &Perlin, spot:IVec3) -> f64 {
        let per = perlin;

        let mut spot = spot;
        spot.y += 50;
        let spot = (Vec3::new(spot.x as f32, spot.y as f32, spot.z as f32) / 3.0)
            + Vec3::new(0.0, 10.0, 0.0);
        // let xzdivisor1 = 600.35 * 4.0;
        let xzdivisor2 = 1000.35 * 4.0;

        let mut y = spot.y - 20.0;

        let noise1 = f64::max(
            0.0,
            20.0 + per.get([
                spot.x as f64 / xzdivisor2,
                y as f64 / xzdivisor2,
                spot.z as f64 / xzdivisor2,
            ]) * 5.0
                - f64::max(
                    y as f64 / 1.7 + per.get([spot.x as f64 / 65.0, spot.z as f64 / 65.0]) * 10.0,
                    0.0,
                ),
        ) * 2.0;

        y += 100.0;

        let noise2 = f64::max(
            0.0,
            50.0 + per.get([
                spot.x as f64 / 100.35,
                y as f64 / 50.35,
                spot.z as f64 / 100.35,
            ]) * 10.0
                + per.get([
                    spot.x as f64 / 300.35,
                    y as f64 / 100.35,
                    spot.z as f64 / 300.35,
                ]) * 10.0
                - f64::max(y as f64 / 3.0, 0.0),
        );

        let mut p = per.get([spot.x as f64 / 500.0, spot.z as f64 / 500.0]) * 2.0;

        p = f64::max(p, 0.0);
        p = f64::min(p, 1.0);

        // Mixing noise1 and noise2 based on p, assuming `mix` is a function that blends the two values
        // Rust doesn't have a direct `mix` function, but you can create one or use a linear interpolation
        let noisemix = ChunkSystem::mix(noise1, noise2, p);

        let texture = per.get([
            spot.x as f64 / 12.35,
            y as f64 / 12.35,
            spot.z as f64 / 12.35,
        ]) * 1.0;

        let noise3 = f64::max(
            0.0,
            50.0 + per.get([
                spot.x as f64 / 25.35,
                y as f64 / 25.35,
                spot.z as f64 / 25.35,
            ]) * 10.0
                + per.get([
                    spot.x as f64 / 60.35,
                    y as f64 / 50.35,
                    spot.z as f64 / 60.35,
                ]) * 10.0
                - f64::max(y as f64 / 3.0, 0.0),
        );

        let mut p2 = 0.5
            + per.get([
                (spot.x as f64 + 4500.0) / 150.0,
                (spot.y as f64 + 5000.0) / 150.0,
                (spot.z as f64 - 5000.0) / 150.0,
            ]) * 1.0;

        let p3 = (per.get([
            (spot.x as f64 - 1500.0) / 3500.0,
            (spot.z as f64 + 1000.0) / 3500.0,
        ]) * 10.0)
            .min(9.0);

        p2 = f64::max(p2, 0.0);
        p2 = f64::min(p2, 1.0);

        ChunkSystem::mix(noisemix + texture, noise3, p2.clamp(0.0, 1.0)).min(20.0) + p3
    }

    pub fn noise_func2(&self, spot:IVec3) -> f64 {
        let p2 = self.perlin.read();
        let mut y = spot.y - 20;

        let noise1 = f64::max(
            0.0,
            20.0 + p2.get([
                spot.x as f64 / 25.35,
                y as f64 / 20.35,
                spot.z as f64 / 25.35,
            ]) * 5.0
                - f64::max(
                    y as f64 / 2.0 + p2.get([spot.x as f64 / 65.0, spot.z as f64 / 65.0]) * 10.0,
                    0.0,
                ),
        );

        y += 60;

        let noise2 = f64::max(
            0.0,
            50.0 + p2.get([
                spot.x as f64 / 55.35,
                y as f64 / 25.35,
                spot.z as f64 / 55.35,
            ]) * 10.0
                + p2.get([
                    spot.x as f64 / 25.35,
                    y as f64 / 65.35,
                    spot.z as f64 / 25.35,
                ]) * 20.0
                - f64::max(y as f64 * 3.0, 0.0),
        );

        let mut p = p2.get([spot.x as f64 / 500.0, spot.z as f64 / 500.0]) * 5.0;

        p = f64::max(p, 0.0);
        p = f64::min(p, 1.0);

        // Mixing noise1 and noise2 based on p, assuming `mix` is a function that blends the two values
        // Rust doesn't have a direct `mix` function, but you can create one or use a linear interpolation
        ChunkSystem::mix(noise1, noise2, p * 0.5)
    }

    pub fn blockatmemo(&self, spot:IVec3, memo: &mut HashMap<IVec3, u32>) -> u32 {
        // return memo.get(&spot).map_or_else(|| {
        //     let b = self.blockat(spot);
        //     memo.insert(spot, b);
        //     b
        // }, |b| *b);

        return match memo.get(&spot) {
            Some(b) => *b,
            None => {
                let b = self.blockat(spot);
                memo.insert(spot, b);
                b
            }
        };

        // if memo.contains_key(&spot) {
        //     return *memo.get(&spot).unwrap();
        // } else {
        //     let b = self.blockat(spot);
        //     memo.insert(spot, b);
        //     return b;
        // }
    }
    pub fn blockat(&self, spot:IVec3) -> u32 {
        let nudm = unsafe {NONUSERDATAMAP.as_ref().unwrap()};
        Self::_blockat(
            &nudm.clone(),
            unsafe { &USERDATAMAPANDMISCMAP.as_ref().unwrap().clone() },
            &self.perlin.read(),
            spot,
        )
    }
    pub fn _blockat(
        nonuserdatamap: &Arc<DashMap<IVec3, u32>>,
        userdatamap: &UserDataMapAndMiscMap,
        perlin: &Perlin,
        spot:IVec3,
    ) -> u32 {

        match userdatamap.get(&spot) {
            Some(id) => {
                return id;
            }
            None => {}
        }

        match nonuserdatamap.get(&spot) {
            Some(id) => {
                return *id;
            }
            None => return Self::_natural_blockat(perlin, spot),
        }
    }

    pub fn natural_blockat(&self, spot:IVec3) -> u32 {
        return Self::_natural_blockat(&self.perlin.read(), spot);
    }

    pub fn _natural_blockat(perlin: &Perlin, spot:IVec3) -> u32 {
        let per = perlin;
        if spot.y == 0 {
            return 15;
        }

        let floorblock = get_floor_block_based_on_climate(climates::get_climate(
            Self::_temp_noise(per, spot) as f32,
            Self::_humidity_noise(per, spot) as f32,
        ));

        let ret = match 0 {
            // 1 => {
            //     if self.noise_func2(spot) > 10.0 {
            //         if self.noise_func2(spot +IVec3 { x: 0, y: 1, z: 0 }) < 10.0 {
            //             14
            //         } else {
            //             1
            //         }

            //     } else {
            //         0
            //     }
            // }
            _ => {
                static WL: f32 = 60.0;

                let biomenum = Self::_biome_noise(
                    per,
                    IVec2 {
                        x: spot.x,
                        y: spot.z,
                    },
                );
                let biomenum2 = Self::_biome_noise(
                    per,
                    IVec2 {
                        x: spot.x * 20 + 5000,
                        y: spot.z * 20 + 5000,
                    },
                );

                let underdirt = 5;
                let surface = floorblock;
                let undersurface = if floorblock == 1 { 1 } else { 4 };
                let liquid = if floorblock == 62 { 51 } else { 2 };
                let beach = if floorblock == 62 { 62 } else { 1 };

                

                if Self::_noise_func(per, spot) > 10.0 {
                    if Self::_noise_func(per, spot +IVec3 { x: 0, y: 10, z: 0 }) > 10.0 {
                        if Self::_ore_noise(per, spot) > 1.0 {
                            35
                        } else {
                            underdirt
                        }
                    } else {
                        let beachnoise = per.get([
                            spot.y as f64 / 7.5,
                            spot.z as f64 / 7.5,
                            spot.x as f64 / 7.5,
                        ]);
                        if spot.y > (WL + beachnoise as f32) as i32
                            || Self::_noise_func(per, spot +IVec3 { x: 0, y: 5, z: 0 }) > 10.0
                        {
                            if Self::_noise_func(per, spot +IVec3 { x: 0, y: 1, z: 0 }) < 10.0
                            {
                                surface
                            } else {
                                undersurface
                            }
                        } else {
                            beach
                        }
                    }
                } else {
                    if spot.y < WL as i32 {
                        liquid
                    } else {
                        0
                    }
                }
            }
        };
        // if ret == 5 {
        //     if Self::is_illuminite(&spot) {
        //         return 90;
        //     }
        // }
        if ret != 2 && ret != 51 {
            if Self::_cave_noise(per, spot) > 0.5 {
                return 0;
            }
        }
        ret
    }
}
