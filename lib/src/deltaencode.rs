use once_cell::sync::Lazy;

use crate::{chunk::{CH_H, CH_W}, vec::IVec3};


#[macro_export]
macro_rules! block_seq {
    ($ch_w:expr, $ch_h:expr) => {{
        const fn generate() -> [IVec3; $ch_w * $ch_w * $ch_h] {
            let mut arr = [IVec3{x:0,y:0,z:0}; $ch_w * $ch_w * $ch_h];
            let mut i = 0;
            let mut x = 0;
            while x < $ch_w {
                let mut z = 0;
                while z < $ch_w {
                    let mut y = 0;
                    while y < $ch_h {
                        arr[i] = IVec3{x: x as i32, y: y as i32, z: z as i32};
                        i += 1;
                        y += 1;
                    }
                    z += 1;
                }
                x += 1;
            }
            arr
        }
        generate()
    }};
}


pub const BLOCK_SEQ: [IVec3; (CH_W*CH_W*CH_H) as usize] = block_seq!(CH_W as usize, CH_H as usize);

