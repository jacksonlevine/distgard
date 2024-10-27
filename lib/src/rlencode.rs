
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

pub fn cpos_to_world(cpos: &IVec2) -> IVec3 {
    return IVec3::new(cpos.x * CH_W, 0, cpos.y * CH_W);
}

pub fn rlencode_chunk(cpos: &IVec2) -> String {
    let mut result = String::from("");
    let mut lastblock: i32 = -1;
    let mut count = 0;

    for (index, spot) in BLOCK_SEQ.iter().enumerate() {
        let realspot = cpos_to_world(cpos) + spot;
        let blockat = unsafe { HASH.get(&realspot).unwrap_or(&0) };
        if lastblock as u32 != *blockat  {
            //if its a new block, put the last run count and start a new run
            if count > 0 {
                result += (count.to_string() + "\n").as_str(); //put last count
            }
            lastblock = *blockat as i32;
            count = 1;
            result += (blockat.to_string() + " ").as_str(); //put new block id

        } 
        else {
            //if its the same block continue the same run
            count += 1;

        }

        if index == BLOCK_SEQ.len() - 1 {
            //we're at the end, put the count of whatever run we were counting
            result += (count.to_string() + "\n").as_str();
        }
    }

    return result;
}

pub fn rldecode_chunk(data: String) {
    let mut tokens = data.split_whitespace();
    let mut seq_index = 0;
    
    while let Some(idstring) = tokens.next() {
        let block = idstring.parse::<u32>().unwrap_or(0);

        if let Some(count_str) = tokens.next() {
            let count = count_str.parse::<usize>().unwrap_or(0);
            if block != 0 {
                for i in seq_index..(seq_index+count) {
                    unsafe { HASH.insert(BLOCK_SEQ[i], block) };
                }
            }
            seq_index += count;
        }
    }
}

fn main() {
    
    test_encode_decode();
    test_encode_decode2();
}


fn test_encode_decode() {
    for i in 0..100 {
        for x in 0..10 {
            unsafe { HASH.insert(IVec3::new(x, i, 0), 12) };
        }
    }
    let encoded = rlencode_chunk(&IVec2::new(0,0));

    unsafe { HASH.clear() };
    
    rldecode_chunk(encoded.clone());

    let encoded2 = rlencode_chunk(&IVec2::new(0,0));

    if encoded == encoded2 {
        println!("The blocks remain the same!");
    } else {
        println!("Stuff is messed up!");
    }
}

fn test_encode_decode2() {
    for i in 0..CH_H {
        for x in 0..CH_W {
            for z in 0..CH_W {
                unsafe { HASH.insert(IVec3::new(x, i, z), 12) };
            }
        }
    }
    let encoded = rlencode_chunk(&IVec2::new(0,0));

    unsafe { HASH.clear() };
    
    rldecode_chunk(encoded.clone());

    let encoded2 = rlencode_chunk(&IVec2::new(0,0));

    if encoded == encoded2 {
        println!("The blocks remain the same!");
    } else {
        println!("Stuff is messed up!");
    }
}
