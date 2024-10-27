use std::iter::Map;

use arrayvec::ArrayVec;
use bevy::{a11y::accesskit::Tree, utils::HashMap};
use num_enum::FromPrimitive;
use once_cell::sync::Lazy;


pub const VOX_MODEL_PATHS: [&'static str; 52] = [
    path!("assets/voxelmodels/bush.vox"),
    path!("assets/voxelmodels/tree1.vox"),
    path!("assets/voxelmodels/tree2.vox"),
    path!("assets/voxelmodels/rock1.vox"),
    path!("assets/voxelmodels/rock2.vox"),
    path!("assets/voxelmodels/tree3.vox"),
    path!("assets/voxelmodels/tree4.vox"),
    path!("assets/voxelmodels/tree5.vox"),
    path!("assets/voxelmodels/bamboo1.vox"),
    path!("assets/voxelmodels/bamboo2.vox"),
    path!("assets/voxelmodels/tallgrass1.vox"),
    path!("assets/voxelmodels/tallgrass2.vox"),
    path!("assets/voxelmodels/tallgrass3.vox"),
    path!("assets/voxelmodels/rubbertree.vox"),
    path!("assets/voxelmodels/ptree.vox"),
    path!("assets/voxelmodels/redrock.vox"),
    path!("assets/voxelmodels/crystal1.vox"),
    path!("assets/voxelmodels/awds.vox"),
    path!("assets/voxelmodels/pinetree1.vox"),
    path!("assets/voxelmodels/pinetree2.vox"),
    path!("assets/voxelmodels/articwillow.vox"),
    path!("assets/voxelmodels/cedartree1.vox"),
    path!("assets/voxelmodels/cedartree2.vox"),
    path!("assets/voxelmodels/palmtree1.vox"),
    path!("assets/voxelmodels/palmtree2.vox"),
    path!("assets/voxelmodels/palmtree3.vox"),
    path!("assets/voxelmodels/joshuatree1.vox"),
    path!("assets/voxelmodels/joshuatree2.vox"),
    path!("assets/voxelmodels/joshuatree3.vox"),

    path!("assets/voxelmodels/paperbirch1.vox"),
    path!("assets/voxelmodels/paperbirch2.vox"),
    path!("assets/voxelmodels/paperbirch3.vox"),

    path!("assets/voxelmodels/greenalder1.vox"),
    path!("assets/voxelmodels/greenalder2.vox"),

    path!("assets/voxelmodels/willow1.vox"),
    path!("assets/voxelmodels/willow2.vox"),
    path!("assets/voxelmodels/willow3.vox"),

    path!("assets/voxelmodels/beech1.vox"),
    path!("assets/voxelmodels/beech2.vox"),
    path!("assets/voxelmodels/beech3.vox"),

    path!("assets/voxelmodels/westernhemlock1.vox"),
    path!("assets/voxelmodels/westernhemlock2.vox"),
    path!("assets/voxelmodels/westernhemlock3.vox"),

    path!("assets/voxelmodels/eucalyptus1.vox"),
    path!("assets/voxelmodels/eucalyptus2.vox"),
    path!("assets/voxelmodels/eucalyptus3.vox"),

    path!("assets/voxelmodels/saguaro1.vox"),
    path!("assets/voxelmodels/saguaro2.vox"),
    path!("assets/voxelmodels/saguaro3.vox"),

    path!("assets/voxelmodels/figtree1.vox"),
    path!("assets/voxelmodels/figtree2.vox"),
    path!("assets/voxelmodels/pumpkin.vox")
];

#[derive(PartialEq, FromPrimitive, Clone, Copy)]
#[repr(usize)]
pub enum VoxelModel {
    #[num_enum(default)]
    Bush = 0,
    Tree1 = 1,
    Tree2 = 2,
    Rock1 = 3,
    Rock2 = 4,
    Tree3 = 5,
    Tree4 = 6,
    Tree5 = 7,
    Bamboo1 = 8,
    Bamboo2 = 9,
    TallGrass1 = 10,
    TallGrass2 = 11,
    TallGrass3 = 12,
    RubberTree = 13,
    PTree = 14,
    RedRock = 15,
    Crystal1 = 16,
    Awds = 17,
    PineTree1 = 18,
    PineTree2 = 19,
    ArticWillow = 20,
    CedarTree1 = 21,
    CedarTree2 = 22,
    PalmTree1 = 23,
    PalmTree2 = 24,
    PalmTree3 = 25,
    JoshuaTree1 = 26,
    JoshuaTree2 = 27,
    JoshuaTree3 = 28,

    PaperBirch1 = 29,
    PaperBirch2 = 30,
    PaperBirch3 = 31,

    GreenAlder1 = 32,
    GreenAlder2 = 33,

    Willow1 = 34,
    Willow2 = 35,
    Willow3 = 36,

    Beech1 = 37,
    Beech2 = 38,
    Beech3 = 39,

    WesternHemlock1 = 40,
    WesternHemlock2 = 41,
    WesternHemlock3 = 42,

    Eucalyptus1 = 43,
    Eucalyptus2 = 44,
    Eucalyptus3 = 45,

    Saguaro1 = 46,
    Saguaro2 = 47,
    Saguaro3 = 48,

    FigTree1 = 49,
    FigTree2 = 50,

    Pumpkin = 51,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Climate {
    PolarDesert,
    BorealForest,
    WetTundra,
    TemperateGrassland,
    DeciduousForest,
    TemperateRainforest,
    HotDesert,
    Savannah,
    TropicalRainforest,
}

const CLIMATE_GRID: [Climate; 9] = [
    Climate::PolarDesert,
    Climate::BorealForest,
    Climate::WetTundra,
    Climate::TemperateGrassland,
    Climate::DeciduousForest,
    Climate::TemperateRainforest,
    Climate::HotDesert,
    Climate::Savannah,
    Climate::TropicalRainforest,
];

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Elevation {
    Low,
    Mid,
    High,
}

#[derive(Clone, Copy)]
pub enum TerrainFeatureType {
    ArticWillowDwarfShrub,
    Pine,
    ArticWillow,
    Oak,
    Maple,
    Cedar,
    Palm,
    Joshua,
    Rubber,
    PaperBirch,
    GreenAlder,
    Willow,
    Beech,
    WesternHemlock,
    Eucalyptus,
    Saguaro,
    FigTree,
    Pumpkin,
    Boulder,
    Bamboo,
    LightberryBush,

}

pub const MAX_TREES_PER_CLIMATE: usize = 4;

pub fn get_vox_mod_from_treetype(
    treetype: TerrainFeatureType,
) -> Option<ArrayVec<VoxelModel, MAX_TREES_PER_CLIMATE>> {
    match treetype {
        TerrainFeatureType::ArticWillowDwarfShrub => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::Awds);
            Some(v)
        }
        TerrainFeatureType::Pine => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::PineTree1);
            v.push(VoxelModel::PineTree2);
            Some(v)
        }
        TerrainFeatureType::ArticWillow => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::ArticWillow);
            Some(v)
        }
        TerrainFeatureType::Oak => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::Tree1);
            v.push(VoxelModel::Tree2);
            Some(v)
        }
        TerrainFeatureType::Maple => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::Tree3);
            v.push(VoxelModel::Tree4);
            Some(v)
        }
        TerrainFeatureType::Cedar => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::CedarTree1);
            v.push(VoxelModel::CedarTree2);
            Some(v)
        }
        TerrainFeatureType::Palm => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::PalmTree1);
            v.push(VoxelModel::PalmTree2);
            v.push(VoxelModel::PalmTree3);
            Some(v)
        }
        TerrainFeatureType::Joshua => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::JoshuaTree1);
            v.push(VoxelModel::JoshuaTree2);
            v.push(VoxelModel::JoshuaTree3);
            Some(v)
        }
        TerrainFeatureType::Rubber => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::RubberTree);
            Some(v)
        }
        TerrainFeatureType::PaperBirch => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::PaperBirch1);
            v.push(VoxelModel::PaperBirch2);
            v.push(VoxelModel::PaperBirch3);
            Some(v)
        }
        TerrainFeatureType::GreenAlder => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::GreenAlder1);
            v.push(VoxelModel::GreenAlder2);
            Some(v)
        }
        TerrainFeatureType::Willow => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::Willow1);
            v.push(VoxelModel::Willow2);
            v.push(VoxelModel::Willow3);
            Some(v)
        }
        TerrainFeatureType::Beech => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::Beech1);
            v.push(VoxelModel::Beech2);
            v.push(VoxelModel::Beech3);
            Some(v)
        }
        TerrainFeatureType::WesternHemlock => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::WesternHemlock1);
            v.push(VoxelModel::WesternHemlock2);
            v.push(VoxelModel::WesternHemlock3);
            Some(v)
        }
        TerrainFeatureType::Eucalyptus => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::Eucalyptus1);
            v.push(VoxelModel::Eucalyptus2);
            v.push(VoxelModel::Eucalyptus3);
            Some(v)
        }
        TerrainFeatureType::Saguaro => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::Saguaro1);
            v.push(VoxelModel::Saguaro2);
            v.push(VoxelModel::Saguaro3);
            Some(v)
        }
        TerrainFeatureType::FigTree => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::FigTree1);
            v.push(VoxelModel::FigTree2);
            Some(v)
        }
        TerrainFeatureType::Pumpkin => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::Pumpkin);
            Some(v)
        }
        TerrainFeatureType::Boulder => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::Rock1);
            v.push(VoxelModel::Rock2);
            Some(v)
        }
        TerrainFeatureType::Bamboo => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::Bamboo1);
            v.push(VoxelModel::Bamboo2);
            Some(v)
        }
        TerrainFeatureType::LightberryBush => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::Bush);
            Some(v)
        }
    }
}

//get climate based on temperature and humidity -1.0 thru 1.0
pub fn get_climate(temp: f32, hum: f32) -> &'static Climate {
    // Clamp temp and humidity to the range [-1.0, 1.0]
    let temp_clamped = temp.clamp(-1.0, 1.0);
    let hum_clamped = hum.clamp(-1.0, 1.0);

    // Map the clamped values to indices
    let temp_idx = ((temp_clamped + 1.0) * 1.5).floor() as usize; // Maps to [0, 2]
    let hum_idx = ((hum_clamped + 1.0) * 1.5).floor() as usize; // Maps to [0, 2]

    // Ensure indices are within the bounds
    let temp_idx = temp_idx.min(2); // Ensure it's at most 2
    let hum_idx = hum_idx.min(2); // Ensure it's at most 2

    // Calculate the flat index
    let index = temp_idx * 3 + hum_idx; // GRID_SIZE is 3

    // Return the climate from the flat grid
    &CLIMATE_GRID[index]
}

//get tree types based on climate
pub fn get_tree_types(climate: &Climate) -> &'static [TerrainFeatureType] {
    static CLIMATE_CATEGORIES: Lazy<HashMap<Climate, &'static [TerrainFeatureType]>> =
        Lazy::new(|| {
            let mut map: hashbrown::HashMap<Climate, &'static [TerrainFeatureType]> = HashMap::default();
            map.insert(
                Climate::PolarDesert,
                &[
                    TerrainFeatureType::Boulder,
                    TerrainFeatureType::ArticWillowDwarfShrub,
                ],
            );
            map.insert(
                Climate::BorealForest,
                &[
                    TerrainFeatureType::LightberryBush,
                    TerrainFeatureType::Boulder,
                    TerrainFeatureType::Pine,
                    TerrainFeatureType::PaperBirch,
                ],
            );
            map.insert(
                Climate::WetTundra,
                &[
                    TerrainFeatureType::Boulder,
                    TerrainFeatureType::ArticWillow,
                    TerrainFeatureType::GreenAlder,
                ],
            );
            map.insert(
                Climate::TemperateGrassland,
                &[
                    TerrainFeatureType::LightberryBush,
                    TerrainFeatureType::Oak,
                    TerrainFeatureType::Willow,
                    TerrainFeatureType::Pumpkin
                ],
            );
            map.insert(
                Climate::DeciduousForest,
                &[
                    TerrainFeatureType::LightberryBush,
                    TerrainFeatureType::Oak,
                    TerrainFeatureType::Maple,
                    TerrainFeatureType::Beech,
                ],
            );
            map.insert(
                Climate::TemperateRainforest,
                &[
                    TerrainFeatureType::LightberryBush,
                    TerrainFeatureType::Bamboo,
                    TerrainFeatureType::Boulder,
                    TerrainFeatureType::Maple,
                    TerrainFeatureType::Cedar,
                    TerrainFeatureType::WesternHemlock,
                ],
            );
            map.insert(
                Climate::HotDesert,
                &[
                    TerrainFeatureType::Joshua,
                    TerrainFeatureType::Saguaro,
                    TerrainFeatureType::Boulder,
                ],
            );
            map.insert(
                Climate::Savannah,
                &[
                    TerrainFeatureType::Palm,
                    TerrainFeatureType::Eucalyptus,

                ],
            );
            map.insert(
                Climate::TropicalRainforest,
                &[
                    TerrainFeatureType::Bamboo,
                    TerrainFeatureType::Rubber,
                    TerrainFeatureType::FigTree,
                ],
            );
            map
        });
       
    match CLIMATE_CATEGORIES.get(climate) {
        Some(tree_types) => *tree_types,
        None => &[],
    }
}

pub fn get_floor_block_based_on_climate(climate: &Climate) -> u32 {
    match climate {
        Climate::HotDesert => 1,
        Climate::PolarDesert => 62,

        _ => 3,
    }
}
