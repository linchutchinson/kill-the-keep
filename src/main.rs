mod prelude {
    pub use macroquad::prelude::*;
    pub use legion::*;

    pub const BG_COLOR: Color = Color::new(58.0 / 255.0, 38.0 / 255.0, 24.0 / 255., 1.0);
}

use crate::prelude::*;

struct State {
    world: World,
    resources: Resources,
    schedule: Schedule,
}

impl State {
    fn new() -> Self {
        let world = World::default();
        let resources = Resources::default();

        let schedule = Schedule::builder()
            .add_thread_local(draw_test_system())
            .build();
        
        Self {
            world,
            resources,
            schedule,
        }
    }
}

#[system]
fn draw_test() {
    clear_background(BG_COLOR);

    draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
    draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
    draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);
}

#[macroquad::main("Kill the Keep")]
async fn main() {
    let mut state = State::new();

    loop {
        state.schedule.execute(&mut state.world, &mut state.resources);

        next_frame().await
    }
}
