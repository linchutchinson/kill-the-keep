mod card_data;
mod card_zones;
mod combatant_textures;
mod components;
mod enemy_position_helpers;
mod energy;
mod game_state;
mod hand_helpers;
mod spawner;
mod systems;
mod turn_state;
mod ui_textures;

mod prelude {
    pub use ::rand::prelude::*;
    pub use ::rand::seq::SliceRandom;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub use macroquad::input::*;
    pub use macroquad::prelude::*;

    pub const BG_COLOR: Color = Color::new(58.0 / 255.0, 38.0 / 255.0, 24.0 / 255., 1.0);
    pub const TEXT_COLOR: Color = Color::new(229. / 255., 225. / 255., 220. / 255., 1.0);

    pub const WINDOW_WIDTH: i32 = 1280;

    pub const CARD_HEIGHT: f32 = 256.0;
    pub const CARD_WIDTH: f32 = CARD_HEIGHT * 2.0 / 3.0;

    pub use crate::card_data::*;
    pub use crate::card_zones::*;
    pub use crate::combatant_textures::*;
    pub use crate::components::*;
    pub use crate::enemy_position_helpers::*;
    pub use crate::energy::*;
    pub use crate::game_state::*;
    pub use crate::hand_helpers::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
    pub use crate::ui_textures::*;
}

use crate::prelude::*;

struct State {
    world: World,
    resources: Resources,
    initialization_schedule: Schedule,
    start_of_battle_schedule: Schedule,
    start_of_turn_schedule: Schedule,
    player_turn_schedule: Schedule,
    enemy_turn_schedule: Schedule,
    end_of_battle_schedule: Schedule,
}

impl State {
    fn new() -> Self {
        let world = World::default();
        let mut resources = Resources::default();

        resources.insert(CardZones::new());
        resources.insert(UITextures::new());
        resources.insert(CombatantTextures::new());
        resources.insert(GameState::Initialization);
        resources.insert(TurnState::StartOfBattle);
        resources.insert(Energy::new());
        resources.insert(CardDB::new());

        Self {
            world,
            resources,
            initialization_schedule: build_initialization_schedule(),
            start_of_battle_schedule: build_start_of_battle_schedule(),
            start_of_turn_schedule: build_start_of_turn_schedule(),
            player_turn_schedule: build_player_turn_schedule(),
            enemy_turn_schedule: build_enemy_turn_schedule(),
            end_of_battle_schedule: build_end_of_battle_schedule(),
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

    loop {
        let current_state = state.resources.get::<GameState>().unwrap().clone();

        match current_state {
            GameState::Initialization => {
                state
                    .initialization_schedule
                    .execute(&mut state.world, &mut state.resources);
            }

            GameState::InBattle => {
                let turn_state = state.resources.get::<TurnState>().unwrap().clone();

                match turn_state {
                    TurnState::StartOfBattle => {
                        state
                            .start_of_battle_schedule
                            .execute(&mut state.world, &mut state.resources);
                    }

                    TurnState::StartOfTurn { round_number: _ } => {
                        state
                            .start_of_turn_schedule
                            .execute(&mut state.world, &mut state.resources);
                    }
                    TurnState::PlayerTurn { round_number: _ } => {
                        state
                            .player_turn_schedule
                            .execute(&mut state.world, &mut state.resources);
                    }
                    TurnState::EnemyTurn { round_number: _ } => {
                        state
                            .enemy_turn_schedule
                            .execute(&mut state.world, &mut state.resources);
                    }
                    TurnState::BattleOver {
                        player_victorious: _,
                    } => {
                        state
                            .end_of_battle_schedule
                            .execute(&mut state.world, &mut state.resources);
                    }
                }
            }
        }

        next_frame().await
    }
}
