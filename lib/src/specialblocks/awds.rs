use once_cell::sync::Lazy;

use crate::specialblocks::vertexutils::rotate_coordinates_around_y_negative_90;
// use crate::textureface::TextureFace;







pub struct AwdsInfo {
    
}


impl AwdsInfo {



    pub fn awds_model_from_index(index: usize) -> &'static [f32] {
        AwdsInfo::base_awds_model()
    }

    pub fn get_awds_uvs() -> Vec<f32> {


        let uvs = vec![
            0.10952672362327576, 0.10352814197540283, 0.0, 0.0,
            0.013783721253275871, 0.007785141468048096, 0.0, 0.0,
            0.10952672362327576, 0.007785141468048096, 0.0, 0.0,
            0.10952672362327576, 0.10352814197540283, 0.0, 0.0,
            0.10952672362327576, 0.007785141468048096, 0.0, 0.0,
            0.013783721253275871, 0.007785141468048096, 0.0, 0.0,
            0.10952672362327576, 0.10352814197540283, 0.0, 0.0,
            0.013783721253275871, 0.007785141468048096, 0.0, 0.0,
            0.013783721253275871, 0.10352814197540283, 0.0, 0.0,
            0.10952672362327576, 0.10352814197540283, 0.0, 0.0,
            0.013783721253275871, 0.10352814197540283, 0.0, 0.0,
            0.013783721253275871, 0.007785141468048096, 0.0, 0.0,
            0.10952672362327576, 0.10352814197540283, 0.0, 0.0,
            0.013783721253275871, 0.007785141468048096, 0.0, 0.0,
            0.013783721253275871, 0.10352814197540283, 0.0, 0.0,
            0.10952672362327576, 0.10352814197540283, 0.0, 0.0,
            0.10952672362327576, 0.007785141468048096, 0.0, 0.0,
            0.013783721253275871, 0.007785141468048096, 0.0, 0.0,
            0.10952672362327576, 0.10352814197540283, 0.0, 0.0,
            0.10952672362327576, 0.007785141468048096, 0.0, 0.0,
            0.013783721253275871, 0.007785141468048096, 0.0, 0.0,
            0.10952672362327576, 0.10352814197540283, 0.0, 0.0,
            0.013783721253275871, 0.007785141468048096, 0.0, 0.0,
            0.10952672362327576, 0.007785141468048096, 0.0, 0.0,
            0.10952672362327576, 0.10352814197540283, 0.0, 0.0,
            0.013783721253275871, 0.10352814197540283, 0.0, 0.0,
            0.013783721253275871, 0.007785141468048096, 0.0, 0.0,
            0.10952672362327576, 0.10352814197540283, 0.0, 0.0,
            0.013783721253275871, 0.007785141468048096, 0.0, 0.0,
            0.013783721253275871, 0.10352814197540283, 0.0, 0.0,
            0.10952672362327576, 0.10352814197540283, 0.0, 0.0,
            0.013783721253275871, 0.10352814197540283, 0.0, 0.0,
            0.013783721253275871, 0.007785141468048096, 0.0, 0.0,
            0.10952672362327576, 0.10352814197540283, 0.0, 0.0,
            0.013783721253275871, 0.007785141468048096, 0.0, 0.0,
            0.10952672362327576, 0.007785141468048096, 0.0, 0.0,
            
            


        ];
        uvs
    }

    pub fn base_awds_model() -> &'static [f32] {
        static PLAYER_IS_MINUS_Z: [f32; 180] = [
            1.0233936309814453, -0.01798945665359497, -0.08723944425582886, 0.0, 14.0,
-0.022547125816345215, 1.531323790550232, 1.0557303428649902, 0.0, 14.0,
1.0233936309814453, 1.5313236713409424, -0.08723944425582886, 0.0, 14.0,
-0.23424899578094482, -0.01798945665359497, 0.23858359456062317, 0.0, 14.0,
-0.23424899578094482, 1.5313236713409424, 0.23858359456062317, 0.0, 14.0,
1.235095500946045, 1.531323790550232, 0.7299075126647949, 0.0, 14.0,
0.9083948135375977, -0.01798945665359497, 1.142768383026123, 0.0, 14.0,
0.09245166182518005, 1.531323790550232, -0.17427721619606018, 0.0, 14.0,
0.0924517810344696, -0.017989397048950195, -0.17427721619606018, 0.0, 14.0,
1.0233936309814453, -0.01798945665359497, -0.08723944425582886, 0.0, 14.0,
-0.02254718542098999, -0.017989397048950195, 1.0557303428649902, 0.0, 14.0,
-0.022547125816345215, 1.531323790550232, 1.0557303428649902, 0.0, 14.0,
-0.23424899578094482, -0.01798945665359497, 0.23858359456062317, 0.0, 14.0,
1.235095500946045, 1.531323790550232, 0.7299075126647949, 0.0, 14.0,
1.2350953817367554, -0.017989397048950195, 0.7299075722694397, 0.0, 14.0,
0.9083948135375977, -0.01798945665359497, 1.142768383026123, 0.0, 14.0,
0.9083948135375977, 1.5313236713409424, 1.142768383026123, 0.0, 14.0,
0.09245166182518005, 1.531323790550232, -0.17427721619606018, 0.0, 14.0,
1.0233936309814453, -0.01798945665359497, -0.08723944425582886, 0.0, 14.0,
1.0233936309814453, 1.5313236713409424, -0.08723944425582886, 0.0, 14.0,
-0.022547125816345215, 1.531323790550232, 1.0557303428649902, 0.0, 14.0,
-0.23424899578094482, -0.01798945665359497, 0.23858359456062317, 0.0, 14.0,
1.235095500946045, 1.531323790550232, 0.7299075126647949, 0.0, 14.0,
-0.23424899578094482, 1.5313236713409424, 0.23858359456062317, 0.0, 14.0,
0.9083948135375977, -0.01798945665359497, 1.142768383026123, 0.0, 14.0,
0.0924517810344696, -0.017989397048950195, -0.17427721619606018, 0.0, 14.0,
0.09245166182518005, 1.531323790550232, -0.17427721619606018, 0.0, 14.0,
1.0233936309814453, -0.01798945665359497, -0.08723944425582886, 0.0, 14.0,
-0.022547125816345215, 1.531323790550232, 1.0557303428649902, 0.0, 14.0,
-0.02254718542098999, -0.017989397048950195, 1.0557303428649902, 0.0, 14.0,
-0.23424899578094482, -0.01798945665359497, 0.23858359456062317, 0.0, 14.0,
1.2350953817367554, -0.017989397048950195, 0.7299075722694397, 0.0, 14.0,
1.235095500946045, 1.531323790550232, 0.7299075126647949, 0.0, 14.0,
0.9083948135375977, -0.01798945665359497, 1.142768383026123, 0.0, 14.0,
0.09245166182518005, 1.531323790550232, -0.17427721619606018, 0.0, 14.0,
0.9083948135375977, 1.5313236713409424, 1.142768383026123, 0.0, 14.0,

            
            
            

        ];
        &PLAYER_IS_MINUS_Z
    }
}