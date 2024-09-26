use std::ptr::{addr_of, addr_of_mut};
use bevy::prelude::*;
use gl::types::{GLenum, GLuint};
use gltf::mesh::util::ReadIndices;

use crate::{
    audio::spawn_audio_thread, blockinfo::Blocks, cmd::Cmd, game::{
        Game, JGltfNode, AUDIOPLAYER, CAMERA, CROUCHING, CURRENT_AVAIL_RECIPES, DECIDEDSPORMP, MOUSEX, MOUSEY, SHOWTOOLTIP, SINGLEPLAYER, TOOLTIPNAME
    }, keybinds::{AboutToRebind, ABOUTTOREBIND, LISTENINGFORREBIND}, menu3d::draw_3d_menu_button, newclient::{ADDRESSENTERED, THEENTEREDADDRESS}, recipes::{RECIPES_DISABLED, RECIPE_COOLDOWN_TIMER}, statics::{
        load_misc, load_or_initialize_statics, save_lesa, LAST_ENTERED_SERVERADDRESS, MISCSETTINGS
    }, texture::Texture
};

// clipboard support
use clipboard::{ClipboardContext, ClipboardProvider};
use imgui::ClipboardBackend;

// TODO: can we use glfw as the clipboard backend?
pub struct ClipboardSupport(pub ClipboardContext);

impl ClipboardBackend for ClipboardSupport {
    fn get(&mut self) -> Option<String> {
        self.0.get_contents().ok()
    }
    fn set(&mut self, text: &str) {
        let _ = self.0.set_contents(text.to_owned());
    }
}

use glfw::{
    Action, Context, Glfw, GlfwReceiver, Key, //Modifiers,
    PWindow, WindowEvent,
};

use once_cell::sync::Lazy;

use imgui::Key as ImGuiKey;
use imgui::*;
use imgui_opengl_renderer::Renderer;
use parking_lot::{Mutex, RwLock};
use std::{
    sync::{atomic::AtomicBool, Arc},
    time::{Duration, Instant},
};

use imgui::Condition;

pub static mut WINDOWWIDTH: i32 = 0;
pub static mut WINDOWHEIGHT: i32 = 0;

pub static mut UNCAPKB: Lazy<Arc<AtomicBool>> = Lazy::new(|| Arc::new(AtomicBool::new(false)));

pub struct WindowAndKeyContext {
    pub width: u32,
    pub height: u32,
    pub game: Option<Game>,
    pub cmd: Cmd,

    pub previous_time: Instant,
    pub delta_time: f32,

    pub glfw: Glfw,
    pub window: Arc<RwLock<PWindow>>,
    pub events: GlfwReceiver<(f64, WindowEvent)>,

    pub imgui: imgui::Context,
    pub guirenderer: imgui_opengl_renderer::Renderer,
    pub addressentered: Arc<AtomicBool>,
    pub serveraddress: Arc<Mutex<Option<String>>>,

    pub serveraddrbuffer: String,

    pub logo: Texture,
    pub menu_camera: crate::camera::Camera,

    pub gltf_models: Vec<(
        gltf::Document,
        Vec<gltf::buffer::Data>,
        Vec<gltf::image::Data>,
    )>,
    pub gltf_vbos: Vec<Vec<Vec<GLuint>>>,
    pub gltf_vaos: Vec<Vec<Vec<GLuint>>>,
    pub gltf_counts: Vec<Vec<Vec<usize>>>,
    pub gltf_drawmodes: Vec<Vec<Vec<GLenum>>>,
    pub gltf_ebos: Vec<Vec<Vec<GLuint>>>,
    pub gltf_textures: Vec<Vec<Vec<GLuint>>>,
    pub gltf_paths: Vec<String>,
    pub modelshader: crate::shader::Shader,

    pub nodes: Vec<Vec<JGltfNode>>,

    #[cfg(feature = "steam")]
    pub client: Arc<Client>,
    #[cfg(feature = "steam")]
    pub single: SingleClient,
}

fn toggle_fullscreen(window_ptr: *mut glfw::ffi::GLFWwindow) {
    unsafe {
        let monitor = glfw::ffi::glfwGetWindowMonitor(window_ptr);
        if monitor.is_null() {
            let primary_monitor = glfw::ffi::glfwGetPrimaryMonitor();
            if !primary_monitor.is_null() {
                let mode = glfw::ffi::glfwGetVideoMode(primary_monitor);
                if !mode.is_null() {
                    glfw::ffi::glfwSetWindowMonitor(
                        window_ptr,
                        primary_monitor,
                        0,
                        0,
                        (*mode).width as i32,
                        (*mode).height as i32,
                        glfw::ffi::DONT_CARE,
                    );
                }
            }
        } else {
            glfw::ffi::glfwSetWindowMonitor(
                window_ptr,
                std::ptr::null_mut(),
                100,
                100,
                1280,
                720,
                glfw::ffi::DONT_CARE,
            );
        }
    }
}

// use steamworks::{restart_app_if_necessary, AppId, Client, SingleClient};

pub static MAINMENUSONG: &str = path!("assets/music/dd2dd3.mp3");

impl WindowAndKeyContext {

    pub fn new(windowname: &'static str, width: u32, height: u32) -> Self {
        #[cfg(feature = "steam")]
        let (client, single) = Client::init().unwrap();
        // #[cfg(feature = "steam")]
        // restart_app_if_necessary(AppId::from(3114230));

        unsafe {
            WINDOWHEIGHT = height as i32;
            WINDOWWIDTH = width as i32;
        }

        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        let (mut window, events) = glfw
            .create_window(width, height, windowname, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");
        gl::load_with(|s| window.get_proc_address(s) as *const _);

        load_misc();

        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);
        window.set_char_polling(true);
        window.set_char_mods_polling(true);
        window.make_current();

        // Initialize ImGui
        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);
        imgui.set_clipboard_backend(ClipboardContext::new().ok().map(ClipboardSupport).unwrap());

        let scale_factor = 1.0;

        imgui.io_mut().font_global_scale = scale_factor;
        imgui.style_mut().scale_all_sizes(scale_factor);

        {
            let io = imgui.io_mut();

            io.key_map[ImGuiKey::Backspace as usize] = Key::Backspace as u32;
            io.key_map[ImGuiKey::Tab as usize] = Key::Tab as u32;
            io.key_map[ImGuiKey::LeftArrow as usize] = Key::Left as u32;
            io.key_map[ImGuiKey::RightArrow as usize] = Key::Right as u32;
            io.key_map[ImGuiKey::UpArrow as usize] = Key::Up as u32;
            io.key_map[ImGuiKey::DownArrow as usize] = Key::Down as u32;
            io.key_map[ImGuiKey::PageUp as usize] = Key::PageUp as u32;
            io.key_map[ImGuiKey::PageDown as usize] = Key::PageDown as u32;
            io.key_map[ImGuiKey::Home as usize] = Key::Home as u32;
            io.key_map[ImGuiKey::End as usize] = Key::End as u32;
            io.key_map[ImGuiKey::Insert as usize] = Key::Insert as u32;
            io.key_map[ImGuiKey::Delete as usize] = Key::Delete as u32;
            io.key_map[ImGuiKey::Backspace as usize] = Key::Backspace as u32;
            io.key_map[ImGuiKey::Space as usize] = Key::Space as u32;
            io.key_map[ImGuiKey::Enter as usize] = Key::Enter as u32;
            io.key_map[ImGuiKey::Escape as usize] = Key::Escape as u32;
            io.key_map[ImGuiKey::C as usize] = Key::C as u32;
            io.key_map[ImGuiKey::V as usize] = Key::V as u32;
        }

        let font_size = 16.0;
        imgui.fonts().add_font(&[FontSource::TtfData {
            data: include_bytes!("../../font.ttf"),
            size_pixels: font_size,
            config: Some(FontConfig {
                oversample_h: 1,
                oversample_v: 1,
                pixel_snap_h: true,
                size_pixels: 72.0,
                ..Default::default()
            }),
        }]);

        let renderer = Renderer::new(&mut imgui, |s| window.get_proc_address(s) as *const _);

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
            gl::FrontFace(gl::CW);
        }

        let mut menu_camera = crate::camera::Camera::new();
        menu_camera.position = Vec3::new(0.0, 3.0, -5.0);
        menu_camera.recalculate();

        #[cfg(feature = "audio")]
        {
            unsafe {
                AUDIOPLAYER.preload(path!("assets/sfx/mclick1.mp3"), path!("assets/sfx/mclick1.mp3"));
                AUDIOPLAYER.preload(path!("assets/sfx/mclickgo.mp3"), path!("assets/sfx/mclickgo.mp3"));
            }
        }

        let mut wak = WindowAndKeyContext {
            menu_camera,
            width,
            height,
            game: None,
            cmd: Cmd::new(),
            previous_time: Instant::now(),
            delta_time: 0.0,
            glfw,
            window: Arc::new(RwLock::new(window)),
            events,
            imgui,
            guirenderer: renderer,
            addressentered: Arc::new(AtomicBool::new(false)),
            serveraddress: Arc::new(Mutex::new(None)),
            serveraddrbuffer: String::with_capacity(128),
            logo: Texture::new(path!(
                "assets/Untitled3.png"
            ))
            .unwrap_or_else(|err| {
                eprintln!(
                    "Error: {err:?}, path: {}",
                    path!(
                        "assets/Untitled3.png"
                    )
                );
                panic!("Error!!!!!!!!1111, {err:?}");
            }),
            nodes: Vec::new(),
            gltf_models: Vec::new(),
            gltf_vbos: Vec::new(),
            gltf_vaos: Vec::new(),
            gltf_counts: Vec::new(),
            gltf_drawmodes: Vec::new(),
            gltf_ebos: Vec::new(),
            gltf_textures: Vec::new(),
            gltf_paths: Vec::new(),
            modelshader: crate::shader::Shader::new(path!("assets/mvert.glsl"), path!("assets/mfrag.glsl")),
            #[cfg(feature = "steam")]
            client: Arc::new(client),
            #[cfg(feature = "steam")]
            single,
        };

        load_or_initialize_statics();
        unsafe {
            wak.serveraddrbuffer = (*LAST_ENTERED_SERVERADDRESS).clone();
            wak.serveraddrbuffer.reserve(100);
        }

        #[cfg(feature = "glfw")]
        {
            wak.load_model(path!("assets/models/menulogo.gltf"));
            wak.load_model(path!("assets/models/menubuttontop.gltf"));
            wak.load_model(path!("assets/models/menubuttonbottom.gltf"));
            wak.load_model(path!("assets/models/skybox.gltf"));
            wak.create_model_vbos();
        }

        
            

        wak
    }

    #[cfg(feature = "glfw")]
    pub fn load_model(&mut self, path: &'static str) {
        use std::path::Path;

        use gltf::animation::util::ReadOutputs;

        use crate::game::{AnimationChannel, JGltfNode, Joint};

        let (document, buffers, images) = gltf::import(path).expect("Failed to load model");
        self.gltf_models.push((document.clone(), buffers.clone(), images.clone()));
        let path = Path::new(path);
        let gp = path.parent()
            .map(|p| p.to_str().unwrap_or(""))
            .unwrap_or("")
            .to_string();
        self.gltf_paths.push(gp);

        //let animindex = self.animations.len();
        let nodeindex = self.nodes.len();

        //self.animations.push(Vec::new());
        self.nodes.push(Vec::new());

        for animation in document.animations() {
            let mut channels = Vec::new();
            for channel in animation.channels() {
                let _sampler = channel.sampler();
                let reader = channel.reader(|buffer| Some(&buffers[buffer.index()]));
                let inputs = reader.read_inputs().unwrap().collect::<Vec<f32>>();
                let outputs: Vec<Vec<f32>> = match reader.read_outputs().unwrap() {
                    ReadOutputs::Translations(translations) => translations.map(|v| v.to_vec()).collect(),
                    ReadOutputs::Rotations(rotations) => rotations.into_f32().map(|v| v.to_vec()).collect(),
                    ReadOutputs::Scales(scales) => scales.map(|v| v.to_vec()).collect(),
                    ReadOutputs::MorphTargetWeights(weights) => weights.into_f32().collect::<Vec<f32>>().iter().map(|w| vec![*w]).collect(),
                };

                let keyframes = inputs.into_iter().zip(outputs).collect();

                channels.push(AnimationChannel {
                    node_index: channel.target().node().index(),
                    property: channel.target().property(),
                    keyframes,
                });
            }



           
        }

        for skin in document.skins() {
            let reader = skin.reader(|buffer| Some(&buffers[buffer.index()]));
            let inverse_bind_matrices: Vec<Mat4> = reader.read_inverse_bind_matrices().unwrap().map(|m| {
                let flat: Vec<f32> = m.iter().flatten().cloned().collect();
                Mat4::from_cols_array(&flat.try_into().expect("Slice with incorrect length"))
            }).collect();

            let _joints: Vec<Joint> = skin.joints()
                .zip(inverse_bind_matrices)
                .map(|(joint, inverse_bind_matrix)| Joint {
                    node_index: joint.index(),
                    inverse_bind_matrix,
                })
                .collect();


        }

        for node in document.nodes() {
            self.nodes[nodeindex].push(JGltfNode {
                transform: Mat4::from_cols_array_2d(&node.transform().matrix()),
                children: node.children().map(|child| child.index()).collect(),
            });
        }
    }
    
    fn collect_indices(data: ReadIndices) -> Vec<u32> {
        match data {
            ReadIndices::U8(iter) => {
                iter.map(|index| index as u32).collect()
            },
            ReadIndices::U16(iter) => {
                iter.map(|index| index as u32).collect()
            },
            ReadIndices::U32(iter) => {
                iter.collect()
            },
        }
    }
    #[cfg(feature = "glfw")]
    pub fn create_model_vbos(&mut self) {
        use gl::types::{GLsizeiptr, GLvoid};

        use crate::model::load_document_textures;

        for (index, (document, buffers, _images)) in self.gltf_models.iter().enumerate() {
            self.gltf_counts.push(Vec::new());
            self.gltf_drawmodes.push(Vec::new());
            self.gltf_vaos.push(Vec::new());
            self.gltf_vbos.push(Vec::new());
            self.gltf_textures.push(Vec::new());

            let textures = load_document_textures(&document, &buffers, self.gltf_paths[index].as_str());

            for mesh in document.meshes() {
                let mut mesh_vbos = Vec::new();
                let mut mesh_vaos = Vec::new();
                let mut mesh_counts = Vec::new();
                let mut mesh_drawmodes = Vec::new();
                let mut textures_here = Vec::new();
                
                for primitive in mesh.primitives() {

                    let material = primitive.material();
                    let pbr = material.pbr_metallic_roughness();

                    let default_texture_index = 0;

                    let base_color_texture_index = pbr.base_color_texture().map(|info| info.texture().index())
                    .or_else(|| {
                        document.textures().nth(0).map(|tex| tex.index()) // Example: Just grab the first texture if available
                    })
                    .unwrap_or(default_texture_index);

                    textures_here.push(textures[base_color_texture_index]);

                    //if let Some((_, accessor)) = primitive.attributes().find(|(semantic, _)| *semantic == Semantic::Positions) {
                        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                        let positions = reader.read_positions().unwrap().collect::<Vec<_>>();
                        let indices = WindowAndKeyContext::collect_indices(reader.read_indices().unwrap()); 
                        let uvs = reader.read_tex_coords(0).unwrap().into_f32().collect::<Vec<_>>();

                        let mut ebo: GLuint = 0;
                        unsafe {
                            gl::CreateBuffers(1, &mut ebo);
                            gl::NamedBufferData(
                                ebo,
                                (indices.len() * std::mem::size_of::<u32>()) as GLsizeiptr,
                                indices.as_ptr() as *const GLvoid,
                                gl::STATIC_DRAW,  // Usage hint
                            );
                        }


                        //let vertex_count = positions.len();
                        let index_count = indices.len();
                        let mut vbo: GLuint = 0;



                        let mut uv_vbo: GLuint = 0;

                        unsafe {
                            gl::CreateBuffers(1, &mut vbo);
                            gl::NamedBufferData(
                                vbo,
                                (positions.len() * std::mem::size_of::<[f32; 3]>()) as GLsizeiptr,
                                positions.as_ptr() as *const GLvoid,
                                gl::STATIC_DRAW,
                            );


                            gl::CreateBuffers(1, &mut uv_vbo);
                            gl::NamedBufferData(
                                uv_vbo,
                                (uvs.len() * std::mem::size_of::<[f32; 2]>()) as GLsizeiptr,
                                uvs.as_ptr() as *const GLvoid,
                                gl::STATIC_DRAW,
                            );
                        }

                        mesh_vbos.push(vbo);
                        mesh_counts.push(index_count);
                        mesh_drawmodes.push(primitive.mode().as_gl_enum());
                        

                        // Create VAO
                        let mut vao: GLuint = 0;
                        unsafe {
                            gl::CreateVertexArrays(1, &mut vao);
                            gl::VertexArrayVertexBuffer(vao, 0, vbo, 0, (3 * std::mem::size_of::<f32>()) as i32);
                            gl::EnableVertexArrayAttrib(vao, 0);
                            gl::VertexArrayAttribFormat(vao, 0, 3, gl::FLOAT, gl::FALSE, 0);
                            gl::VertexArrayAttribBinding(vao, 0, 0);
                            

                            gl::VertexArrayVertexBuffer(vao, 1, uv_vbo, 0, (2 * std::mem::size_of::<f32>()) as i32);
                            gl::EnableVertexArrayAttrib(vao, 1);
                            gl::VertexArrayAttribFormat(vao, 1, 2, gl::FLOAT, gl::FALSE, 0);
                            gl::VertexArrayAttribBinding(vao, 1, 1);


                            gl::VertexArrayElementBuffer(vao, ebo);

                        }
                        mesh_vaos.push(vao);
                    //}
                }
                self.gltf_vbos[index].push(mesh_vbos.clone());
                println!("Adding {} length vbo list to gltfvbos", mesh_vbos.len());
                self.gltf_vaos[index].push(mesh_vaos);
                self.gltf_counts[index].push(mesh_counts);
                self.gltf_drawmodes[index].push(mesh_drawmodes);
                self.gltf_textures[index].push(textures_here);
            }
        }
    }

    pub fn run(&mut self) {
        #[cfg(feature = "glfw")]
        self.glfw.poll_events();

        #[cfg(feature = "steam")]
        self.single.run_callbacks();

        let current_time = Instant::now();
        self.delta_time = current_time
            .duration_since(self.previous_time)
            .as_secs_f32();
        self.previous_time = current_time;

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        }

        let mut main_menu = false;

        unsafe {
            match DECIDEDSPORMP {
                false => {

                    static mut PLAY_SONG: bool = true;

                    
                    
                    

                    if PLAY_SONG {
                        #[cfg(feature = "audio")]
                        {
                            spawn_audio_thread();
                            AUDIOPLAYER.play_in_head(MAINMENUSONG);
                            PLAY_SONG = false;
                        }
                        
                    }

                    self.imgui
                        .io_mut()
                        .update_delta_time(Duration::from_secs_f32(self.delta_time));

                    let (width, height) = self.window.read().get_framebuffer_size();
                    self.imgui.io_mut().display_size = [width as f32, height as f32];

                    let screen_width = width as f32;
                    let screen_height = height as f32;

                    // Start the ImGui frame
                    let ui = self.imgui.frame();

                    let window_flags = WindowFlags::NO_DECORATION
                        | WindowFlags::NO_MOVE
                        | WindowFlags::NO_RESIZE
                        | WindowFlags::NO_SCROLLBAR
                        | WindowFlags::NO_TITLE_BAR
                        | WindowFlags::NO_BACKGROUND;

                    // Scale the window and button size proportionally to the screen size
                    let window_size = (screen_width * 0.75, screen_height * 0.75);
                    let window_pos = [
                        screen_width / 2.0 - (window_size.0 / 2.0),
                        screen_height / 2.0 - (window_size.1 / 2.0) + screen_height * 0.1, // Slightly offset vertically
                    ];

                    ui.window("Transparent Window")
                        .size([window_size.0, window_size.1], Condition::Always)
                        .position(window_pos, Condition::Always)
                        .flags(window_flags)
                        .build(|| {
                            let button_width = screen_width * 0.15; // Scale button width by 15% of screen width
                            let button_height = screen_height * 0.07; // Scale button height by 7% of screen height
                            let window_size = ui.window_size();

                            let available_width = window_size[0];
                            let available_height = window_size[1];

                            let pos_x = (available_width - button_width) / 2.0;
                            let pos_y = (available_height - (button_height) - 10.0) / 2.0;

                            // Adjust the image size and position
                            let scaled_size = [
                                (self.logo.size.0 as f32 * screen_width / 1280.0).round(), // Scale based on reference size
                                (self.logo.size.1 as f32 * screen_height / 720.0).round(),
                            ];
                            let image_pos_x = (available_width - scaled_size[0]) / 2.0;
                            let image_pos_y = ((available_height - scaled_size[1]) / 2.0) - screen_height * 0.2;

                            // commands dont work in menu yet
                            // if self.cmd.cmd_open {
                            //     ui.set_cursor_pos([0f32, 0f32]);
                            //     if ui.input_text("##", &mut self.cmd.cmd_text)
                            //     .enter_returns_true(true)
                            //     .build() {
                            //         self.cmd.run(self.game.as_mut().unwrap());
                            //     }
                            // }

                            ui.set_cursor_pos([image_pos_x, image_pos_y]);

                            draw_3d_menu_button(
                                &self.modelshader, &self.menu_camera, 
                                &self.gltf_vaos, &self.gltf_textures, 
                                &self.gltf_counts, &self.gltf_drawmodes,
                                false, Vec3::new(0.0, 0.0, 0.0), 3
                            );

                            // Draw 3D menu button for the image
                            draw_3d_menu_button(
                                &self.modelshader, &self.menu_camera, 
                                &self.gltf_vaos, &self.gltf_textures, 
                                &self.gltf_counts, &self.gltf_drawmodes,
                                false, Vec3::new(0.40, 4.5, 0.0), 0
                            );

                            // ui.set_cursor_pos([pos_x - screen_width * 0.04, pos_y - screen_height * 0.08]);
                            // ui.text_colored([1.0, 0.0, 0.0, 1.0], "Welcome! Please choose an option.");

                            // Center the button and adjust its size
                            ui.set_cursor_pos([pos_x, pos_y - screen_height * 0.02]);

                            // Make button invisible but functional
                            let sttok1 = ui.push_style_var(StyleVar::Alpha(0.0));

                            // Singleplayer button
                            if ui.button_with_size("Singleplayer", [button_width, button_height]) {
                                #[cfg(feature="audio")]
                                {
                                    AUDIOPLAYER.play_in_head(path!("assets/sfx/mclickgo.mp3"));
                                }
                                SINGLEPLAYER = true;
                                DECIDEDSPORMP = true;
                            }

                            static mut ELEMENT1MOUSED: bool = false;
                            let hovered = ui.is_item_hovered();

                            if hovered != unsafe { ELEMENT1MOUSED } {
                                unsafe {
                                    ELEMENT1MOUSED = hovered;
                                }
                                #[cfg(feature="audio")]
                                {
                                    AUDIOPLAYER.play_in_head(path!("assets/sfx/mclick1.mp3"));
                                }
                            }

                            draw_3d_menu_button(
                                &self.modelshader, &self.menu_camera, 
                                &self.gltf_vaos, &self.gltf_textures, 
                                &self.gltf_counts, &self.gltf_drawmodes,
                                hovered, Vec3::new(0.0, 2.75, 0.0), 1
                            );

                                    ui.set_cursor_pos([pos_x, pos_y + screen_height * 0.08  - screen_height * 0.01]);

                            // Multiplayer button
                            if ui.button_with_size("Multiplayer", [button_width, button_height]) {
                                #[cfg(feature="audio")]
                                {
                                    AUDIOPLAYER.play_in_head(path!("assets/sfx/mclickgo.mp3"));
                                }
                                SINGLEPLAYER = false;
                                DECIDEDSPORMP = true;
                            }

                            static mut ELEMENT2MOUSED: bool = false;
                            let hovered = ui.is_item_hovered();

                            if hovered != unsafe { ELEMENT2MOUSED } {
                                unsafe {
                                    ELEMENT2MOUSED = hovered;
                                }
                                #[cfg(feature="audio")]
                                {
                                    AUDIOPLAYER.play_in_head(path!("assets/sfx/mclick1.mp3"));
                                }
                            }
                            draw_3d_menu_button(
                                &self.modelshader, &self.menu_camera, 
                                &self.gltf_vaos, &self.gltf_textures, 
                                &self.gltf_counts, &self.gltf_drawmodes,
                                hovered, Vec3::new(0.0, 2.0, 0.0), 2
                            );

                            // Pop the button style after use
                            sttok1.pop();
                        });
                    // Render the ImGui frame
                    self.guirenderer.render(&mut self.imgui);

                    // avoid borrow checker
                    (*addr_of_mut!(*self)).handle_events(self.imgui.io_mut())
                }
                true => {
                    match self.game.as_mut() {
                        Some(g) => {
                            let gmenuopen = g.vars.menu_open;

                            let gcraftopen = g.crafting_open;

                            #[cfg(feature = "glfw")]
                            let gchestopen = g.hud.chest_open;

                            #[cfg(not(feature = "glfw"))]
                            let gchestopen = false;

                            if g.vars.main_menu {
                                main_menu = true;
                            } else {
                                if g.loadedworld.load(std::sync::atomic::Ordering::Relaxed) {
                                    g.update();

                                    // if MISCSETTINGS.controllersupport == true {
                                    //     let state = self.glfw.get_joystick(glfw::JoystickId::Joystick1);

                                    //     static mut lastx: f64 = 0.0;
                                    //     static mut lasty: f64 = 0.0;

                                    //     static mut x: f64 = 0.0;
                                    //     static mut y: f64 = 0.0;

                                    //     let axes = state.get_axes();

                                    //     if axes.len() >= 2 {
                                    //         unsafe {
                                    //             x += axes[0] as f64;
                                    //             y += axes[1] as f64;

                                    //             if lastx != x || lasty != y {
                                    //                 lastx = x;
                                    //                 lasty = y;
                                    //                 g.cursor_pos(x, y);
                                    //             }
                                    //         }
                                    //     }
                                    // }
                                    
                                }

                                self.imgui
                                    .io_mut()
                                    .update_delta_time(Duration::from_secs_f32(self.delta_time));

                                if UNCAPKB.load(std::sync::atomic::Ordering::Relaxed) {
                                    self.imgui.io_mut().want_capture_keyboard = false;
                                    self.imgui.io_mut().want_text_input = false;
                                    self.imgui.io_mut().want_capture_mouse = false;
                                    UNCAPKB.store(false, std::sync::atomic::Ordering::Relaxed);
                                }

                                if gchestopen {
                                    let ui = self.imgui.frame();

                                    let window_flags = WindowFlags::NO_DECORATION
                                        | WindowFlags::NO_MOVE
                                        | WindowFlags::NO_RESIZE
                                        | WindowFlags::NO_SCROLLBAR
                                        | WindowFlags::NO_TITLE_BAR
                                        | WindowFlags::NO_INPUTS;

                                    if SHOWTOOLTIP {
                                        ui.window("Mouse Tooltip Window")
                                            .size([300.0, 50.0], Condition::Always)
                                            .position(
                                                [MOUSEX as f32, MOUSEY as f32],
                                                Condition::Always,
                                            )
                                            .flags(window_flags)
                                            .build(|| {
                                                ui.text(TOOLTIPNAME);
                                            });
                                    }

                                    self.guirenderer.render(&mut self.imgui);
                                }

                                if gmenuopen {
                                    let gamecurrentbuttons = g.currentbuttons.clone();

                                    let (width, height) = self.window.read().get_framebuffer_size();
                                    self.imgui.io_mut().display_size =
                                        [width as f32, height as f32];

                                    // Start the ImGui frame
                                    let ui = self.imgui.frame();

                                    let window_flags = WindowFlags::NO_DECORATION
                                        | WindowFlags::NO_MOVE
                                        | WindowFlags::NO_RESIZE
                                        | WindowFlags::NO_SCROLLBAR
                                        | WindowFlags::NO_TITLE_BAR
                                        | WindowFlags::NO_BACKGROUND;

                                    let window_size = (900.0, 700.0);

                                    let window_pos = [
                                        width as f32 / 2.0 - (window_size.0 / 2.0),
                                        height as f32 / 2.0 - (window_size.1 / 2.0),
                                    ];

                                    // unsafe {
                                    //     UNCAPKB.store(false, std::sync::atomic::Ordering::Relaxed);
                                    // }

                                    ui.window("Transparent Window")
                                        .size([window_size.0, window_size.1], Condition::Always)
                                        .position(window_pos, Condition::Always)
                                        .flags(window_flags)
                                        .build(|| {

                                            let len = gamecurrentbuttons.len();

                                            let button_height = 20.0;
                                            let window_size = ui.window_size();
                                            let available_width = window_size[0];
                                            let available_height = window_size[1] + 200.0;
                                            let mut pos_y = (available_height - (len as f32 * button_height) - 10.0 * (len as f32 - 1.0)) / 2.0;
            

                                            if gamecurrentbuttons.len() > 0 {

                                                if gamecurrentbuttons[0].0 == "bindings" {

                                                    
                                                    
                                                    for (index, (binding, glfwkey)) in gamecurrentbuttons.iter().skip(1).enumerate() {
                                                        
                                                        let button_width = 10.0 * 20.0;
                                                    

                                                        let pos_x = (available_width - (button_width * 2.0)) / 2.0;

                                                        ui.set_cursor_pos([pos_x, pos_y + 25.0]);
                                                        if LISTENINGFORREBIND {
                                                            ui.text_colored([1.0, 1.0, 0.0, 1.0], "Listening for new key binding...");
                                                        }
    
                                                        ui.set_cursor_pos([pos_x, pos_y]);
    
                                                        

                                                        if index == 0 {
                                                            if ui.button_with_size(binding, [button_width, button_height]) {
                                                                g.button_command(glfwkey.to_string());
                                                                UNCAPKB.store(true, std::sync::atomic::Ordering::Relaxed);
                                                            }
                                                        } else {
                                                            if ui.button_with_size(binding, [button_width, button_height]) {
                                                             
                                                            }
    
                                                            ui.set_cursor_pos([pos_x + button_width, pos_y]);
                                                            

                                                            // let name = if glfwkey.starts_with("Button") { glfwkey } else { 
                                                            //     &glfw::get_key_name(None, Some(glfwkey.parse::<i32>().unwrap_or(0))).unwrap_or("Unknown key".into())
                                                            // };
                                                            let name = glfwkey;

                                                            let int = glfwkey.parse::<i32>().unwrap_or(1);

                                                        

                                                            // if !glfwkey.starts_with("Button") {
                                                            //     name = &realname;
                                                            // }
                                                            
                                                            if !name.is_empty() {
                                                                if ui.button_with_size(name, [button_width, button_height]) {
                                                                
                                                                    LISTENINGFORREBIND = true;
                                                                    if !glfwkey.starts_with("Button") {
                                                                        ABOUTTOREBIND = Some(AboutToRebind {
                                                                            key: crate::keybinds::Rebindable::Key(int),
                                                                            action: binding.clone()
                                                                        });
                                                                    } else {
                                                                        ABOUTTOREBIND = Some(AboutToRebind {
                                                                            key: crate::keybinds::Rebindable::MouseButton(
                                                                                match glfwkey.as_str() {
                                                                                    "Button1" => {
                                                                                        glfw::MouseButton::Button1
                                                                                    }
                                                                                    "Button2" => {
                                                                                        glfw::MouseButton::Button2
                                                                                    }
                                                                                    "Button3" => {
                                                                                        glfw::MouseButton::Button3
                                                                                    }
                                                                                    "Button4" => {
                                                                                        glfw::MouseButton::Button4
                                                                                    }
                                                                                    "Button5" => {
                                                                                        glfw::MouseButton::Button5
                                                                                    }
                                                                                    "Button6" => {
                                                                                        glfw::MouseButton::Button6
                                                                                    }
                                                                                    "Button7" => {
                                                                                        glfw::MouseButton::Button7
                                                                                    }
                                                                                    _ => {
                                                                                        glfw::MouseButton::Button8
                                                                                    }
                                                                                }
                                                                            ),
                                                                            action: binding.clone()
                                                                        });
                                                                    }
                                                                    
                                                                    UNCAPKB.store(true, std::sync::atomic::Ordering::Relaxed);
                                                                } 
                                                            
                                                            }
                                                            
                                                        }
    
                                                        

                                                        
                                                        pos_y += button_height + 10.0; 
                                                    }
                                                } else {
                                                    for (buttonname, command) in gamecurrentbuttons {

                                                        let button_width = if buttonname.starts_with("Slider") { 25.0 * 20.0  } else  { buttonname.len() as f32 * 20.0 };
                                                    

                                                        let pos_x = (available_width - button_width) / 2.0;
    
    
                                                        ui.set_cursor_pos([pos_x, pos_y]);
                                                        ui.set_next_item_width(250.0);
                                                        if buttonname.starts_with("Slider") {
                                                            let truncated_name = buttonname.split_at(6).1;
                                                            if buttonname == "SliderMouse Sensitivity" {
                                                                if ui.slider(truncated_name, 0.1, 3.0, &mut MISCSETTINGS.mouse_sense) {
                                                                    //g.button_command(command);
                                                                }
                                                            }
                                                            if buttonname == "SliderMusic Volume" {
                                                                if ui.slider(truncated_name, 0.0, 1.0, &mut MISCSETTINGS.music_vol) {
                                                                    //g.button_command(command);
                                                                }
                                                            }
                                                            if buttonname == "SliderSounds Volume" {
                                                                if ui.slider(truncated_name, 0.0, 1.0, &mut MISCSETTINGS.sound_vol) {
                                                                    //g.button_command(command);
                                                                }
                                                            }
                                                        } else if buttonname.starts_with("Switch") {

                                                            if buttonname == "SwitchJoystick" {
                                                                if ui.checkbox("Enable Joystick Support (Beta)", &mut MISCSETTINGS.controllersupport) {

                                                                }
                                                            }

                                                        } else {
                                                            if ui.button_with_size(buttonname, [button_width, button_height]) {
                                                                g.button_command(command);
                                                                UNCAPKB.store(true, std::sync::atomic::Ordering::Relaxed);
                                                            }
                                                        }
                                                        
                                                        pos_y += button_height + 10.0; 
                                                    }
                                            
                                                }
                                                    
                                               
                                            }
                                            
                                        
                                        });

                                    // Render the ImGui frame
                                    self.guirenderer.render(&mut self.imgui);
                                } else {
                                    if gcraftopen {
                                        //println!("Gcraft is open");
                                        let cb = g.currentbuttons.clone();

                                        let (width, height) =
                                            self.window.read().get_framebuffer_size();
                                        self.imgui.io_mut().display_size =
                                            [width as f32, height as f32];

                                        // Start the ImGui frame
                                        let ui = self.imgui.frame();

                                        let window_flags = WindowFlags::NO_DECORATION
                                            | WindowFlags::NO_MOVE
                                            | WindowFlags::NO_RESIZE
                                            | WindowFlags::NO_TITLE_BAR;

                                        let window_size = (700.0, 700.0);

                                        let window_pos = [
                                            width as f32 / 2.0 - (window_size.0 / 2.0),
                                            height as f32 / 2.0 - (window_size.1 / 2.0),
                                        ];
                                        let mut recipeindexscrafted = Vec::new();

                                        ui.window("Transparent Window")
                                            .size([window_size.0, window_size.1], Condition::Always)
                                            .position(window_pos, Condition::Always)
                                            .flags(window_flags)
                                            .build(|| {
                                                let button_width = 200.0;
                                                let button_height = 20.0;
                                                let window_size = ui.window_size();

                                                let available_width = window_size[0];
                                                let available_height = window_size[1];

                                                let pos_x = (available_width - button_width) / 2.0;
                                                let mut pos_y = (available_height
                                                    - (cb.len() as f32 * button_height)
                                                    - 10.0 * (cb.len() as f32 - 1.0))
                                                    / 2.0;

                                                // for (buttonname, command) in cb {
                                                //     ui.set_cursor_pos([pos_x, pos_y]);
                                                //     if ui.button_with_size(buttonname, [button_width, button_height]) {
                                                //         g.button_command(command);
                                                //     }
                                                //     pos_y += button_height + 10.0; // Add some spacing between buttons
                                                // }
                                                ui.text_colored(
                                                    [1.0, 1.0, 0.0, 1.0],
                                                    "Hold ctrl to craft all of a recipe",
                                                );
                                                if CROUCHING {
                                                    ui.text_colored(
                                                        [1.0, 1.0, 0.0, 1.0],
                                                        "Ctrl pressed.",
                                                    );
                                                }
                                                for (index, recipeent) in CURRENT_AVAIL_RECIPES
                                                    .lock()
                                                    .iter_mut()
                                                    .enumerate()
                                                {
                                                    let recipe = recipeent.recipe.clone();
                                                    ui.set_cursor_pos([pos_x, pos_y]);
                                                    let str = format!(
                                                        "{}, {}",
                                                        Blocks::get_name(recipe.1 .0),
                                                        recipe.1 .1
                                                    );
                                                    if RECIPES_DISABLED {
                                                        ui.text_colored(
                                                            [0.0, 0.0, 1.0, 1.0],
                                                            str,
                                                        );
                                                    } else {
                                                        if ui.button_with_size(
                                                            str,
                                                            [button_width, button_height],
                                                        ) {
                                                            recipeindexscrafted.push(index);
                                                            //g.craft_recipe_index(index);
                                                            recipeent.disabled = true;
                                                            RECIPES_DISABLED = true;
                                                        }
                                                    }

                                                    let mut costs = String::from("Using ");

                                                    for (index, entry) in
                                                        recipe.0.iter().enumerate()
                                                    {
                                                        costs += entry.1.to_string().as_str();
                                                        costs += " ";
                                                        costs += Blocks::get_name(entry.0);
                                                        if index < (recipe.0.len() - 1) {
                                                            costs += ", ";
                                                        } else {
                                                            costs += ".";
                                                        }
                                                    }

                                                    ui.text_colored(
                                                        [1.0, 0.0, 0.0, 1.0],
                                                        costs,
                                                    );

                                                    pos_y += button_height + 10.0;
                                                    // Add some spacing between buttons
                                                }
                                            });

                                        for recipe in recipeindexscrafted {
                                            g.craft_recipe_index(recipe, CROUCHING);
                                        }
                                        Game::update_avail_recipes(&g.inventory.clone());

                                        if RECIPES_DISABLED {
                                            if RECIPE_COOLDOWN_TIMER < 0.5 {
                                                RECIPE_COOLDOWN_TIMER += self.delta_time;
                                            } else {
                                                RECIPES_DISABLED = false;
                                                RECIPE_COOLDOWN_TIMER = 0.0;
                                            }
                                        }

                                        #[cfg(feature = "glfw")]
                                        g.update_inventory();

                                        // Render the ImGui frame
                                        self.guirenderer.render(&mut self.imgui);
                                    }
                                    else {
                                        let (width, height) =
                                            self.window.read().get_framebuffer_size();
                                        self.imgui.io_mut().display_size =
                                            [width as f32, height as f32];

                                        // Start the ImGui frame
                                        let ui = self.imgui.frame();

                                        let window_flags = WindowFlags::NO_DECORATION
                                            | WindowFlags::NO_MOVE
                                            | WindowFlags::NO_RESIZE
                                            | WindowFlags::NO_SCROLLBAR
                                            | WindowFlags::NO_TITLE_BAR
                                            | WindowFlags::NO_BACKGROUND;

                                        let window_size = (700.0, 700.0);
                                        let window_pos = [0f32, 0f32];

                                        ui.window("Command Line")
                                            .size([window_size.0, window_size.1], Condition::Always)
                                            .position(window_pos, Condition::Always)
                                            .flags(window_flags)
                                            .build(|| {
                                                if self.cmd.cmd_open {
                                                    ui.set_keyboard_focus_here();
                                                    ui.set_cursor_pos([0f32, 0f32]);
                                                    if ui.input_text("##", &mut self.cmd.cmd_text)
                                                    .enter_returns_true(true)
                                                    .build() {
                                                        self.cmd.run(self.game.as_mut().unwrap());
                                                    }
                                                }
                                            });

                                        // Render the ImGui frame
                                        self.guirenderer.render(&mut self.imgui);
                                    }
                                }

                                // avoid borrow checker
                                (*addr_of_mut!(*self)).handle_events(self.imgui.io_mut())
                            }
                        }
                        None => {
                            main_menu = true;
                        }
                    }

                    if main_menu && !SINGLEPLAYER {
                        self.imgui
                            .io_mut()
                            .update_delta_time(Duration::from_secs_f32(self.delta_time));

                        let (width, height) = self.window.read().get_framebuffer_size();
                        self.imgui.io_mut().display_size = [width as f32, height as f32];

                        // Start the ImGui frame
                        let ui = self.imgui.frame();

                        let window_flags = WindowFlags::NO_DECORATION
                            | WindowFlags::NO_MOVE
                            | WindowFlags::NO_RESIZE
                            | WindowFlags::NO_SCROLLBAR
                            | WindowFlags::NO_TITLE_BAR
                            | WindowFlags::NO_BACKGROUND;

                        let window_size = (700.0, 700.0);

                        let window_pos = [
                            width as f32 / 2.0 - (window_size.0 / 2.0),
                            height as f32 / 2.0 - (window_size.1 / 2.0),
                        ];

                        ui.window("Transparent Window")
                            .size([window_size.0, window_size.1], Condition::Always)
                            .position(window_pos, Condition::Always)
                            .flags(window_flags)
                            .build(|| {
                                let button_width = 500.0;
                                let button_height = 20.0;
                                let window_size = ui.window_size();

                                let available_width = window_size[0];
                                let available_height = window_size[1];

                                let pos_x = (available_width - button_width) / 2.0;
                                let pos_y = (available_height - (button_height) - 10.0) / 2.0;

                                ui.set_cursor_pos([pos_x, pos_y]);

                                ui.text("Enter server address:");

                                ui.set_cursor_pos([pos_x, pos_y + 25.0]);
                                ui.set_next_item_width(button_width);

                                ui.input_text("##serveraddress", &mut self.serveraddrbuffer)
                                    .build();

                                ui.set_cursor_pos([pos_x, pos_y + 50.0]);

                                if ui.button_with_size("Connect", [button_width, button_height]) {
                                    SINGLEPLAYER = false;
                                    DECIDEDSPORMP = true;
                                    *LAST_ENTERED_SERVERADDRESS = self.serveraddrbuffer.clone();
                                    THEENTEREDADDRESS = self.serveraddrbuffer.clone();
                                    ADDRESSENTERED.store(true, std::sync::atomic::Ordering::Relaxed);
                                    save_lesa();
                                    *(self.serveraddress.lock()) =
                                        Some(self.serveraddrbuffer.clone());
                                    self.addressentered
                                        .store(true, std::sync::atomic::Ordering::Relaxed);
                                }
                                // pos_y += button_height + 10.0; // Add some spacing between buttons
                            });

                        // Render the ImGui frame
                        self.guirenderer.render(&mut self.imgui);

                        // avoid borrow checker
                        (*addr_of_mut!(*self)).handle_events(self.imgui.io_mut())
                    }
                }
            }
        }

        self.window.write().swap_buffers();
    }

    fn handle_events(&mut self, io: &mut Io) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::MouseButton(mousebutton, action, _) => {
                    if unsafe { !LISTENINGFORREBIND } {
                        let index = match mousebutton {
                            glfw::MouseButton::Button1 => 0,
                            glfw::MouseButton::Button2 => 1,
                            glfw::MouseButton::Button3 => 2,
                            glfw::MouseButton::Button4 => 3,
                            glfw::MouseButton::Button5 => 4,
                            glfw::MouseButton::Button6 => 5,
                            glfw::MouseButton::Button7 => 6,
                            glfw::MouseButton::Button8 => 7,
                        };
                        io.mouse_down[index] = action == glfw::Action::Press;
                    }

                    match self.game.as_mut() {
                        Some(g) => {
                            if unsafe { LISTENINGFORREBIND } {
                                match unsafe{ &*addr_of!(ABOUTTOREBIND) } {
                                    Some(atr) => {
                    
                                        match atr.key {
                                            crate::keybinds::Rebindable::Key(_oldscan) => {
                                                
                                            },
                                            crate::keybinds::Rebindable::MouseButton(mb) => {
                                                unsafe {
                                                    if !MISCSETTINGS.mousebinds.contains_key(&format!("{:?}", mousebutton)) {
                                                        MISCSETTINGS.mousebinds.remove(&format!("{:?}", mb));
                                                        MISCSETTINGS.mousebinds.insert(format!("{:?}", mousebutton), atr.action.clone());
                                                        g.button_command("bindingsmenu".into());
                                                    }
                                                    
                                                    LISTENINGFORREBIND = false;
                                                }
                                            },
                                        }
                                        
                                    }
                                    None => {}
                                }
                            } else {
                                //println!("Io capture mouse: {}", io.want_capture_mouse);
                                if !io.want_capture_mouse && !g.vars.menu_open {
                                    if mousebutton == glfw::MouseButtonLeft {
                                        if !io.want_capture_mouse {
                                            if !g.vars.menu_open && !g.hud.chest_open {
                                                self.window
                                                    .write()
                                                    .set_cursor_mode(
                                                        glfw::CursorMode::Disabled,
                                                    );
                                                g.set_mouse_focused(true);
                                            }
                                        }
                                    }
                                    #[cfg(feature = "glfw")]
                                    g.mouse_button(mousebutton, action);
                                }
                            }
                        }
                        None => {}
                    }
                }
                glfw::WindowEvent::FramebufferSize(wid, hei) => {
                    self.width = wid as u32;
                    self.height = hei as u32;
                    unsafe {
                        gl::Viewport(0, 0, wid, hei);
                        WINDOWHEIGHT = hei;
                        WINDOWWIDTH = wid;
                        if self.game.is_some(){
                            let cam = CAMERA.as_ref().unwrap();
                            let mut c = cam.lock();
                            let cfov = c.fov;
                            c.update_fov(cfov);
                        }
                        let cam = &mut self.menu_camera;
                        let cfov = cam.fov;
                        cam.update_fov(cfov);
                    }
                }
                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    match self.game.as_mut() {
                        Some(g) => {
                            g.cursor_pos(xpos, ypos);
                            if !g.vars.mouse_focused {
                                io.mouse_pos = [xpos as f32, ypos as f32];
                            }
                        }
                        None => {
                            io.mouse_pos = [xpos as f32, ypos as f32];
                        }
                    }
                }
                glfw::WindowEvent::Key(key, _scancode, action, _modifiers) => {
                    let pressed = action == glfw::Action::Press
                    || action == glfw::Action::Repeat;

                    if unsafe { !LISTENINGFORREBIND } {
                        if (key as usize) < 512 {
                            io.keys_down[key as usize] = action != glfw::Action::Release;
                        }
                    }

                    match self.game.as_mut() {
                        Some(g) => {
                            if unsafe { LISTENINGFORREBIND } {
                                let keyscan = key.get_scancode().unwrap_or(0);

                                match unsafe { &*addr_of!(ABOUTTOREBIND) } {
                                    Some(atr) => {
                    
                                        match atr.key {
                                            crate::keybinds::Rebindable::Key(oldscan) => {
                                                unsafe {
                                                    if !MISCSETTINGS.keybinds.contains_key(&keyscan) {
                                                        MISCSETTINGS.keybinds.remove(&oldscan);
                                                        MISCSETTINGS.keybinds.insert(keyscan, atr.action.clone());
                                                        g.button_command("bindingsmenu".into());
                                                    }
                                                    
                                                    LISTENINGFORREBIND = false;
                                                }
                                            },
                                            crate::keybinds::Rebindable::MouseButton(_mb) => {
                    
                                            },
                                        }
                                        
                                    }
                                    None => {}
                                }
                            } else {
                                if g.crafting_open
                                    && pressed
                                    && unsafe { MISCSETTINGS
                                        .keybinds
                                        .get(&key.get_scancode().unwrap())
                                        .unwrap_or(&"Blah".to_string())
                                        == "Craft"
                                    }
                                {
                                    //println!("SHould close craft");
                                    // g.crafting_open = false;
                                    // self.window.write().set_cursor_mode(glfw::CursorMode::Disabled);
                                    // g.set_mouse_focused(true);

                                    g.crafting_open = false;

                                    self.window.write().set_cursor_mode(
                                        glfw::CursorMode::Disabled,
                                    );
                                    g.set_mouse_focused(true);
                                    unsafe { UNCAPKB.store(true, std::sync::atomic::Ordering::Relaxed); }
                                
                                } else {
                                    if g.vars.menu_open
                                        && pressed
                                        && unsafe { MISCSETTINGS
                                            .keybinds
                                            .get(&key.get_scancode().unwrap())
                                            .unwrap_or(&"Blah".to_string())
                                            == "Exit/Menu"
                                        }
                                    {
                                        g.vars.menu_open = false;
                                        self.window.write().set_cursor_mode(
                                            glfw::CursorMode::Disabled,
                                        );
                                        g.set_mouse_focused(true);
                                        unsafe { UNCAPKB.store(true, std::sync::atomic::Ordering::Relaxed); }
                                    }

                                    if (!io.want_capture_keyboard
                                        && !io.want_text_input)
                                        && !g.vars.menu_open
                                    {
                                        #[cfg(feature = "glfw")]
                                        g.keyboard(key, action);

                                        if key == Key::Escape {
                                            if g.vars.menu_open {
                                                self.window
                                                    .write()
                                                    .set_cursor_mode(
                                                        glfw::CursorMode::Normal,
                                                    );
                                                g.set_mouse_focused(false);
                                            } else {
                                                g.vars.menu_open = false;
                                                self.window
                                                    .write()
                                                    .set_cursor_mode(
                                                        glfw::CursorMode::Disabled,
                                                    );
                                                g.set_mouse_focused(true);
                                            }
                                        }
                                    } else {
                                        //println!()
                                    }
                                }
                            }
                        }
                        None => {
                            if action == glfw::Action::Press {
                                match key {
                                    glfw::Key::LeftShift | glfw::Key::RightShift => {
                                        io.key_shift = true
                                    }
                                    glfw::Key::LeftControl | glfw::Key::RightControl => {
                                        io.key_ctrl = true
                                    }
                                    glfw::Key::LeftAlt | glfw::Key::RightAlt => {
                                        io.key_alt = true
                                    }
                                    glfw::Key::LeftSuper | glfw::Key::RightSuper => {
                                        io.key_super = true
                                    }
                                    _ => {}
                                }
                            } else if action == glfw::Action::Release {
                                match key {
                                    glfw::Key::LeftShift | glfw::Key::RightShift => {
                                        io.key_shift = false
                                    }
                                    glfw::Key::LeftControl | glfw::Key::RightControl => {
                                        io.key_ctrl = false
                                    }
                                    glfw::Key::LeftAlt | glfw::Key::RightAlt => {
                                        io.key_alt = false
                                    }
                                    glfw::Key::LeftSuper | glfw::Key::RightSuper => {
                                        io.key_super = false
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    match key {
                        Key::F11 => {
                            if action == Action::Press {
                                let wind = self.window.write();
                                toggle_fullscreen(wind.window_ptr())
                            }
                        }
                        Key::GraveAccent => {
                            if action == Action::Press {
                                self.cmd.cmd_open = !self.cmd.cmd_open;
                                unsafe { UNCAPKB.store(!self.cmd.cmd_open, std::sync::atomic::Ordering::Relaxed); }
                                if self.game.is_some() {
                                    self.game.as_mut().unwrap().set_mouse_focused(!self.cmd.cmd_open);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                // glfw::WindowEvent::CharModifiers(char, _modifiers) => {
                //     println!("{:?}", char);
                // }
                glfw::WindowEvent::Char(char) => {
                    io.add_input_character(char);
                }
                glfw::WindowEvent::Scroll(x, y) => {
                    io.mouse_wheel_h += x as f32;
                    io.mouse_wheel += y as f32;

                    match self.game.as_mut() {
                        Some(g) => {
                            if !g.vars.menu_open {
                                #[cfg(feature = "glfw")]
                                g.scroll(y);
                            }
                        }
                        None => {}
                    }
                }
                _ => {}
            }
        }
    }

    // TODO: create better abstractions around glfw/imgui input
    // fn set_mod(io: &mut imgui::Io, modifier: Modifiers) {
    //     io.key_ctrl = modifier.intersects(Modifiers::Control);
    //     io.key_alt = modifier.intersects(Modifiers::Alt);
    //     io.key_shift = modifier.intersects(Modifiers::Shift);
    //     io.key_super = modifier.intersects(Modifiers::Super);
    // }
}
