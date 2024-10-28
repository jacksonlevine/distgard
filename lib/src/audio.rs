use std::{collections::HashMap, fs::File, io::{BufReader, Cursor, Read}, thread};
use bevy::prelude::*;
use lockfree::queue::Queue;
use once_cell::sync::Lazy;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, SpatialSink};
use tracing::info;


#[cfg(feature = "audio")]
use crate::game::AUDIOPLAYER;
use crate::statics::MISCSETTINGS;



pub const POSFACTOR: f32 = 0.5; //Used for increasing the spread range of spatial sounds by scaling down the positions of everything.

static mut FUNC_QUEUE: Lazy<Queue<FuncQueue>> = Lazy::new(|| Queue::new());

enum FuncQueue {
    PlayInHead(String),
    PlayInHeadSong(String),
    Play(String, Vec3, Vec3, f32)
}

#[derive(Debug)]
pub struct AudioError {

}


pub struct SoundSeries {
    pub sounds: Vec<String>,
    pub index: usize
}

impl SoundSeries {
    pub fn new(ids: Vec<String>) -> Self {
        Self {
            sounds: ids,
            index: 0
        }
    }

    pub fn increment(&mut self) {
        self.index = (self.index + 1) % self.sounds.len();
    }
}

pub struct SoundSink {
    sink: SpatialSink,
    // worldpos: Vec3
}

impl SoundSink {
    pub fn new(stream: &OutputStreamHandle, worldpos: Vec3, camerapos: Vec3, cameraright: Vec3) -> Self {
        Self {
            sink: SpatialSink::try_new(stream, 
                worldpos.into(), 
                (camerapos - cameraright).into(), 
                (camerapos + cameraright).into()).unwrap(),
            // worldpos
        }
    }
}

#[cfg(feature = "audio")]
pub fn spawn_audio_thread() {
    thread::spawn(|| {
        unsafe {
            loop {
                match FUNC_QUEUE.pop() {
                    Some(f) => {
                        match f {
                            FuncQueue::PlayInHead(f) => {
                                AUDIOPLAYER._play_in_head(f);
                            },
                            FuncQueue::PlayInHeadSong(f) => {
                                AUDIOPLAYER._play_in_head_song(f);
                            },
                            FuncQueue::Play(id, pos, vel, vol) => {
                                AUDIOPLAYER._play(id, &pos, &vel, vol)
                            },
                        }
                        
                    }
                    None => {

                    }
                }
            }
        }
    });
    
    
}

pub struct AudioPlayer {
    pub output: OutputStreamHandle,
    pub _stream: OutputStream,
    pub sounds: HashMap<String, Vec<u8>>,
    pub sinks: HashMap<String, SoundSink>,
    pub headsinks: HashMap<String, Sink>,
    pub serieslist: HashMap<String, SoundSeries>
}

impl AudioPlayer {
    pub fn new() -> Result<Self, AudioError> {

        let(stream, handle ) = OutputStream::try_default().unwrap();

        Ok(AudioPlayer {
            output: handle,
            _stream: stream,
            sounds: HashMap::new(),
            sinks: HashMap::new(),
            headsinks: HashMap::new(),
            serieslist: HashMap::new()
        })

    }

    pub fn update(&mut self) {

    }

    pub fn preload(&mut self, id: &'static str, file_path: &'static str) -> Result<(), AudioError> {
        self._preload(id.to_string(), file_path.to_string())
    }

    pub fn _preload(&mut self, _id: String, file_path: String) -> Result<(), AudioError> {
        match File::open(&file_path) {
            Ok(mut file) => {
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer).unwrap();
                self.sounds.insert(file_path.clone(), buffer);
                self.sinks.insert(file_path.clone(), SoundSink::new(&self.output, Vec3::ZERO, Vec3::ZERO, Vec3::ZERO));
                self.headsinks.insert(file_path.to_string(), Sink::try_new(&self.output).unwrap());
            }
            Err(e) => {
                info!("Error: {e} with file path \"{file_path}\"");
                println!("Error: {e} with file path \"{file_path}\"");
            }
        }
     
        Ok(())
    }

    pub fn preload_series(&mut self, _series_name: &'static str, _paths: Vec<&'static str>) {
        let mut paths = Vec::new();
        for path in _paths {
            paths.push(path.to_string());
            let _ = self._preload(path.to_string(), path.to_string());
        }
        let ss = SoundSeries::new(paths);
        self.serieslist.insert(_series_name.to_string(), ss);
    }

    pub fn play_next_in_series(
        &mut self,
        _series_name: &'static str,
        _pos: &Vec3,
        _vel: &Vec3,
        _vol: f32,
    ) -> Result<(), AudioError> {

        let soundname = match self.serieslist.get_mut(_series_name) {
            Some(series) => {

                let ret = series.sounds[series.index].clone();
                series.increment();
                ret
            }   
            None => {
                info!("Sound series tried to play that we don't know, {}", _series_name);
                String::new()
            }
        };

        self.play_stringname(soundname, _pos, _vel, _vol);

        Ok(())
    }


    pub fn play_in_head(&mut self, id: &'static str) {
        unsafe { FUNC_QUEUE.push(FuncQueue::PlayInHead(id.to_string())) };
    }

    pub fn play_in_head_song(&mut self, id: &'static str) {
        unsafe { FUNC_QUEUE.push(FuncQueue::PlayInHeadSong(id.to_string())) };
    }

    pub fn stop_head_sound(&mut self, id: String) {
        match self.headsinks.get(&id.to_string()) {
            Some(sink) => {
                sink.stop();
            }
            None => {

            }
        }
    }

    pub fn _play_in_head(&mut self, id: String) {
        let mut needtopreload = false;
        match self.sounds.get(&id.to_string()) {
            Some(sound) => {


                match self.headsinks.get(&id.to_string()) {
                    Some(sink) => {

        
                        let cursor = Cursor::new(sound.clone());
                        let reader = BufReader::new(cursor);
                        let source = Decoder::new(reader).unwrap();

                        sink.stop();
        
                        sink.append(source);
                        sink.set_volume(0.5);
                    },
                    None => {
                        println!("There was a sound but no sink. This shouldn't happen");
                    },
                }



            },
            None => {
                needtopreload = true;
            },
        }

        if needtopreload {
            match self._preload(id.clone(), id.clone()) {
                Ok(_) => {
                    self._play_in_head(id.clone());
                }
                Err(_e) => {
                    println!("Couldn't play or preload {}", id);
                }
            }
            
        }
    }




    pub fn _play_in_head_song(&mut self, id: String) {
        let mut needtopreload = false;
        match self.sounds.get(&id.to_string()) {
            Some(sound) => {


                match self.headsinks.get(&id.to_string()) {
                    Some(sink) => {

        
                        let cursor = Cursor::new(sound.clone());
                        let reader = BufReader::new(cursor);
                        let source = Decoder::new(reader).unwrap();

                        sink.stop();
        
                        sink.append(source);
                        sink.set_volume(unsafe { MISCSETTINGS.music_vol });
                    },
                    None => {
                        println!("There was a sound but no sink. This shouldn't happen");
                    },
                }



            },
            None => {
                needtopreload = true;
            },
        }

        if needtopreload {
            match self._preload(id.clone(), id.clone()) {
                Ok(_) => {
                    self._play_in_head(id.clone());
                }
                Err(_e) => {
                    println!("Couldn't play or preload {}", id);
                }
            }
            
        }
    }

    pub fn stop_sound(&mut self, id: &'static str) {
        match self.sinks.get(&id.to_string()) {
            Some(sink) => {
                sink.sink.stop();
            }
            None => {

            }
        }
    }

    pub fn play_stringname(&mut self, id: String, pos: &Vec3, vel: &Vec3, vol: f32) {
        unsafe { FUNC_QUEUE.push(FuncQueue::Play(id, *pos, *vel, vol)) };
    }

    pub fn play(&mut self, id: &'static str, pos: &Vec3, vel: &Vec3, vol: f32) {
        unsafe { FUNC_QUEUE.push(FuncQueue::Play(id.to_string(), *pos , *vel, vol)) };
    }

    pub fn _play(&mut self, id: String, pos: &Vec3, vel: &Vec3, vol: f32) {

        let vol = vol * 5.0;

        let pos = *pos * POSFACTOR;

        let vol = vol * unsafe { MISCSETTINGS.sound_vol };
        let mut needtopreload = false;
        match self.sounds.get(&id.to_string()) {
            Some(sound) => {


                match self.sinks.get(&id.to_string()) {
                    Some(sink) => {

                        let sink = &sink.sink;
        
                        let cursor = Cursor::new(sound.clone());
                        let reader = BufReader::new(cursor);
                        let source = Decoder::new(reader).unwrap();
                        if !sink.is_paused() {
                            sink.clear();
                        }
        
                        sink.append(source);
                        sink.play();
                        sink.set_emitter_position((pos).into());

                        
                        


                        sink.set_volume(vol);
                    },
                    None => {
                        println!("There was a sound but no sink. This shouldn't happen");
                    },
                }



            },
            None => {
                needtopreload = true;
            },
        }

        if needtopreload {
            match self._preload(id.clone(), id.clone()) {
                Ok(_) => {
                    self._play(id, &pos, vel, vol);
                }
                Err(_e) => {
                    println!("Couldn't play or preload {}", id);
                }
            }
        }
        
    }

    pub fn cleanup_channels(&mut self) {

    }

    pub fn set_listener_attributes(
        &mut self,
        position: Vec3,
        right: Vec3
    ) {
        let position = position * POSFACTOR;
        for entry in &self.sinks {
            let sink = entry.1;
            sink.sink.set_left_ear_position((position - right).into());
            sink.sink.set_right_ear_position((position + right).into());
        }
    }
}