use bevy::prelude::*;

pub fn ivec2length(vec: &IVec2) -> f32 {
    ((vec.x as f32).powi(2) + (vec.y as f32).powi(2)).sqrt()
}