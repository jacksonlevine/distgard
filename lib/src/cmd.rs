use crate::game::Game;

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
        match self.cmd_text.as_str() {
            "day" => {
                let mut tod = game.timeofday.lock();
                *tod = 450f32;
            }
            "night" => {
                let mut tod = game.timeofday.lock();
                *tod = 0f32;
            }
            _ => {}
        }
        self.cmd_text.clear();
        game.set_mouse_focused(true);
    }
}