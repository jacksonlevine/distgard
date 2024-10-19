use bevy::math::Vec3;

use crate::{chunk::LightColor, cube::CubeSide};

pub const BLOCK_DIRECTION_BITS: u32 =       0b0000_0000_0000_0011_0000_0000_0000_0000;
pub const BLOCK_MARKED_FOR_DELETION: u32 = 0b0000_0000_0000_0100_0000_0000_0000_0000;
pub struct Blocks {}

pub const BLOCK_COUNT: u32 = 91;

static BREAKTIMES: [f32; BLOCK_COUNT as usize] = [
    0.1,
    0.5,
    0.7,
    0.7,
    0.5,
    1.0,
    0.7,
    0.2,
    0.7,
    1.5,
    0.7,
    0.8,
    1.1,
    1.5,
    0.7,
    9999999.0,
    1.2,
    0.5,
    1.0,
    1.0,
    0.6,
    1.5,
    1.0,
    0.2,

    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,

    1.0, 
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,

    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    0.5,
    0.5,
    1.0,
    1.0,
    0.3,
    0.7,
    1.0,
    0.7,
    1.0,
    1.0,
    0.5,
    1.5,
    0.7,
    1.0,
    0.7,
    1.5,
    0.9,
    0.7,
    1.5,


    1.0, 1.0, 
    1.0, 1.0, 
    1.0, 1.0,

    1.0, 1.0, 
    1.0, 1.0,
    1.0, 1.0, 

    1.0, 1.0, 
    1.0, 

    1.0,
    1.0,


    1.0, 1.0, 1.0,
    1.0, 1.0, 1.0,
    1.0, 1.0, 1.0,

    2.0
];

static TEXS: [[(u8, u8); 3]; BLOCK_COUNT as usize] = [
            //sides   //bot   //top
            [(0, 0), (0, 0), (0, 0)],  // 0
            [(1, 0), (1, 0), (1, 0)],  // 1 sand
            [(2, 0), (2, 0), (2, 0)],  // 2 water
            [(3, 0), (4, 0), (3, 1)],  // 3 grass
            [(4, 0), (4, 0), (4, 0)],  // 4 dirt
            [(5, 0), (5, 0), (5, 0)],  // 5 cobble
            [(6, 0), (6, 1), (6, 1)],  // 6 log
            [(7, 0), (7, 0), (7, 0)],  // 7 leaves                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                
            [(8, 0), (8, 0), (8, 0)],    // 08 glass
            [(9, 0), (9, 0), (9, 0)],    // 09 smooth stone
            [(10, 0), (10, 0), (10, 0)], // 10 planks wood
            [(7, 1), (7, 1), (7, 1)], // 11 bush leaves
            [(4, 2), (4, 2), (4, 2)], // 12 petrified wood
            [(6, 2), (6, 2), (6, 2)], // 13 red stone
            [(7, 2), (7, 2), (7, 2)], // 14 salted earth
            [(8, 2), (8, 2), (8, 2)], // 15 bedrock
            [(0, 3), (0, 3), (0, 3)], // 16 red crystal unattainable
            [(0, 4), (0, 4), (0, 4)], // 17 red crystal

            [(12, 1), (12, 1), (12, 1)], // 18 light

            [(12, 0), (12, 0), (12, 0)], // 19 door
            [(0, 1), (0, 1), (0, 1)], // 20 ladder
            [(14, 2), (14, 2), (14, 2)], // 21 wooden trunk
            [(13, 1), (14, 1), (14, 1)], // 22 bamboo
            [(1, 3), (1, 3), (1, 3)], // 23 tallgrass

            [(10, 2), (10, 2), (10, 2)], // 24 blue light
            [(11, 2), (11,2), (11, 2)], // 25 purple light
            [(12, 2), (12, 2), (12, 2)], // 26 yellow light

            [(13, 2), (13, 2), (13, 2)], // 27 red light
            [(10, 3), (10, 3), (10, 3)], // 28 green light
            [(11, 3), (11, 3), (11, 3)], // 29 orange light
            [(12, 3), (12, 3), (12, 3)], // 30 teal light
            [(1,5), (1,5), (1,5)], // 31 crafttable

            [(3, 3), (3, 3), (3, 3)], // 32 apple
            [(2, 3),(2, 3),(2, 3)], // 33 bamboo chute
            [(7,4),(7,4),(7,4)], // 34 dead leaves


            [(2,4),(2,4),(2,4)], // 35 metal rock
            [(2,5),(2,5),(2,5)], // 36 crude blade

            [(3,5),(3,5),(3,5)], // 37 crude pick
            [(4,5),(4,5),(4,5)], // 38 crude mattock
            [(5,5),(5,5),(5,5)], // 39 crude axe


            [(10,4),(10,4),(10,4)], // 40 jumper blue
            [(11,4),(11,4),(11,4)], // 41 jumper yellow
            [(10,5),(10,5),(10,5)], // 42 trampoline block

            [(0,8),(2,8),(2,8)], // 43 rubber tree wood
            [(1,8),(1,8),(1,8)], // 44 rubber tree leaves
            [(10,6),(10,6),(10,6)], // 45 conveyor/highway
            [(11,5),(11,5),(11,5)], // 46 auto trampoline block
            [(1,6),(1,6),(1,6)], // 47  metal plate block

            [(8,4),(4,0),(8,5)], // 48, snowy grass
            [(9,4),(9,4),(9,4)], // 49, torch
            [(7, 5), (7, 0), (8, 5)],  // 50 snow leaves
            [(1, 7), (1, 7), (1, 7)],  // 51 ice
            [(1, 12), (1, 12), (1, 12)],  // 52 Artic Willow Dwarf Shrub
            [(3, 6), (3, 7), (3, 7)],  // 53 Pine Wood
            [(3, 8), (3, 8), (3, 8)],  // 54 Pine Leaves
            [(2, 12), (2, 12), (2, 12)],  // 55 Artic Willow Leaves
            [(4, 6), (4, 7), (4, 7)],  // 56 Cedar Wood
            [(4, 8), (4, 8), (4, 8)],  // 57 Cedar Leaves
            [(5, 6), (5, 7), (5, 7)],  // 58 Palm Wood
            [(5, 8), (5, 8), (5, 8)],  // 59 Palm Leaves
            [(6, 6), (6, 7), (6, 7)],  // 60 Joshua Wood
            [(6, 8), (6, 8), (6, 8)],  // 61 Joshua Leaves
            [(0, 9),(1,0),(8,5)], // 62, snowy sand
            [(13, 3),(13, 3),(13, 3)], // 63, fence



            [(7, 6), (7, 7), (7, 7)],  // 64 Paper Birch Wood
            [(7, 8), (7, 8), (7, 8)],  // 65 Paper Birch Leaves

            [(8, 6), (8, 7), (8, 7)],  // 66 Green Alder Wood
            [(8, 8), (8, 8), (8, 8)],  // 67 Green Alder Leaves

            [(9, 6), (9, 7), (9, 7)],  // 68 Willow Wood
            [(9, 8), (9, 8), (9, 8)],  // 69 willow Leaves


            [(3, 9), (3, 10), (3, 10)],  // 70 Beech Wood
            [(3, 11), (3, 11), (3, 11)],  // 71 Beech Leaves

            [(4, 9), (4, 10), (4, 10)],  // 72 Western Hemlock Wood
            [(4, 11), (4, 11), (4, 11)],  // 73 Western Hemlock Leaves


            [(5, 9), (5, 10), (5, 10)],  // 74 Eucalyptus Wood
            [(5, 11), (5, 11), (5, 11)],  // 75 Eucalyptus Leaves

            [(6, 9), (6, 10), (6, 10)],  // 76 Fig Wood
            [(6, 11), (6, 11), (6, 11)],  // 77 Fig Leaves
            
            [(7, 9), (7, 10), (7, 10)],  // 78 Saguaro Block

            [(3, 12), (4, 13), (3, 13)],  // 79 Pumpkin
            [(4, 12), (4, 13), (3, 13)],  // 80 Jack o lantern

            [(5, 12), (5, 12), (5, 12)],  // 81 Pine Planks
            [(6, 12), (6, 12), (6, 12)],  // 82 Cedar Planks
            [(7, 12), (7, 12), (7, 12)], // 83 Palm Planks
            [(8, 12), (8, 12), (8, 12)], // 84 Joshua Planks
            [(9, 12), (9, 12), (9, 12)], // 85 Birch Planks
            [(10, 12), (10, 12), (10, 12)], // 86 Willow Planks
            [(11, 12), (11, 12), (11, 12)], // 87 Beech Planks
            [(12, 12), (12, 12), (12, 12)], // 88 Western Hemlock Planks
            [(13, 12), (13, 12), (13, 12)], // 89 Eucalyptus Planks
            [(12, 4), (12, 4), (12, 4)], // 90 Illuminite Ore
        ];


use std::collections::HashMap;

pub static BLOCK_NAME_TO_ID: std::sync::LazyLock::<HashMap<String, u32>> = std::sync::LazyLock::new(|| { 
    let mut map = HashMap::new();
    for i in 0..BLOCK_COUNT {
        map.insert(Blocks::get_name(i).to_string(), i);
        map.insert(Blocks::get_name(i).to_ascii_lowercase().replace(" ", "_"), i);
    }
    map
});


impl Blocks {
    pub fn get_name(id: u32) -> &'static str {
        match id {
            0 => {"Air"}
            1 => {"Sand"}
            2 => {"Water"}
            3 => {"Grass"}
            4 => {"Dirt"}
            5 => {"Cobblestone"}
               6 => {"Wood"}
        7 => {"Leaves"}
            8 => {"Glass"}
            9 => {"Stone"}
            10 => {"Wood Planks"}
            11 => {"Bush Leaves"}
            12 => {"Petrified Wood"}
            13 => {"Red Stone"}
            14 => {"Salted Earth"}
            15 => {"Bedrock"}
            16 => {"Red Crystal Unattainable"}
            17 => {"Red Crystal"}
            18 => {"Light"}
            19 => {"Door"}
            20 => {"Ladder"}
            21 => {"Wooden Trunk"}
            22 => {"Bamboo"}
            23 => {"Tall Grass"}
            24 => {"Blue Light"}
            25 => {"Purple Light"}
            26 => {"Yellow Light"}
            27 => {"Red Light"}
            28 => {"Green Light"}
            29 => {"Orange Light"}
            30 => {"Teal Light"}
            31 => {"Crafting Bench"}

            32 => {"Apple"}
            33 => {"Bamboo Piece"}
            34 => {"Dead Leaf Mulch"}
            35 => {"Metal Rock"}
            36 => {"Crude Blade"}

            37 => {"Crude Pick"}
            38 => {"Crude Mattock"}
            39 => {"Crude Axe"}

            40 => {"Jump Switcher Block"}
            41 => {"Jump Switcher Block"}
            42 => {"Trampoline Block"}

            43 => {"Rubber Tree Wood"}
            44 => {"Rubber Tree Leaves"}
            45 => {"Conveyor"}
            46 => {"Auto Trampoline"}
            47 => {"Metal Plate Block"}
            48 => {"Snowy Grass Block"}
            49 => {"Torch"}
        50 => {"Snowy Leaves"}
            51 => {"Ice"}
            52 => {"Artic Willow Dwarf Shrub"}
              53 => {"Pine Wood"}
        54 => {"Pine Leaves"}
            55 => {"Artic Willow Leaves"}
              56 => {"Cedar Wood"}
        57 => {"Cedar Leaves"}
              58 => {"Palm Wood"}
        59 => {"Palm Leaves"}
              60 => {"Joshua Wood"}
        61 => {"Joshua Leaves"}
            62 => {"Snowy Sand"}
            63 => {"Fence"}
            64 => {"Paper Birch Wood"}
            65 => {"Paper Birch Leaves"}
            66 => {"Green Alder Wood"}
            67 => {"Green Alder Leaves"}
            68 => {"Willow Wood"}
            69 => {"Willow Leaves"}
            70 => {"Beech Wood"}
            71 => {"Beech Leaves"}
            72 => {"Western Hemlock Wood"}
            73 => {"Western Hemlock Leaves"}
            74 => {"Eucalyptus Wood"}

            75 => {"Eucalyptus Leaves"}
            76 => {"Fig Wood"}
            77 => {"Fig Leaves"}
            78 => {"Saguaro Block"}
            79 => {"Pumpkin"}
            80 => {"Jack o lantern"}
            
            81 => {"Pine Planks"}
            82 => {"Cedar Planks"}
            83 => {"Palm Planks"}
            84 => {"Joshua Planks"}
            85 => {"Birch Planks"}
            86 => {"Willow Planks"}
            87 => {"Beech Planks"}
            88 => {"Western Hemlock Planks"}
            89 => {"Eucalyptus Planks"}
            90 => {"Illuminite Ore"}

            _ => {
                "Unknown Item"
            }
        }
    }
    pub fn get_light_color(id: u32) -> LightColor {
        static WHITE: LightColor = LightColor{x: 10, y: 15, z:15};
        static BLUE: LightColor = LightColor{x: 0, y:0, z:15};
        static PURPLE: LightColor = LightColor{x: 7, y:0, z:10};
        static YELLOW: LightColor = LightColor{x: 15, y:15, z:0};

        static RED: LightColor = LightColor{x: 15, y:0, z:0};
        static GREEN: LightColor = LightColor{x: 0, y:15, z:0};
        static ORANGE: LightColor = LightColor{x: 15, y:7, z:0};
        static TEAL: LightColor = LightColor{x: 2, y:15, z:12};

        static TORCH: LightColor = LightColor{x: 8, y:8, z:8};

        static ILLUMINITE: LightColor = LightColor{x: 0, y:0, z:3};
        
        static JACKOLANTERN: LightColor = LightColor{x: 5, y:5, z:4};

        match id {
            18 => {
                WHITE
            }
            24 => {
                BLUE
            }
            25 => {
                PURPLE
            }
            49 => {
                TORCH
            }
            26 => {
                YELLOW
            }

            27 => {
                RED
            }
            28 => {
                GREEN
            }
            29 => {
                ORANGE
            }
            30 => {
                TEAL
            }
            80 => {
                JACKOLANTERN
            }
            90 => {
                ILLUMINITE
            }
            _ => {
                WHITE
            }
        }
    }
    pub fn get_break_time(id: u32) -> f32 {
        return BREAKTIMES[id as usize];
    }
    pub fn get_texs_length() -> usize {
        return TEXS.len();
    }
    pub fn get_tex_coords(id: u32, side: CubeSide) -> &'static (u8, u8) {
        static SIDES: [usize; 6] = [0, 0, 1, 2, 0, 0];

        let id = (id & Blocks::block_id_bits()).clamp(0, TEXS.len() as u32 - 1);
        
        return &TEXS[id as usize][SIDES[side as usize]];
    }

    pub fn is_overwritable(id: u32) -> bool {
        static OV: [u32; 2] = [
            0, 2
        ];
        return OV.contains(&id);
    }
    pub fn is_transparent(id: u32) -> bool {
        static TRANSPARENTS: [u32; 3] = [
            2, 8, 49
        ];
        return TRANSPARENTS.contains(&id);
    }
    pub fn is_climbable(id: u32) -> bool {
        static CLIMBABLES: [u32; 2] = [
            20, 22
        ];
        return CLIMBABLES.contains(&id);
    }
    pub fn is_semi_transparent(id: u32) -> bool {
        static SEMI_TRANSPARENTS: [u32; 22] = [
            7, 11, 19, 20, 21, 22, 23, 31, 44, 50, 52, 54, 55, 57, 59, 63,
            65, 67, 69, 75, 77, 80
        ];
        return SEMI_TRANSPARENTS.contains(&id);
    }
    pub fn is_non_placeable(id: u32) -> bool {
        static NP: [u32; 7] = [
            32, 33, 17, 36, 37, 38, 39
        ];
        return NP.contains(&id);
    }
    pub fn is_light(id: u32) -> bool {
        static LIGHTS: [u32; 11] = [
            18, 24, 25, 26, 27, 28, 29, 30, 49, 80, 90
        ];
        return LIGHTS.contains(&id);
    }
    pub fn is_food(id: u32) -> bool {
        static FOOD: [u32; 2] = [
            32, 33
        ];
        return FOOD.contains(&id);
    }

    pub fn block_id_bits() -> u32 {
        0b0000_0000_0000_0000_1111_1111_1111_1111
    }

    pub fn get_direction_bits(input: u32) -> u32 {
        return (input & BLOCK_DIRECTION_BITS) >> 16;
    }

    pub fn set_direction_bits(input: &mut u32, direction: u32) {
        let bits = direction << 16;
        *input |= bits;
    }



    pub fn block_flag_bits() -> u32 {
        0b1111_1111_1111_1111_0000_0000_0000_0000
    }
    pub fn get_food_stats(id: u32) -> (i32, i32) {
        match id {
            _ => {
                (6, 50)
            }
        }
    }
    pub fn get_walk_series(id: u32) -> &'static str {
        match id {
            62 | 48 | 50 => {
                "snowstepseries"
            }
            3 | 54 | 52 => {
                "grassstepseries"
            }
            34 => {
                "mulchstepseries"
            }
            7 | 65 | 67 | 69 | 71 | 73 | 75 | 77 => {
                "plantplaceseries"
            }
            11 | 57 | 59 => {
                "plantplaceseries"
            }
            1 => {
                "sandstepseries"
            }
            6 | 64 | 66 | 68 | 70 | 72 | 74 | 76 | 81..=89 => {
                "woodstepseries"
            }
            4 => {
                "dirtstepseries"
            }
            10 | 53 | 56 | 58 => {
                "woodstepseries"
            }
            22 => {
                "grassstepseries"
            }
            2 => {
                "waterstepseries"
            }
            _ => {
                "stonestepseries"
            }
        }
    }
    pub fn get_place_series(id: u32) -> &'static str {
        match id {
            3 | 48 | 54 => {
                "grassstepseries"
            }
            34 => {
                "mulchstepseries"
            }
            7 | 52 | 57 | 59 => {
                "plantplaceseries"
            }
            8 | 51 => {
                "glassplaceseries"
            }
            22 => {
                "plantplaceseries"
            }
            18 | 90 => {
                "glassplaceseries"
            }
            19 => {
                "doorseries"
            }
            11 => {
                "plantplaceseries"
            }
            _ => {
                "stoneplaceseries"
            }
        }
    }
    pub fn get_slickness(id: u32) -> f32 {
        match id {
            0 => {
                0.01
            }
            51 => {
                0.9
            }
            _ => {
                0.01
            }
        }
    }

    //for reusing textures
    pub fn get_block_tint(id: u32) -> Option<Vec3>{
        match id {
            _ => {
                None
            }
        }
    }
}
