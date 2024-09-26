use crate::game::{Game, WEATHERTYPE};
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
}

pub struct Cmd {
    pub cmd_open: bool,
    pub cmd_text: String,
}

impl Cmd {
    pub fn new() -> Self {
        Cmd {
            cmd_open: false,
            cmd_text: String::with_capacity(128),
        }
    }

    pub fn run(&mut self, game: &mut Game) {
        self.cmd_open = false;


        let mut lexer = Token::lexer(self.cmd_text.as_str());

        match lexer.next() {
            Some(Ok(Token::TimeSet)) => {
                let mut tod = game.timeofday.lock();
                match lexer.next() {
                    Some(Ok(Token::Number(num))) => {
                        *tod = num as f32;
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
            _ => {}
        }
        
        self.cmd_text.clear();
        game.set_mouse_focused(true);
    }
}