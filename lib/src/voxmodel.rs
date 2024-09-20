





use vox_format::data::*;
use vox_format::types::*;

pub struct JVoxModel {
    pub model: VoxModels<Model>,
    pub idnumber: i32
}

impl JVoxModel {
    pub fn new(path: &'static str) -> JVoxModel {
        static mut ID_NUM: i32 = 0;
        unsafe {
            ID_NUM += 1;
            JVoxModel {
                model: vox_format::from_file(path).unwrap(),
                idnumber: ID_NUM
            }
        }
    }

}
