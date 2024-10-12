use std::collections::VecDeque;
use crate::game::{Game, CAMERA, PLAYERPOS, SPAWNPOINT, WAYPOINTS, WEATHERTYPE};
use crate::blockinfo::BLOCK_NAME_TO_ID;
use crate::statics::MISCSETTINGS;
use crate::vec::IVec3;
use bevy::reflect::Map;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
enum Token {
    // Tokens can be literal strings, of any length.
    #[token("timeset")]
    TimeSet,
    
    // Match a sequence of digits (i.e., u32)
    #[regex(r"\d+", |lex| lex.slice().parse::<u32>().unwrap_or(0))] // 0 if it doesnt unwrap for now, might need to change this
    Number(u32),  

    #[token("day")]
    Day,

    #[token("night")]
    Night,

    #[token("weather")]
    Weather,

    #[token("snow")]
    Snow,

    #[token("rain")]
    Rain,

    #[token("clear")]
    Clear,

    #[token("kill")]
    Kill,

    #[token("spawn")]
    Spawn,

    #[token("give")]
    Give,

    #[regex(r"[a-zA-Z_]*")]
    Word,

    #[token("waypoint")]
    Waypoint,

    #[token("new")]
    New
}

pub struct Cmd {
    pub cmd_open: bool,
    pub cmd_history: VecDeque<String>,
    pub cmd_peeking: usize,
    pub cmd_text: String,
}

impl Cmd {
    pub fn new() -> Self {
        Cmd {
            cmd_open: false,
            cmd_history: VecDeque::new(),
            cmd_peeking: 0,
            cmd_text: String::with_capacity(128),
        }
    }

    pub fn run(&mut self, game: &mut Game) {
        let mut lexer = Token::lexer(self.cmd_text.as_str());

        match lexer.next() {
            Some(Ok(Token::TimeSet)) => {
                let mut tod = game.timeofday.lock();
                match lexer.next() {
                    Some(Ok(Token::Number(num))) => {
                        *tod = num as f32;
                    }
                    Some(Ok(Token::Day)) => {
                        *tod = 450.0;
                    }
                    Some(Ok(Token::Night)) => {
                        *tod = 0.0;
                    }
                    _ => {}
                }
            }
            Some(Ok(Token::Weather)) => {
                match lexer.next() {
                    Some(Ok(Token::Snow)) => {
                        unsafe { WEATHERTYPE = 1.0 };
                    }
                    Some(Ok(Token::Rain)) => {
                        unsafe { WEATHERTYPE = 2.0 };
                    }
                    Some(Ok(Token::Clear)) => {
                        unsafe { WEATHERTYPE = 0.0 };
                    }
                    _ => {}
                }
            }
            Some(Ok(Token::Day)) => {
                let mut tod = game.timeofday.lock();
                *tod = 450.0;
            }
            Some(Ok(Token::Night)) => {
                let mut tod = game.timeofday.lock();
                *tod = 0.0;
            }
            Some(Ok(Token::Kill)) => {
                game.take_damage(game.health.load(std::sync::atomic::Ordering::Relaxed));
            }
            Some(Ok(Token::Spawn)) => {
                let cam = unsafe { CAMERA.as_ref().unwrap() };
                let mut camlock = cam.lock();
                unsafe { camlock.position = SPAWNPOINT; }
                camlock.velocity = bevy::prelude::Vec3::ZERO;
                drop(camlock);
            }
            Some(Ok(Token::Give)) => {
                match lexer.next() {
                    Some(Ok(Token::Number(id))) => {
                        match lexer.next() {
                            Some(Ok(Token::Number(amt))) => {
                                game.drops.add_drop(unsafe { PLAYERPOS.snapshot().pos.into() }, id, amt)
                            }
                            _ => {
                                game.drops.add_drop(unsafe { PLAYERPOS.snapshot().pos.into() }, id, 1)
                            }
                        }
                    }
                    Some(Ok(Token::Word)) => {
                        if BLOCK_NAME_TO_ID.contains_key(lexer.slice()) {
                            let block_id = BLOCK_NAME_TO_ID[lexer.slice()];
                            match lexer.next() {
                                Some(Ok(Token::Number(amt))) => {
                                    game.drops.add_drop(unsafe { PLAYERPOS.snapshot().pos.into() }, block_id, amt)
                                }
                                _ => {
                                    game.drops.add_drop(unsafe { PLAYERPOS.snapshot().pos.into() }, block_id, 1)
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            Some(Ok(Token::Waypoint)) => {
                
                match lexer.next() {
                    Some(Ok(Token::Word)) => {
                        let waypointname = lexer.slice();
                        if unsafe { WAYPOINTS.contains_key(waypointname) } {
                            let cam = unsafe { CAMERA.as_ref().unwrap() };
                            let mut camlock = cam.lock();
                            unsafe { camlock.position = WAYPOINTS[waypointname].as_vec3(); }
                            camlock.velocity = bevy::prelude::Vec3::ZERO;
                            drop(camlock);
                        }
                    }
                    Some(Ok(Token::New)) => {
                        match lexer.next() {
                            Some(Ok(Token::Word)) => {
                                let waypointname = lexer.slice();
                                let snapshot = unsafe { PLAYERPOS.snapshot() };
                                unsafe { WAYPOINTS.insert(waypointname.to_string(), unsafe { crate::vec::IVec3::new( snapshot.pos.0.round() as i32, snapshot.pos.1.round() as i32, snapshot.pos.2.round() as i32) }) };
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        
        self.cmd_open = false;
        self.cmd_history.push_front(self.cmd_text.clone());
        self.cmd_text.clear();
        self.cmd_peeking = 0;
        game.set_mouse_focused(true);
    }

    pub fn peek_up(&mut self) {
        if self.cmd_peeking < self.cmd_history.len() {
            self.cmd_peeking += 1;
            self.cmd_text.clear();
            self.cmd_text.push_str(self.cmd_history[self.cmd_peeking - 1].as_str());
        }
    }

    pub fn peek_down(&mut self) {
        if self.cmd_peeking > 1 {
            self.cmd_peeking -= 1;
            self.cmd_text.clear();
            self.cmd_text.push_str(self.cmd_history[self.cmd_peeking - 1].as_str());
        }
        else {
            self.cmd_peeking = 0;
            self.cmd_text.clear();
        }
    }
}