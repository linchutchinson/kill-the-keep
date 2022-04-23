use crate::prelude::*;

const HEALTH_BAR_WIDTH: f32 = 250.0;
const HEALTH_BAR_VERT_OFFSET: f32 = 16.0;
const HEALTH_BAR_HEIGHT: f32 = 16.0;

const HEALTH_BAR_EMPTY_COLOR: Color = Color::new(23. / 255., 22. / 255., 20. / 255., 1.0);
const HEALTH_BAR_FILLED_COLOR: Color = Color::new(232. / 255., 72. / 255., 85. / 255., 1.0);

#[system]
pub fn draw_bg() {
    clear_background(BG_COLOR);
}

#[system(for_each)]
pub fn draw_characters(pos: &Vec2, sprite: &Sprite) {
    draw_texture(sprite.texture, pos.x - sprite.texture.width() * 0.5, pos.y - sprite.texture.height(), Color::new(1.0, 1.0, 1.0, 1.0));
}

#[system(for_each)]
pub fn draw_healthbars(pos: &Vec2, health: &Health) {
    let top_left = *pos + Vec2::new(-HEALTH_BAR_WIDTH / 2.0, HEALTH_BAR_VERT_OFFSET);
    let health_pct = health.current as f32 / health.max as f32;

    draw_rectangle(top_left.x, top_left.y, HEALTH_BAR_WIDTH, HEALTH_BAR_HEIGHT, HEALTH_BAR_EMPTY_COLOR);
    draw_rectangle(top_left.x, top_left.y, HEALTH_BAR_WIDTH * health_pct, HEALTH_BAR_HEIGHT, HEALTH_BAR_FILLED_COLOR);
    
    let hp_text = &format!("{}/{}", health.current, health.max);
    let text_size = measure_text(hp_text, Some(Font::default()), 32, 1.0);
    draw_text(hp_text, pos.x - (text_size.width * 0.5), pos.y + HEALTH_BAR_VERT_OFFSET + HEALTH_BAR_HEIGHT, 32.0, TEXT_COLOR)
}

#[system(for_each)]
pub fn draw_targeting_cursor(_: &Selected, #[resource] ui_tex: &UITextures) {
    let mouse_pos = mouse_position();

    draw_texture(ui_tex.crosshair, mouse_pos.0, mouse_pos.1, Color::new(1.0, 1.0, 1.0, 1.0));
}

#[system]
pub fn draw_energy(#[resource] energy: &Energy) {
    draw_text(&format!("Energy: {}/{}", energy.current, energy.max), screen_width() / 2.0, 32.0, 48.0, TEXT_COLOR);
}
