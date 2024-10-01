use arrayvec::ArrayVec;
use num_enum::FromPrimitive;




pub const VOX_MODEL_PATHS: [&'static str; 29] = [
    "assets/voxelmodels/bush.vox",
    "assets/voxelmodels/tree1.vox",
    "assets/voxelmodels/tree2.vox",
    "assets/voxelmodels/rock1.vox",
    "assets/voxelmodels/rock2.vox",
    "assets/voxelmodels/tree3.vox",
    "assets/voxelmodels/tree4.vox",
    "assets/voxelmodels/tree5.vox",
    "assets/voxelmodels/bamboo1.vox",
    "assets/voxelmodels/bamboo2.vox",
    "assets/voxelmodels/tallgrass1.vox",
    "assets/voxelmodels/tallgrass2.vox",
    "assets/voxelmodels/tallgrass3.vox",
    "assets/voxelmodels/rubbertree.vox",
    "assets/voxelmodels/ptree.vox",
    "assets/voxelmodels/redrock.vox",
    "assets/voxelmodels/crystal1.vox",
    "assets/voxelmodels/awds.vox",
    "assets/voxelmodels/pinetree1.vox",
    "assets/voxelmodels/pinetree2.vox",
    "assets/voxelmodels/articwillow.vox",
    "assets/voxelmodels/cedartree1.vox",
    "assets/voxelmodels/cedartree2.vox",
    "assets/voxelmodels/palmtree1.vox",
    "assets/voxelmodels/palmtree2.vox",
    "assets/voxelmodels/palmtree3.vox",
    "assets/voxelmodels/joshuatree1.vox",
    "assets/voxelmodels/joshuatree2.vox",
    "assets/voxelmodels/joshuatree3.vox",
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
    
}




pub enum Climate {
    PolarDesert,        BorealForest,       WetTundra,
    TemperateGrassland, DeciduousForest,  TemperateRainforest,
    HotDesert,          Savannah,          TropicalRainforest,
}


const CLIMATE_GRID: [Climate; 9] = [
    Climate::PolarDesert,        Climate::BorealForest,     Climate::WetTundra,
    Climate::TemperateGrassland, Climate::DeciduousForest,  Climate::TemperateRainforest,
    Climate::HotDesert,          Climate::Savannah,         Climate::TropicalRainforest,
];


#[derive(Clone, Copy)]
pub enum TreeType {
    ArticWillowDwarfShrub,
    Pine,
    ArticWillow,
    Oak,
    Maple,
    Cedar,
    Palm,
    Joshua,
    Rubber
}

pub const MAX_TREES_PER_CLIMATE: usize = 4;

pub fn get_vox_mod_from_treetype(treetype: TreeType) -> Option<ArrayVec<VoxelModel, MAX_TREES_PER_CLIMATE>> {
    match treetype {
        TreeType::ArticWillowDwarfShrub => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::Awds);
            Some(v)
        },
        TreeType::Pine => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::PineTree1);
            v.push(VoxelModel::PineTree2);
            Some(v)
        },
        TreeType::ArticWillow => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::ArticWillow);
            Some(v)
        },
        TreeType::Oak => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::Tree1);
            v.push(VoxelModel::Tree2);
            Some(v)
        },
        TreeType::Maple => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::Tree3);
            v.push(VoxelModel::Tree4);
            Some(v)
        },
        TreeType::Cedar => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::CedarTree1);
            v.push(VoxelModel::CedarTree2);
            Some(v)
        },
        TreeType::Palm => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::PalmTree1);
            v.push(VoxelModel::PalmTree2);
            v.push(VoxelModel::PalmTree3);
            Some(v)
        },
        TreeType::Joshua => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::JoshuaTree1);
            v.push(VoxelModel::JoshuaTree2);
            v.push(VoxelModel::JoshuaTree3);
            Some(v)
        },
        TreeType::Rubber => {
            let mut v = ArrayVec::new();
            v.push(VoxelModel::RubberTree);
            Some(v)
        },
    }
}



//get climate based on temperature and humidity 0.0-1.0
pub fn get_climate(temp: f32, hum: f32) -> &'static Climate {
    let temp = temp.clamp(0.0, 1.0);
    let hum = hum.clamp(0.0, 1.0);
    let temp = (temp * 3.0) as usize;
    let hum = (hum * 3.0) as usize;
    &CLIMATE_GRID[(temp * 3 + hum).clamp(0, 8)]
}

//get tree types based on climate
pub fn get_tree_types(climate: &Climate) -> ArrayVec<TreeType, MAX_TREES_PER_CLIMATE> {
    match climate {
        Climate::PolarDesert => {
            let mut v = ArrayVec::new();
            v.push(TreeType::ArticWillowDwarfShrub);
            v
        },
        Climate::BorealForest => {
            let mut v = ArrayVec::new();
            v.push(TreeType::Pine);
            v.push(TreeType::ArticWillow);
            v
        },
        Climate::WetTundra => {
            let mut v = ArrayVec::new();
            v.push(TreeType::ArticWillowDwarfShrub);
            v
        },
        Climate::TemperateGrassland => {
            let mut v = ArrayVec::new();
            v.push(TreeType::Oak);
            v
        },
        Climate::DeciduousForest => {
            let mut v = ArrayVec::new();
            v.push(TreeType::Oak);
            v.push(TreeType::Maple);
            v
        },
        Climate::TemperateRainforest => {
            let mut v = ArrayVec::new();
            v.push(TreeType::Maple);
            v
        },
        Climate::HotDesert => {
            let mut v = ArrayVec::new();
            v.push(TreeType::Cedar);
            v
        },
        Climate::Savannah => {
            let mut v = ArrayVec::new();
            v.push(TreeType::Palm);
            v
        },
        Climate::TropicalRainforest => {
            let mut v = ArrayVec::new();
            v.push(TreeType::Joshua);
            v
        },
    }
}


pub fn get_floor_block_based_on_climate(climate: &Climate) -> u32 {
    match climate {
        Climate::HotDesert => 1,
        Climate::PolarDesert => 62,

        _ => 3
    }
}
