mod components;
mod spawner;
mod systems;
mod hand_helpers;
mod card_zones;
mod ui_textures;
mod enemy_position_helpers;
mod turn_state;
mod energy;

mod prelude {
    pub use macroquad::prelude::*;
    pub use macroquad::input::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub use ::rand::prelude::*;

    pub const BG_COLOR: Color = Color::new(58.0 / 255.0, 38.0 / 255.0, 24.0 / 255., 1.0);
    pub const TEXT_COLOR: Color = Color::new(229. / 255., 225. / 255., 220. / 255., 1.0);

    pub const WINDOW_WIDTH : i32 = 1280;

    pub const CARD_HEIGHT: f32 = 256.0;
    pub const CARD_WIDTH: f32 = CARD_HEIGHT * 2.0 / 3.0;

    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::hand_helpers::*;
    pub use crate::card_zones::*;
    pub use crate::ui_textures::*;
    pub use crate::enemy_position_helpers::*;
    pub use crate::turn_state::*;
    pub use crate::energy::*;
}

use crate::prelude::*;

struct State {
    world: World,
    resources: Resources,
    start_of_turn_schedule: Schedule,
    player_turn_schedule: Schedule,
    enemy_turn_schedule: Schedule,
}

impl State {
    fn new() -> Self {
        let world = World::default();
        let mut resources = Resources::default();

        resources.insert(CardZones::new());
        resources.insert(UITextures::new());
        resources.insert(TurnState::StartOfTurn);
        resources.insert(Energy::new());
        
        Self {
            world,
            resources,
            start_of_turn_schedule: build_start_of_turn_schedule(),
            player_turn_schedule: build_player_turn_schedule(),
            enemy_turn_schedule: build_enemy_turn_schedule(),
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Kill the Keep".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: 720,
        ..Default::default()   
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");

    let mut state = State::new();
    spawn_hero(&mut state.world);
    spawn_orc(&mut state.world);

    (0..10).for_each(|_| {
        spawn_card_in_deck(&mut state.world, &mut state.resources)
    });

    loop {
        let current_state = state.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::StartOfTurn => {
                state.start_of_turn_schedule.execute(&mut state.world, &mut state.resources);
            },
            TurnState::PlayerTurn => {
                state.player_turn_schedule.execute(&mut state.world, &mut state.resources);
            },
            TurnState::EnemyTurn => {
                state.enemy_turn_schedule.execute(&mut state.world, &mut state.resources);
            }
        }


        next_frame().await
    }
}
