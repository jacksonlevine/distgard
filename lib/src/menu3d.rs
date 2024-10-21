use gl::types::{GLenum, GLuint};
use glfw::ffi::glfwGetTime;
use bevy::prelude::*;
use crate::{aabb::{ray_intersects_aabb, Ray, AABB}, camera::Camera, shader::Shader};



    pub fn draw_3d_menu_button(modelshader: &Shader, visions_camera: &Camera, gltf_vaos: &Vec<Vec<Vec<GLuint>>>, gltf_textures: &Vec<Vec<Vec<GLuint>>>,
    gltf_counts: &Vec<Vec<Vec<usize>>>,
    gltf_drawmodes: &Vec<Vec<Vec<GLenum>>>, moused: bool, pos: Vec3, element: usize) {

                let model_index = element;
                unsafe {

                    //gl::Clear(gl::DEPTH_BUFFER_BIT);

                    //gl::Disable(gl::DEPTH_TEST);
                    gl::Disable(gl::CULL_FACE);
                    gl::UseProgram(modelshader.shader_id);
            let mvp_loc = gl::GetUniformLocation(modelshader.shader_id, b"mvp\0".as_ptr() as *const i8);


            gl::UniformMatrix4fv(mvp_loc, 1, gl::FALSE, visions_camera.mvp.to_cols_array().as_ptr());
            gl::Uniform1i(
                gl::GetUniformLocation(
                    modelshader.shader_id,
                    b"ourTexture\0".as_ptr() as *const i8,
                ),
                1,
            );
            gl::Uniform1f(
                gl::GetUniformLocation(
                    modelshader.shader_id,
                    b"time\0".as_ptr() as *const i8,
                ),
                glfwGetTime() as f32,
            );
            gl::Uniform1f(
                gl::GetUniformLocation(
                    modelshader.shader_id,
                    b"buttonactive\0".as_ptr() as *const i8,
                ),
                if moused {1.0} else {0.0},
            );

            gl::Uniform1f(
                gl::GetUniformLocation(
                    modelshader.shader_id,
                    b"istitle\0".as_ptr() as *const i8,
                ),
                if element == 0 {1.0} else {0.0},
            );

            gl::Uniform1f(
                gl::GetUniformLocation(
                    modelshader.shader_id,
                    b"isbutton\0".as_ptr() as *const i8,
                ),
                if element != 0 {1.0} else {0.0},
            );

            gl::Uniform1f(
                gl::GetUniformLocation(
                    modelshader.shader_id,
                    b"issky\0".as_ptr() as *const i8,
                ),
                if element == 3 {1.0} else {0.0},
            );



                let index = model_index;
                let vaosetset = &gltf_vaos[index];

                //info!("Doing Vaosetset {index}");
                let texsetset = &gltf_textures[index];

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
                                    modelshader.shader_id,
                                    b"scale\0".as_ptr() as *const i8,
                                ),
                                1.0,
                            );

                                    gl::Uniform3f(
                                        gl::GetUniformLocation(
                                            modelshader.shader_id,
                                            b"pos\0".as_ptr() as *const i8,
                                        ),
                                        pos.x, pos.y, pos.z
                                    );



                            gl::Uniform1f(
                                gl::GetUniformLocation(
                                    modelshader.shader_id,
                                    b"interp_time\0".as_ptr() as *const i8,
                                ),
                                1.0
                            );

                            gl::Uniform3f(
                                gl::GetUniformLocation(
                                    modelshader.shader_id,
                                    b"lastpos\0".as_ptr() as *const i8,
                                ),
                                0.0, 0.0,0.0
                            );
                            

                            gl::Uniform1f(
                                gl::GetUniformLocation(
                                    modelshader.shader_id,
                                    b"xrot\0".as_ptr() as *const i8,
                                ),
                                0.0,
                            );
                            gl::Uniform1f(
                                gl::GetUniformLocation(
                                    modelshader.shader_id,
                                    b"yrot\0".as_ptr() as *const i8,
                                ),
                                std::f32::consts::PI
                            );

                            gl::Uniform1f(
                                gl::GetUniformLocation(
                                    modelshader.shader_id,
                                    b"opacity\0".as_ptr() as *const i8,
                                ),
                                1.0
                            );

                            gl::Uniform1f(
                                gl::GetUniformLocation(
                                    modelshader.shader_id,
                                    b"zrot\0".as_ptr() as *const i8,
                                ),
                                0.0,
                            );

                            gl::Uniform1f(
                                gl::GetUniformLocation(
                                    modelshader.shader_id,
                                    b"ambientBrightMult\0".as_ptr() as *const i8,
                                ),
                                1.0,
                            );

                            gl::Uniform3f(
                                gl::GetUniformLocation(
                                    modelshader.shader_id,
                                    b"camPos\0".as_ptr() as *const i8,
                                ),
                                visions_camera.position.x,
                                visions_camera.position.y,
                                visions_camera.position.z
                            );

                            gl::Uniform3f(
                                gl::GetUniformLocation(
                                    modelshader.shader_id,
                                    b"lastrot\0".as_ptr() as *const i8,
                                ),
                                0.0,0.0,0.0
                            );


                            gl::Uniform3f(
                                gl::GetUniformLocation(
                                    modelshader.shader_id,
                                    b"camDir\0".as_ptr() as *const i8,
                                ),
                                visions_camera.direction.x,
                                visions_camera.direction.y,
                                visions_camera.direction.z
                            );

                            gl::Uniform1f(
                                gl::GetUniformLocation(
                                    modelshader.shader_id,
                                    b"viewDistance\0".as_ptr() as *const i8,
                                ),
                                8.0
                            );

                            gl::Uniform4f(
                                gl::GetUniformLocation(
                                    modelshader.shader_id,
                                    b"fogCol\0".as_ptr() as *const i8,
                                ),
                                1.0, 0.0 , 0.0, 1.0
                            );

                            gl::Uniform1f(gl::GetUniformLocation(
                                modelshader.shader_id,
                                b"sunset\0".as_ptr() as *const i8,
                            ), 0.0);
                            gl::Uniform1f(gl::GetUniformLocation(
                                modelshader.shader_id,
                                b"sunrise\0".as_ptr() as *const i8,
                            ), 0.0);




                        
                        gl::DrawElements(gltf_drawmodes[index][ind][ii],  gltf_counts[index][ind][ii] as i32, gl::UNSIGNED_INT, std::ptr::null());
                    }
                    
                }
                //gl::Enable(gl::DEPTH_TEST);
                    gl::Enable(gl::CULL_FACE);
                }
      
        



    }








    pub fn draw_3d_menu_button_with_mvp(modelshader: &Shader, visions_camera: &Camera, gltf_vaos: &Vec<Vec<Vec<GLuint>>>, gltf_textures: &Vec<Vec<Vec<GLuint>>>,
        gltf_counts: &Vec<Vec<Vec<usize>>>,
        gltf_drawmodes: &Vec<Vec<Vec<GLenum>>>, moused: bool, pos: Vec3, element: usize, mvp: &Mat4) {





                let aabb = AABB{
                    min: pos + Vec3::new(-0.2, -0.1, -0.2) + Vec3::new(0.0, -0.3, 0.0),
                    max: pos - Vec3::new(-0.2, -0.1, -0.2) + Vec3::new(0.0, -0.3, 0.0),
                };
                let camray = Ray {
                    direction: visions_camera.direction,
                    origin: visions_camera.position
                };

                let moused = ray_intersects_aabb(camray, aabb);

                    let model_index = element;
                    unsafe {
    
                        //gl::Clear(gl::DEPTH_BUFFER_BIT);
    
                        //gl::Disable(gl::DEPTH_TEST);
                        gl::Disable(gl::CULL_FACE);
                        gl::UseProgram(modelshader.shader_id);
                let mvp_loc = gl::GetUniformLocation(modelshader.shader_id, b"mvp\0".as_ptr() as *const i8);
    
    
                gl::UniformMatrix4fv(mvp_loc, 1, gl::FALSE, mvp.to_cols_array().as_ptr());
                gl::Uniform1i(
                    gl::GetUniformLocation(
                        modelshader.shader_id,
                        b"ourTexture\0".as_ptr() as *const i8,
                    ),
                    1,
                );
                gl::Uniform1f(
                    gl::GetUniformLocation(
                        modelshader.shader_id,
                        b"time\0".as_ptr() as *const i8,
                    ),
                    glfwGetTime() as f32,
                );
                gl::Uniform1f(
                    gl::GetUniformLocation(
                        modelshader.shader_id,
                        b"buttonactive\0".as_ptr() as *const i8,
                    ),
                    if moused {1.0} else {0.0},
                );
    
                gl::Uniform1f(
                    gl::GetUniformLocation(
                        modelshader.shader_id,
                        b"istitle\0".as_ptr() as *const i8,
                    ),
                    if element == 0 {1.0} else {0.0},
                );
    
                gl::Uniform1f(
                    gl::GetUniformLocation(
                        modelshader.shader_id,
                        b"isbutton\0".as_ptr() as *const i8,
                    ),
                    if element != 0 {1.0} else {0.0},
                );
    
                gl::Uniform1f(
                    gl::GetUniformLocation(
                        modelshader.shader_id,
                        b"issky\0".as_ptr() as *const i8,
                    ),
                    if element == 3 {1.0} else {0.0},
                );
    
    
    
                    let index = model_index;
                    let vaosetset = &gltf_vaos[index];
    
                    //info!("Doing Vaosetset {index}");
                    let texsetset = &gltf_textures[index];
    
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
                                        modelshader.shader_id,
                                        b"scale\0".as_ptr() as *const i8,
                                    ),
                                    0.5,
                                );
    
                                        gl::Uniform3f(
                                            gl::GetUniformLocation(
                                                modelshader.shader_id,
                                                b"pos\0".as_ptr() as *const i8,
                                            ),
                                            pos.x, pos.y, pos.z
                                        );
    
    
    
                                gl::Uniform1f(
                                    gl::GetUniformLocation(
                                        modelshader.shader_id,
                                        b"interp_time\0".as_ptr() as *const i8,
                                    ),
                                    1.0
                                );
    
                                gl::Uniform3f(
                                    gl::GetUniformLocation(
                                        modelshader.shader_id,
                                        b"lastpos\0".as_ptr() as *const i8,
                                    ),
                                    0.0, 0.0,0.0
                                );
                                
    
                                gl::Uniform1f(
                                    gl::GetUniformLocation(
                                        modelshader.shader_id,
                                        b"xrot\0".as_ptr() as *const i8,
                                    ),
                                    0.0,
                                );
                                gl::Uniform1f(
                                    gl::GetUniformLocation(
                                        modelshader.shader_id,
                                        b"yrot\0".as_ptr() as *const i8,
                                    ),
                                    std::f32::consts::PI
                                );
    
                                gl::Uniform1f(
                                    gl::GetUniformLocation(
                                        modelshader.shader_id,
                                        b"opacity\0".as_ptr() as *const i8,
                                    ),
                                    1.0
                                );
    
                                gl::Uniform1f(
                                    gl::GetUniformLocation(
                                        modelshader.shader_id,
                                        b"zrot\0".as_ptr() as *const i8,
                                    ),
                                    0.0,
                                );
    
                                gl::Uniform1f(
                                    gl::GetUniformLocation(
                                        modelshader.shader_id,
                                        b"ambientBrightMult\0".as_ptr() as *const i8,
                                    ),
                                    1.0,
                                );
    
                                gl::Uniform3f(
                                    gl::GetUniformLocation(
                                        modelshader.shader_id,
                                        b"camPos\0".as_ptr() as *const i8,
                                    ),
                                    visions_camera.position.x,
                                    visions_camera.position.y,
                                    visions_camera.position.z
                                );
    
                                gl::Uniform3f(
                                    gl::GetUniformLocation(
                                        modelshader.shader_id,
                                        b"lastrot\0".as_ptr() as *const i8,
                                    ),
                                    0.0,0.0,0.0
                                );
    
    
                                gl::Uniform3f(
                                    gl::GetUniformLocation(
                                        modelshader.shader_id,
                                        b"camDir\0".as_ptr() as *const i8,
                                    ),
                                    visions_camera.direction.x,
                                    visions_camera.direction.y,
                                    visions_camera.direction.z
                                );
    
                                gl::Uniform1f(
                                    gl::GetUniformLocation(
                                        modelshader.shader_id,
                                        b"viewDistance\0".as_ptr() as *const i8,
                                    ),
                                    8.0
                                );
    
                                gl::Uniform4f(
                                    gl::GetUniformLocation(
                                        modelshader.shader_id,
                                        b"fogCol\0".as_ptr() as *const i8,
                                    ),
                                    1.0, 0.0 , 0.0, 1.0
                                );
    
                                gl::Uniform1f(gl::GetUniformLocation(
                                    modelshader.shader_id,
                                    b"sunset\0".as_ptr() as *const i8,
                                ), 0.0);
                                gl::Uniform1f(gl::GetUniformLocation(
                                    modelshader.shader_id,
                                    b"sunrise\0".as_ptr() as *const i8,
                                ), 0.0);
    
    
    
    
                            
                            gl::DrawElements(gltf_drawmodes[index][ind][ii],  gltf_counts[index][ind][ii] as i32, gl::UNSIGNED_INT, std::ptr::null());
                        }
                        
                    }
                    //gl::Enable(gl::DEPTH_TEST);
                        gl::Enable(gl::CULL_FACE);
                    }
          
            
    
    
    
        }