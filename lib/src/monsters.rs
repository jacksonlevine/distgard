





pub struct Monsters {

}



impl Monsters {



  pub fn get_aggro_sound(model_index: usize) -> &'static str {
    match model_index {
      0 => {
        path!("assets/sfx/monster1.mp3")
      }
      1 => {
        path!("assets/sfx/monster1.mp3")
      }
      2 => {
        path!("assets/sfx/monster1.mp3")
      }
      3 => {
        path!("assets/sfx/monster2.mp3")
      }
      _ => {
        path!("assets/sfx/monster2.mp3")
      }
    }
  }
}