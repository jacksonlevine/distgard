use std::{collections::HashSet, ptr::addr_of, time::Duration};

use arrayvec::ArrayVec;
use bevy::{math::Vec3, prelude::Resource, reflect::Array, time::Timer};
use dashmap::DashMap;
use glfw::ffi::glfwGetTime;
use lockfree::queue::Queue;
use once_cell::sync::Lazy;
use bevy::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{blockinfo::Blocks, chunk::{ChunkSystem, CH_H, CH_W}, game::{DeathType, Game, AUDIOPLAYER, CAMERA, CHUNKSYS, DEATHTYPE, PLAYERPOS}, modelentity::ModelEntity, planetinfo::Planets, rad::calculate_rotation};







pub static mut EVERIS_POSITION: Vec3 = Vec3::new(0.0, 80.0, 0.0);
pub static mut EVERIS_ROT: Vec3 = Vec3::new(0.0, 0.0, 0.0);

pub static mut EVERIS_VISIT_TIMER: f32 = 0.0;
pub static mut EVERIS_IS_VISITING: bool = false;


pub static mut CURRENT_VISIT_SPOT: IVec2 = IVec2{x:0, y:0};
pub static mut HIGHEST: i32 = 0;

pub const MAX_LIGHTS: usize = 13;

pub const VISIT_LENGTH: f32 = 10.0;


pub struct EverisVisitSpot{
    pub spot: IVec2,
    pub highestlight: i32
}
pub static mut EVERIS_VISIT_QUEUE: Lazy<Queue<EverisVisitSpot>> = Lazy::new(|| {
    Queue::new()
});

pub static QUEUED_FOR_EVERIS: Lazy<DashMap<IVec2, bool>> = Lazy::new(||DashMap::new());

pub static mut REMOVED_LIGHTS_YET: bool = false;

pub fn wait_or_visit_queued_spots(

    time: Res<Time>
) {

    unsafe {
        
        

        if EVERIS_IS_VISITING {
            if EVERIS_VISIT_TIMER <= VISIT_LENGTH {

                if EVERIS_VISIT_TIMER > (VISIT_LENGTH * 0.5) && !REMOVED_LIGHTS_YET {
                    

                    let csys = unsafe { CHUNKSYS.as_ref().unwrap() };

                    let mut implic: HashSet<_> = HashSet::new();

                    for y in 0..CH_H {
                        for x in 0..CH_W {
                            for z in 0..CH_W {
                                let spothere =IVec3::new(CURRENT_VISIT_SPOT.x * CH_W + x, y, CURRENT_VISIT_SPOT.y * CH_W + z);
                                let blockhere = csys.blockat(spothere);
                                let idhere = blockhere & Blocks::block_id_bits();

                                if Blocks::is_light(idhere) {
                                    Game::delete_block_recursively(csys, idhere, spothere, &mut implic);
                                    //csys.read().set_block_no_sound(spothere, 0, true);
                                }
                            }
                        }
                    }

                    for vec in implic {
                        csys.queue_rerender_with_key(vec, true, true);
                    }

                    
                    QUEUED_FOR_EVERIS.remove(&CURRENT_VISIT_SPOT);

                    // for source in unsafe { &currentdeletespots } {
                    //     csys.read().set_block_no_sound(IVec3::new(source.x, source.y, source.z), 0, true);
                    //     implic.insert(ChunkSystem::spot_to_chunk_pos(source));
                    // }
                    // for vec in implic {
                    //     csys.read().queue_rerender_with_key(vec, true, true);
                    // }
                    
                    


                    REMOVED_LIGHTS_YET = true;
                }
            }

            

        }
    }
    
}

pub static mut EVERIS_KILLED_YET: bool = false;

impl Game {

    pub fn update_everis(&self) {
        unsafe {
            if !EVERIS_IS_VISITING {

                match unsafe { &EVERIS_VISIT_QUEUE }.pop() {
                    Some(spot) => {
            
                        EVERIS_IS_VISITING = true;
                        CURRENT_VISIT_SPOT = spot.spot;
                        HIGHEST = spot.highestlight;
                        REMOVED_LIGHTS_YET = false;
                        #[cfg(feature="audio")]
                        {
                            let spothere = Vec3::new((CURRENT_VISIT_SPOT.x * CH_W + (CH_W / 2)) as f32, HIGHEST as f32, (CURRENT_VISIT_SPOT.y * CH_W + (CH_W / 2)) as f32);
                            AUDIOPLAYER.play_in_head(path!("assets/sfx/eye3.mp3"));
                        }
                        
                    }
                    None => {
                        
                    }
                }
    
            }
        }
    }

    pub fn draw_everis(&self) {


        #[cfg(feature = "glfw")]
        unsafe {

           if !EVERIS_KILLED_YET  && EVERIS_VISIT_TIMER > 6.0 {
                DEATHTYPE = DeathType::STATIC;

                self.take_damage_no_drops(200);
                
                EVERIS_KILLED_YET = true;
           }

            

            //gl::DepthMask(gl::FALSE);
            gl::Disable(gl::CULL_FACE);
            gl::UseProgram(self.modelshader.shader_id);
            let mvp_loc = gl::GetUniformLocation(self.modelshader.shader_id, b"mvp\0".as_ptr() as *const i8);

            

            static everis_modelentity: Lazy<ModelEntity> = Lazy::new(|| {
                let csys = unsafe { CHUNKSYS.as_ref().unwrap() };
                let cam = unsafe { CAMERA.as_ref().unwrap() };
                ModelEntity::new(4, unsafe { EVERIS_POSITION }, 1.0, unsafe { EVERIS_ROT }, csys, cam, false)
            });



            let modelents = vec![&everis_modelentity ];

            
            let camclone = {
                let cam = CAMERA.as_ref().unwrap();
                let cam_lock = cam.lock();
                cam_lock.clone()
                //Camera::new()
            };

            //EVERIS_ROT = calculate_rotation(EVERIS_POSITION, camclone.position);

            gl::UniformMatrix4fv(mvp_loc, 1, gl::FALSE, camclone.mvp.to_cols_array().as_ptr());
            gl::Uniform1i(
                gl::GetUniformLocation(
                    self.modelshader.shader_id,
                    b"ourTexture\0".as_ptr() as *const i8,
                ),
                1,
            );

            gl::Uniform1f(
                gl::GetUniformLocation(
                    self.modelshader.shader_id,
                    b"istitle\0".as_ptr() as *const i8,
                ),
                0.0,
            );

            gl::Uniform1f(
                gl::GetUniformLocation(
                    self.modelshader.shader_id,
                    b"time\0".as_ptr() as *const i8,
                ),
               0.0 //THIS IS ONLY NEEDED FOR THE MENU BUTTONS MOVING UP AND DOWN
            );
            gl::Uniform1f(
                gl::GetUniformLocation(
                    self.modelshader.shader_id,
                    b"isbutton\0".as_ptr() as *const i8,
                ),
                0.0,
            );
            gl::Uniform1f(
                gl::GetUniformLocation(
                    self.modelshader.shader_id,
                    b"issky\0".as_ptr() as *const i8,
                ),
                0.0,
            );
            gl::Uniform1f(
                gl::GetUniformLocation(
                    self.modelshader.shader_id,
                    b"buttonactive\0".as_ptr() as *const i8,
                ),
                0.0
            );


            gl::Uniform1f(
                gl::GetUniformLocation(
                    self.modelshader.shader_id,
                    b"walkbob\0".as_ptr() as *const i8,
                ),
                self.vars.walkbobtimer,
            );

            let distfromend =  ( (EVERIS_VISIT_TIMER - 8.0).max(0.0) / 2.0);

            gl::Uniform1f(
                gl::GetUniformLocation(
                    self.modelshader.shader_id,
                    b"opacity\0".as_ptr() as *const i8,
                ),
                (EVERIS_VISIT_TIMER as f32 * 0.5).min(1.0) - distfromend
            );

                
            let camdist = camclone.position.distance(EVERIS_POSITION);

            static mut WASFACE: bool = false;

            let renderface = camdist < 4.0 ;

            let indicestorender = vec![0]; //indices into the little vec declared earlier in this func

            // let shakeoffset = if !renderface { Vec3::ZERO } else {
            //     Vec3::new(RADRNG.gen_range(-0.1..0.1), RADRNG.gen_range(-0.1..0.1), RADRNG.gen_range(-0.1..0.1))
            // };

            let realpos = EVERIS_POSITION;// + shakeoffset;

            if renderface != WASFACE {
                if renderface {
                    #[cfg(feature = "audio")]
                    unsafe {
                        AUDIOPLAYER.play_next_in_series( "aseries", &realpos, &Vec3::ZERO, 1.0 );
                    }
                }
                WASFACE = renderface;
            }


            for inddd in indicestorender {
                let modelent = modelents[inddd];

                //let isjawbottom = inddd == 2;

                let ymod = 0.0; //if isjawbottom { -0.5 + ((glfwGetTime() * 100.0).sin() * 0.5) } else { 0.0 } as f32;
  
                let index = modelent.model_index;
                if index < self.gltf_vaos.len() && index < self.gltf_textures.len() {
                    //println!("Tis true");
                let vaosetset = &self.gltf_vaos[index];

                //info!("Doing Vaosetset {index}");
                let texsetset = &self.gltf_textures[index];

                for (ind, vaoset) in vaosetset.iter().enumerate() {
                    //info!("Doing Vaoset {ind} of Vaosetset {index}");

                    let texset = &texsetset[ind];

                    for(ii, vao) in vaoset.iter().enumerate() {
                        //info!("Doing Vao {ii} of Vaoset {ind} of Vaosetset {index}");
                        gl::BindVertexArray(*vao);

                            
                            if let Some(texture_id) = texset.get(0) {
                                gl::BindTextureUnit(1, *texture_id); 
                            }


                            gl::Uniform1f(
                                gl::GetUniformLocation(
                                    self.modelshader.shader_id,
                                    b"scale\0".as_ptr() as *const i8,
                                ),
                                modelent.scale,
                            );
                            //println!("This models scale is {}", modelent.scale);

         
                                    gl::Uniform3f(
                                        gl::GetUniformLocation(
                                            self.modelshader.shader_id,
                                            b"pos\0".as_ptr() as *const i8,
                                        ),
                                        realpos.x,
                                        realpos.y + self.planet_y_offset + ymod,
                                        realpos.z
                                    );
                      

                            gl::Uniform1f(
                                gl::GetUniformLocation(
                                    self.modelshader.shader_id,
                                    b"interp_time\0".as_ptr() as *const i8,
                                ),
                                glfwGetTime() as f32 - modelent.time_stamp as f32
                            );

                            gl::Uniform3f(
                                gl::GetUniformLocation(
                                    self.modelshader.shader_id,
                                    b"lastpos\0".as_ptr() as *const i8,
                                ),
                                realpos.x,
                                realpos.y + self.planet_y_offset + ymod,
                                realpos.z
                            );
                            

                            gl::Uniform1f(
                                gl::GetUniformLocation(
                                    self.modelshader.shader_id,
                                    b"xrot\0".as_ptr() as *const i8,
                                ),
                                EVERIS_ROT.x,
                            );
                            gl::Uniform1f(
                                gl::GetUniformLocation(
                                    self.modelshader.shader_id,
                                    b"yrot\0".as_ptr() as *const i8,
                                ),
                                EVERIS_ROT.y,
                            );

                            gl::Uniform1f(
                                gl::GetUniformLocation(
                                    self.modelshader.shader_id,
                                    b"zrot\0".as_ptr() as *const i8,
                                ),
                                EVERIS_ROT.z,
                            );

                       

                            let mut blocklighthere = 0.0;

                            let samplingcoord =IVec3::new(
                                modelent.position.x as i32,
                                modelent.position.y as i32,
                                modelent.position.z as i32
                            );
                            let csys = (*addr_of!(CHUNKSYS)).as_ref().unwrap();
                            let csyslock = csys;
                            let lmlock = csyslock.lightmap.lock();

                            match lmlock.get(&samplingcoord) {
                                Some(t) => {
                                    blocklighthere = t.sum().x as f32; //TEMPORARY USING RED
                                }
                                None => {

                                }
                            }

                            let scaledbl = blocklighthere / 16.0;
                            

                            gl::Uniform1f(
                                gl::GetUniformLocation(
                                    self.modelshader.shader_id,
                                    b"ambientBrightMult\0".as_ptr() as *const i8,
                                ),
                                if renderface { 1.0 } else { (self.ambient_bright_mult + scaledbl).clamp(0.0, 1.0) },
                            );
                    

                            

                            gl::Uniform3f(
                                gl::GetUniformLocation(
                                    self.modelshader.shader_id,
                                    b"camPos\0".as_ptr() as *const i8,
                                ),
                                camclone.position.x,
                                camclone.position.y,
                                camclone.position.z
                            );

                            gl::Uniform3f(
                                gl::GetUniformLocation(
                                    self.modelshader.shader_id,
                                    b"lastrot\0".as_ptr() as *const i8,
                                ),
                                EVERIS_ROT.x,
                                EVERIS_ROT.y,
                                EVERIS_ROT.z
                            );


                            gl::Uniform3f(
                                gl::GetUniformLocation(
                                    self.modelshader.shader_id,
                                    b"camDir\0".as_ptr() as *const i8,
                                ),
                                camclone.direction.x,
                                camclone.direction.y,
                                camclone.direction.z
                            );

                            gl::Uniform1f(
                                gl::GetUniformLocation(
                                    self.modelshader.shader_id,
                                    b"viewDistance\0".as_ptr() as *const i8,
                                ),
                                8.0
                            );

                            let fogcol = Planets::get_fog_col(0);

                            gl::Uniform4f(
                                gl::GetUniformLocation(
                                    self.modelshader.shader_id,
                                    b"fogCol\0".as_ptr() as *const i8,
                                ),
                                fogcol.0,
                                fogcol.1,
                                fogcol.2,
                                fogcol.3
                            );

                            gl::Uniform1f(gl::GetUniformLocation(
                                self.modelshader.shader_id,
                                b"sunset\0".as_ptr() as *const i8,
                            ), self.sunset_factor);
                            gl::Uniform1f(gl::GetUniformLocation(
                                self.modelshader.shader_id,
                                b"sunrise\0".as_ptr() as *const i8,
                            ), self.sunrise_factor);




                        
                        gl::DrawElements(self.gltf_drawmodes[index][ind][ii],  self.gltf_counts[index][ind][ii] as i32, gl::UNSIGNED_INT, std::ptr::null());
                    }
                    
                }
                }
            }
                //println!("Drawing a p");
                

             

                
                        

            gl::Enable(gl::CULL_FACE);
            //gl::DepthMask(gl::TRUE);
        }
    }

}