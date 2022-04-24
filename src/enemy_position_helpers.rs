use crate::prelude::*;

pub fn get_player_pos() -> Vec2 {
    Vec2::new(screen_width() / 3.0, screen_height() / 2.0)
}

pub fn get_enemy_pos(idx: i32) -> Vec2 {
    Vec2::new(screen_width() * 2.0 / 3.0 + (128.0 * idx as f32), screen_height() / 2.0)
}
