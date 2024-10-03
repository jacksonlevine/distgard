use once_cell::sync::Lazy;

use crate::specialblocks::vertexutils::rotate_coordinates_around_y_negative_90;
// use crate::textureface::TextureFace;


pub const CONNECT_X_BIT: u32 =    0b1000_0000_0000_0000_0000_0000_0000_0000;
pub const CONNECT_NEGX_BIT: u32 = 0b0100_0000_0000_0000_0000_0000_0000_0000;
pub const CONNECT_Z_BIT: u32 =    0b0010_0000_0000_0000_0000_0000_0000_0000;
pub const CONNECT_NEGZ_BIT: u32 = 0b0001_0000_0000_0000_0000_0000_0000_0000;

pub struct FenceInfo {
    
}


impl FenceInfo {



    pub fn fence_model_from_index(index: usize) -> &'static [f32] {
        FenceInfo::base_fence_model()
    }

    pub fn fence_model_from_combined(combined: u32) -> Vec<f32> {
        let mut model = FenceInfo::base_fence_model().to_vec();
        if combined & CONNECT_X_BIT != 0 {
            model.append(&mut FenceInfo::base_connecty_bit().to_vec());
        }

        if combined & CONNECT_Z_BIT != 0 {
            model.append(&mut rotate_coordinates_around_y_negative_90(FenceInfo::base_connecty_bit(), 1));
        }
        if combined & CONNECT_NEGX_BIT != 0 {
            model.append(&mut rotate_coordinates_around_y_negative_90(FenceInfo::base_connecty_bit(), 2));
        }
        if combined & CONNECT_NEGZ_BIT != 0 {
            model.append(&mut rotate_coordinates_around_y_negative_90(FenceInfo::base_connecty_bit(), 3));
        }
        model
    }

    pub fn fence_uvs_from_combined(combined: u32) -> Vec<f32> {
        let mut uvs = FenceInfo::get_fence_uvs();
        if combined & CONNECT_X_BIT != 0 {
            uvs.append(&mut FenceInfo::get_connecty_bit_uvs());
        }
        if combined & CONNECT_Z_BIT != 0 {
            uvs.append(&mut FenceInfo::get_connecty_bit_uvs());
        }
        if combined & CONNECT_NEGX_BIT != 0 {
            uvs.append(&mut FenceInfo::get_connecty_bit_uvs());
        }
        if combined & CONNECT_NEGZ_BIT != 0 {
            uvs.append(&mut FenceInfo::get_connecty_bit_uvs());
        }
        uvs
    }

    pub fn get_fence_uvs() -> Vec<f32> {


        let uvs = vec![
            0.027545038610696793, 0.14147299528121948, 0.0, 0.0,
0.03639437258243561, 0.11416018009185791, 0.0, 0.0,
0.027545038610696793, 0.11416023969650269, 0.0, 0.0,
0.00011203344911336899, 0.14147299528121948, 0.0, 0.0,
0.009403853677213192, 0.11416018009185791, 0.0, 0.0,
0.00011203344911336899, 0.11416018009185791, 0.0, 0.0,
0.018695689737796783, 0.14147299528121948, 0.0, 0.0,
0.027545038610696793, 0.11416018009185791, 0.0, 0.0,
0.018695689737796783, 0.11416018009185791, 0.0, 0.0,
0.009403853677213192, 0.14147299528121948, 0.0, 0.0,
0.01869567483663559, 0.11416018009185791, 0.0, 0.0,
0.009403853677213192, 0.11416018009185791, 0.0, 0.0,
0.008961383253335953, 0.10486841201782227, 0.0, 0.0,
0.00011203344911336899, 0.11416018009185791, 0.0, 0.0,
0.008961383253335953, 0.11416018009185791, 0.0, 0.0,
0.017810732126235962, 0.10486841201782227, 0.0, 0.0,
0.008961383253335953, 0.11416018009185791, 0.0, 0.0,
0.017810747027397156, 0.11416018009185791, 0.0, 0.0,
0.027545038610696793, 0.14147299528121948, 0.0, 0.0,
0.036394406110048294, 0.14147299528121948, 0.0, 0.0,
0.03639437258243561, 0.11416018009185791, 0.0, 0.0,
0.00011203344911336899, 0.14147299528121948, 0.0, 0.0,
0.009403853677213192, 0.14147299528121948, 0.0, 0.0,
0.009403853677213192, 0.11416018009185791, 0.0, 0.0,
0.018695689737796783, 0.14147299528121948, 0.0, 0.0,
0.027545038610696793, 0.14147299528121948, 0.0, 0.0,
0.027545038610696793, 0.11416018009185791, 0.0, 0.0,
0.009403853677213192, 0.14147299528121948, 0.0, 0.0,
0.018695689737796783, 0.14147299528121948, 0.0, 0.0,
0.01869567483663559, 0.11416018009185791, 0.0, 0.0,
0.008961383253335953, 0.10486841201782227, 0.0, 0.0,
0.00011203344911336899, 0.10486841201782227, 0.0, 0.0,
0.00011203344911336899, 0.11416018009185791, 0.0, 0.0,
0.017810732126235962, 0.10486841201782227, 0.0, 0.0,
0.008961383253335953, 0.10486841201782227, 0.0, 0.0,
0.008961383253335953, 0.11416018009185791, 0.0, 0.0,

            
            



        ];
        uvs
    }

    pub fn get_connecty_bit_uvs() -> Vec<f32> {


        let uvs = vec![
            0.01643194444477558, 0.17058950662612915, 0.0, 0.0,
0.010873440653085709, 0.15736490488052368, 0.0, 0.0,
0.01643194444477558, 0.1573648452758789, 0.0, 0.0,
0.021990463137626648, 0.17058950662612915, 0.0, 0.0,
0.01643194444477558, 0.15736490488052368, 0.0, 0.0,
0.021990463137626648, 0.15736490488052368, 0.0, 0.0,
0.005314921960234642, 0.1573648452758789, 0.0, 0.0,
-0.0002435926435282454, 0.17058950662612915, 0.0, 0.0,
-0.0002435934729874134, 0.15736490488052368, 0.0, 0.0,
0.010873433202505112, 0.1573648452758789, 0.0, 0.0,
0.005314921960234642, 0.17058950662612915, 0.0, 0.0,
0.005314921960234642, 0.15736490488052368, 0.0, 0.0,
0.01643194444477558, 0.17058950662612915, 0.0, 0.0,
0.010873440653085709, 0.15736490488052368, 0.0, 0.0,
0.01643194444477558, 0.1573648452758789, 0.0, 0.0,
0.021990463137626648, 0.17058950662612915, 0.0, 0.0,
0.01643194444477558, 0.15736490488052368, 0.0, 0.0,
0.021990463137626648, 0.15736490488052368, 0.0, 0.0,
0.005314921960234642, 0.1573648452758789, 0.0, 0.0,
-0.0002435926435282454, 0.17058950662612915, 0.0, 0.0,
-0.0002435934729874134, 0.15736490488052368, 0.0, 0.0,
0.010873433202505112, 0.1573648452758789, 0.0, 0.0,
0.005314921960234642, 0.17058950662612915, 0.0, 0.0,
0.005314921960234642, 0.15736490488052368, 0.0, 0.0,
0.01643194444477558, 0.17058950662612915, 0.0, 0.0,
0.010873440653085709, 0.17058950662612915, 0.0, 0.0,
0.010873440653085709, 0.15736490488052368, 0.0, 0.0,
0.021990463137626648, 0.17058950662612915, 0.0, 0.0,
0.016431963071227074, 0.17058950662612915, 0.0, 0.0,
0.01643194444477558, 0.15736490488052368, 0.0, 0.0,
0.005314921960234642, 0.1573648452758789, 0.0, 0.0,
0.005314921960234642, 0.17058950662612915, 0.0, 0.0,
-0.0002435926435282454, 0.17058950662612915, 0.0, 0.0,
0.010873433202505112, 0.1573648452758789, 0.0, 0.0,
0.010873440653085709, 0.17058950662612915, 0.0, 0.0,
0.005314921960234642, 0.17058950662612915, 0.0, 0.0,
0.01643194444477558, 0.17058950662612915, 0.0, 0.0,
0.010873440653085709, 0.17058950662612915, 0.0, 0.0,
0.010873440653085709, 0.15736490488052368, 0.0, 0.0,
0.021990463137626648, 0.17058950662612915, 0.0, 0.0,
0.016431963071227074, 0.17058950662612915, 0.0, 0.0,
0.01643194444477558, 0.15736490488052368, 0.0, 0.0,
0.005314921960234642, 0.1573648452758789, 0.0, 0.0,
0.005314921960234642, 0.17058950662612915, 0.0, 0.0,
-0.0002435926435282454, 0.17058950662612915, 0.0, 0.0,
0.010873433202505112, 0.1573648452758789, 0.0, 0.0,
0.010873440653085709, 0.17058950662612915, 0.0, 0.0,
0.005314921960234642, 0.17058950662612915, 0.0, 0.0,

            
            



        ];
        uvs
    }

    pub fn base_fence_model() -> &'static [f32] {
        static PLAYER_IS_MINUS_Z: [f32; 180] = [
           
        0.33670395612716675, 0.9800000190734863, 0.3444799780845642, 0.0, 14.0,
        0.33670395612716675, 0.019999980926513672, 0.6555200219154358, 0.0, 14.0,
        0.33670395612716675, 0.019999980926513672, 0.3444799780845642, 0.0, 14.0,
        0.33670395612716675, 0.9800000190734863, 0.6555200219154358, 0.0, 14.0,
        0.6632960438728333, 0.019999980926513672, 0.6555200219154358, 0.0, 14.0,
        0.33670395612716675, 0.019999980926513672, 0.6555200219154358, 0.0, 14.0,
        0.6632960438728333, 0.9800000190734863, 0.6555200219154358, 0.0, 14.0,
        0.6632960438728333, 0.019999980926513672, 0.3444799780845642, 0.0, 14.0,
        0.6632960438728333, 0.019999980926513672, 0.6555200219154358, 0.0, 14.0,
        0.6632960438728333, 0.9800000190734863, 0.3444799780845642, 0.0, 14.0,
        0.33670395612716675, 0.019999980926513672, 0.3444799780845642, 0.0, 14.0,
        0.6632960438728333, 0.019999980926513672, 0.3444799780845642, 0.0, 14.0,
        0.6632960438728333, 0.019999980926513672, 0.6555200219154358, 0.0, 14.0,
        0.33670395612716675, 0.019999980926513672, 0.3444799780845642, 0.0, 14.0,
        0.33670395612716675, 0.019999980926513672, 0.6555200219154358, 0.0, 14.0,
        0.33670395612716675, 0.9800000190734863, 0.6555200219154358, 0.0, 14.0,
        0.6632960438728333, 0.9800000190734863, 0.3444799780845642, 0.0, 14.0,
        0.6632960438728333, 0.9800000190734863, 0.6555200219154358, 0.0, 14.0,
        0.33670395612716675, 0.9800000190734863, 0.3444799780845642, 0.0, 14.0,
        0.33670395612716675, 0.9800000190734863, 0.6555200219154358, 0.0, 14.0,
        0.33670395612716675, 0.019999980926513672, 0.6555200219154358, 0.0, 14.0,
        0.33670395612716675, 0.9800000190734863, 0.6555200219154358, 0.0, 14.0,
        0.6632960438728333, 0.9800000190734863, 0.6555200219154358, 0.0, 14.0,
        0.6632960438728333, 0.019999980926513672, 0.6555200219154358, 0.0, 14.0,
        0.6632960438728333, 0.9800000190734863, 0.6555200219154358, 0.0, 14.0,
        0.6632960438728333, 0.9800000190734863, 0.3444799780845642, 0.0, 14.0,
        0.6632960438728333, 0.019999980926513672, 0.3444799780845642, 0.0, 14.0,
        0.6632960438728333, 0.9800000190734863, 0.3444799780845642, 0.0, 14.0,
        0.33670395612716675, 0.9800000190734863, 0.3444799780845642, 0.0, 14.0,
        0.33670395612716675, 0.019999980926513672, 0.3444799780845642, 0.0, 14.0,
        0.6632960438728333, 0.019999980926513672, 0.6555200219154358, 0.0, 14.0,
        0.6632960438728333, 0.019999980926513672, 0.3444799780845642, 0.0, 14.0,
        0.33670395612716675, 0.019999980926513672, 0.3444799780845642, 0.0, 14.0,
        0.33670395612716675, 0.9800000190734863, 0.6555200219154358, 0.0, 14.0,
        0.33670395612716675, 0.9800000190734863, 0.3444799780845642, 0.0, 14.0,
        0.6632960438728333, 0.9800000190734863, 0.3444799780845642, 0.0, 14.0,
        ];
        &PLAYER_IS_MINUS_Z
    }

    pub fn base_connecty_bit() -> &'static [f32] {
        static PLAYER_IS_MINUS_Z: [f32; 240] = [
           
        1.0031154155731201, 0.6200000047683716, 0.41999995708465576, 0.0, 14.0,
0.6224494576454163, 0.7799999713897705, 0.41999995708465576, 0.0, 14.0,
0.6224494576454163, 0.6200000047683716, 0.41999995708465576, 0.0, 14.0,
0.6224494576454163, 0.6200000047683716, 0.5800000429153442, 0.0, 14.0,
1.0031154155731201, 0.7799999713897705, 0.5800000429153442, 0.0, 14.0,
1.0031154155731201, 0.6200000047683716, 0.5800000429153442, 0.0, 14.0,
0.6224494576454163, 0.6200000047683716, 0.5800000429153442, 0.0, 14.0,
1.0031154155731201, 0.6200000047683716, 0.41999995708465576, 0.0, 14.0,
0.6224494576454163, 0.6200000047683716, 0.41999995708465576, 0.0, 14.0,
1.0031154155731201, 0.7799999713897705, 0.5800000429153442, 0.0, 14.0,
0.6224494576454163, 0.7799999713897705, 0.41999995708465576, 0.0, 14.0,
1.0031154155731201, 0.7799999713897705, 0.41999995708465576, 0.0, 14.0,
1.0003330707550049, 0.3199999928474426, 0.4200000762939453, 0.0, 14.0,
0.6196669936180115, 0.48000001907348633, 0.4200000762939453, 0.0, 14.0,
0.6196669936180115, 0.3199999928474426, 0.4200000762939453, 0.0, 14.0,
0.6196669936180115, 0.3199999928474426, 0.5800001621246338, 0.0, 14.0,
1.0003330707550049, 0.48000001907348633, 0.5800001621246338, 0.0, 14.0,
1.0003330707550049, 0.3199999928474426, 0.5800001621246338, 0.0, 14.0,
0.6196669936180115, 0.3199999928474426, 0.5800001621246338, 0.0, 14.0,
1.0003330707550049, 0.3199999928474426, 0.4200000762939453, 0.0, 14.0,
0.6196669936180115, 0.3199999928474426, 0.4200000762939453, 0.0, 14.0,
1.0003330707550049, 0.48000001907348633, 0.5800001621246338, 0.0, 14.0,
0.6196669936180115, 0.48000001907348633, 0.4200000762939453, 0.0, 14.0,
1.0003330707550049, 0.48000001907348633, 0.4200000762939453, 0.0, 14.0,
1.0031154155731201, 0.6200000047683716, 0.41999995708465576, 0.0, 14.0,
1.0031154155731201, 0.7799999713897705, 0.41999995708465576, 0.0, 14.0,
0.6224494576454163, 0.7799999713897705, 0.41999995708465576, 0.0, 14.0,
0.6224494576454163, 0.6200000047683716, 0.5800000429153442, 0.0, 14.0,
0.6224494576454163, 0.7799999713897705, 0.5800000429153442, 0.0, 14.0,
1.0031154155731201, 0.7799999713897705, 0.5800000429153442, 0.0, 14.0,
0.6224494576454163, 0.6200000047683716, 0.5800000429153442, 0.0, 14.0,
1.0031154155731201, 0.6200000047683716, 0.5800000429153442, 0.0, 14.0,
1.0031154155731201, 0.6200000047683716, 0.41999995708465576, 0.0, 14.0,
1.0031154155731201, 0.7799999713897705, 0.5800000429153442, 0.0, 14.0,
0.6224494576454163, 0.7799999713897705, 0.5800000429153442, 0.0, 14.0,
0.6224494576454163, 0.7799999713897705, 0.41999995708465576, 0.0, 14.0,
1.0003330707550049, 0.3199999928474426, 0.4200000762939453, 0.0, 14.0,
1.0003330707550049, 0.48000001907348633, 0.4200000762939453, 0.0, 14.0,
0.6196669936180115, 0.48000001907348633, 0.4200000762939453, 0.0, 14.0,
0.6196669936180115, 0.3199999928474426, 0.5800001621246338, 0.0, 14.0,
0.6196669936180115, 0.48000001907348633, 0.5800001621246338, 0.0, 14.0,
1.0003330707550049, 0.48000001907348633, 0.5800001621246338, 0.0, 14.0,
0.6196669936180115, 0.3199999928474426, 0.5800001621246338, 0.0, 14.0,
1.0003330707550049, 0.3199999928474426, 0.5800001621246338, 0.0, 14.0,
1.0003330707550049, 0.3199999928474426, 0.4200000762939453, 0.0, 14.0,
1.0003330707550049, 0.48000001907348633, 0.5800001621246338, 0.0, 14.0,
0.6196669936180115, 0.48000001907348633, 0.5800001621246338, 0.0, 14.0,
0.6196669936180115, 0.48000001907348633, 0.4200000762939453, 0.0, 14.0,
        ];
        &PLAYER_IS_MINUS_Z
    }
}