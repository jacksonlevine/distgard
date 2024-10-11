use std::{ptr::addr_of, time::Duration};

use bevy::{math::Vec3, prelude::Resource, time::Timer};
use glfw::ffi::glfwGetTime;
use once_cell::sync::Lazy;
use bevy::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{game::{Game, AUDIOPLAYER, CAMERA, CHUNKSYS, PLAYERPOS}, modelentity::ModelEntity, planetinfo::Planets, vec};







pub static mut RAD_POSITION: Vec3 = Vec3::new(0.0, 80.0, 0.0);
pub static mut RAD_ROT: Vec3 = Vec3::new(0.0, 0.0, 0.0);

static mut RADRNG: Lazy<StdRng> = Lazy::new(||StdRng::from_entropy());

#[derive(Resource)]
pub struct RadRecordPositionTimer(pub Timer);

#[derive(Resource)]
pub struct RadCyclePositionTimer(pub Timer);

#[derive(Resource)]
pub struct RadPositionsList(pub Vec<Vec3>);


pub fn record_and_sort_rad_positions(time: Res<Time>, mut timer: ResMut<RadRecordPositionTimer>, mut rad_positions: ResMut<RadPositionsList>) {
    if timer.0.tick(Duration::from_secs_f32(time.delta_seconds())).just_finished() {
        let pos = unsafe { PLAYERPOS.snapshot() };
        let newspot = Vec3::new(pos.pos.0, pos.pos.1, pos.pos.2);

        

        if !rad_positions.0.contains(&newspot) {
            rad_positions.0.push(newspot);
            println!("Recorded a rad position at {}, {}, {}", pos.pos.0, pos.pos.1, pos.pos.2);
        }

        
    }
}

pub fn cycle_rad_positions(time: Res<Time>, mut timer: ResMut<RadCyclePositionTimer>, mut rad_positions: ResMut<RadPositionsList>) {
    
    if timer.0.tick(Duration::from_secs_f32(time.delta_seconds())).just_finished() {

        let pos = unsafe { PLAYERPOS.snapshot() };

        rad_positions.0.retain(|x| {
            let d = x.distance(Vec3::new(pos.pos.0, pos.pos.1, pos.pos.2));
            return d < 100.0 && d > 20.0;
        });
        rad_positions.0.sort_by(|a, b| {
            let d1 = a.distance(Vec3::new(pos.pos.0, pos.pos.1, pos.pos.2));
            let d2 = b.distance(Vec3::new(pos.pos.0, pos.pos.1, pos.pos.2));
            d1.partial_cmp(&d2).unwrap()
        });
        let len = rad_positions.0.len();
        rad_positions.0.truncate(len.min(10));

        if rad_positions.0.len() > 0 {

            let randomind = unsafe { RADRNG.gen_range(0..rad_positions.0.len()) };
            
            let pos = rad_positions.0.get(randomind).unwrap().clone();
            unsafe {
                RAD_POSITION = pos;
            }
            println!("Cycling to a rad position at {}, {}, {}", pos.x, pos.y, pos.z);
        }
    }
}


fn calculate_rotation(v1: Vec3, v2: Vec3) -> Vec3 {
    // Compute the direction vector
    let dir = v2 - v1;

    // Normalize the direction vector
    let dir_norm = dir.normalize();

    // Calculate pitch (rotation around X-axis) and yaw (rotation around Y-axis)
    let pitch = dir_norm.y.atan2((dir_norm.z).hypot(dir_norm.x)); // Up/down rotation
    let yaw = dir_norm.x.atan2(dir_norm.z);                       // Left/right rotation
    let roll = 0.0;                                               // Roll can be 0 unless needed

    // Return the Vec3 of radian rotations
    Vec3::new(pitch, yaw, roll)
}

//implement the draw_rad function
impl Game {

    pub fn draw_rad(&self) {


        #[cfg(feature = "glfw")]
        unsafe {

            

            //gl::DepthMask(gl::FALSE);
            gl::Disable(gl::CULL_FACE);
            gl::UseProgram(self.modelshader.shader_id);
            let mvp_loc = gl::GetUniformLocation(self.modelshader.shader_id, b"mvp\0".as_ptr() as *const i8);

            

            static rad_model_entity: Lazy<ModelEntity> = Lazy::new(|| {
                let csys = unsafe { CHUNKSYS.as_ref().unwrap() };
                let cam = unsafe { CAMERA.as_ref().unwrap() };
                ModelEntity::new(1, unsafe { RAD_POSITION }, 1.0, unsafe { RAD_ROT }, csys, cam, false)
            });


            static rad_face_top: Lazy<ModelEntity> = Lazy::new(|| {
                let csys = unsafe { CHUNKSYS.as_ref().unwrap() };
                let cam = unsafe { CAMERA.as_ref().unwrap() };
                ModelEntity::new(2, unsafe { RAD_POSITION }, 1.0, unsafe { RAD_ROT }, csys, cam, false)
            });

            static rad_face_bottom: Lazy<ModelEntity> = Lazy::new(|| {
                let csys = unsafe { CHUNKSYS.as_ref().unwrap() };
                let cam = unsafe { CAMERA.as_ref().unwrap() };
                ModelEntity::new(3, unsafe { RAD_POSITION }, 1.0, unsafe { RAD_ROT }, csys, cam, false)
            });

            let modelents = vec![&rad_model_entity, &rad_face_top, &rad_face_bottom];

            
            let camclone = {
                let cam = CAMERA.as_ref().unwrap();
                let cam_lock = cam.lock();
                cam_lock.clone()
                //Camera::new()
            };

            RAD_ROT = calculate_rotation(RAD_POSITION, camclone.position);

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

            gl::Uniform1f(
                gl::GetUniformLocation(
                    self.modelshader.shader_id,
                    b"opacity\0".as_ptr() as *const i8,
                ),
                1.0
            );

                
            let camdist = camclone.position.distance(RAD_POSITION);

            static mut WASFACE: bool = false;

            let renderface = camdist < 4.0 ;

            let indicestorender = if renderface {
                vec![1, 2]
            } else {
                vec![0]
            };

            let shakeoffset = if !renderface { Vec3::ZERO } else {
                Vec3::new(RADRNG.gen_range(-0.1..0.1), RADRNG.gen_range(-0.1..0.1), RADRNG.gen_range(-0.1..0.1))
            };

            let realpos = RAD_POSITION + shakeoffset;

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
                                RAD_ROT.x,
                            );
                            gl::Uniform1f(
                                gl::GetUniformLocation(
                                    self.modelshader.shader_id,
                                    b"yrot\0".as_ptr() as *const i8,
                                ),
                                RAD_ROT.y,
                            );

                            gl::Uniform1f(
                                gl::GetUniformLocation(
                                    self.modelshader.shader_id,
                                    b"zrot\0".as_ptr() as *const i8,
                                ),
                                RAD_ROT.z,
                            );

                       

                            let mut blocklighthere = 0.0;

                            let samplingcoord = vec::IVec3::new(
                                modelent.position.x as i32,
                                modelent.position.y as i32,
                                modelent.position.z as i32
                            );
                            let csys = (*addr_of!(CHUNKSYS)).as_ref().unwrap();
                            let csyslock = csys.read();
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
                                RAD_ROT.x,
                                RAD_ROT.y,
                                RAD_ROT.z
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