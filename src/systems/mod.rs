use legion::systems::Builder;
use crate::prelude::*;

mod drawing;
mod hand_drawing;
mod card_selection;
mod card_playing;
mod end_turn;
mod draw_cards;
mod enemy_intents;
mod energy;
mod card_effect_messages;
mod damage;
mod block;
mod status_effects;
mod death;

pub fn build_initialization_schedule() -> Schedule {
    Schedule::builder()
        .add_system(spawn_initial_combatants_system())
        .add_system(begin_battle_system())
        .build()
}

pub fn build_start_of_turn_schedule() -> Schedule {
    let mut builder = Schedule::builder();

    builder
        .add_system(position_characters_system())
        .add_system(status_effects::reduce_remaining_duration_of_effects_system())
        .add_system(draw_cards::discard_hand_system())
        .add_system(enemy_intents::create_enemy_intents_system())
        .add_system(energy::refill_energy_system())
        .add_system(block::clear_block_system())
        .flush();
    
    add_render_systems_to_builder(&mut builder);

    builder
        .add_system(draw_cards::draw_cards_system())
        .flush()
        .add_system(end_turn::end_turn_system())
        .add_system(death::check_for_deaths_system())
        .build()
}

pub fn build_player_turn_schedule() -> Schedule {
    let mut builder = Schedule::builder();

    builder
            .add_system(position_characters_system())
            .flush();
    
    add_render_systems_to_builder(&mut builder);

    builder
        .add_system(card_playing::select_card_targets_system())
        .add_system(card_selection::select_cards_system())
        .flush()
        .add_system(card_effect_messages::send_card_damage_system())
        .add_system(card_effect_messages::send_card_block_system())
        .add_system(card_effect_messages::send_card_vulnerability_system())
        .add_system(card_playing::play_card_system())
        .flush();
    
    add_combat_resolution_systems_to_builder(&mut builder);

    builder
        .add_system(end_turn::end_turn_system())
        .add_system(death::check_for_deaths_system())
        .build()
}

pub fn build_enemy_turn_schedule() -> Schedule {
    let mut builder = Schedule::builder();

    builder
        .add_system(position_characters_system())
        .flush();

    add_render_systems_to_builder(&mut builder);
            
    builder
        .add_system(enemy_intents::resolve_enemy_intents_system())
        .flush();

    add_combat_resolution_systems_to_builder(&mut builder);

    builder
        .add_system(end_turn::end_turn_system())
        .add_system(death::check_for_deaths_system())
        .build()
}

pub fn build_end_of_battle_schedule() -> Schedule {
    Schedule::builder()
        .add_system(restart_battle_system())
        .build()
}

fn add_render_systems_to_builder(builder: &mut Builder) -> &mut Builder {
    builder
        .add_thread_local(drawing::draw_bg_system())
        .add_thread_local(drawing::draw_characters_system())
        .add_thread_local(drawing::draw_healthbars_system())
        .add_thread_local(hand_drawing::render_cards_in_hand_system())
        .add_thread_local(enemy_intents::draw_enemy_intents_system())
        .add_thread_local(drawing::draw_targeting_cursor_system())
        .add_thread_local(hand_drawing::render_card_zones_system())
        .add_thread_local(drawing::draw_energy_system())
        .add_thread_local(drawing::draw_status_effects_system())
        .flush()
}

fn add_combat_resolution_systems_to_builder(builder: &mut Builder) -> &mut Builder {
    builder
        .add_system(damage::apply_damage_multipliers_system())
        .add_system(block::apply_block_system())
        .flush()
        .add_system(damage::deal_damage_system())
        .flush()
        .add_system(status_effects::apply_vulnerability_system())
}

#[system(for_each)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Vec2)]
fn position_characters(entity: &Entity, pos: &mut Vec2, ecs: &mut SubWorld) {
    if ecs.entry_ref(*entity).unwrap().get_component::<Player>().is_ok() {
        *pos = get_player_pos();
    } else {
        *pos = get_enemy_pos();
    }
}

#[system]
#[read_component(Player)]
#[read_component(Enemy)]
fn restart_battle(ecs: &mut SubWorld, #[resource] turn_state: &mut TurnState, commands: &mut CommandBuffer, #[resource] combatant_tex: &CombatantTextures) {
    let player_victorious = match turn_state {
        TurnState::BattleOver{ player_victorious } => {
            player_victorious
        },

        _ => {
            panic!();
        }
    };

    let mut enemy_query = <(Entity, &Enemy)>::query();
    enemy_query
        .iter(ecs)
        .for_each(|(entity, _)| {
            commands.remove(*entity);
        });
    
    spawn_orc(commands, combatant_tex);
    
    if !*player_victorious {
        let mut player_query = <(Entity, &Player)>::query();
        player_query
            .iter(ecs)
            .for_each(|(entity, _)| {
                commands.remove(*entity);
            });
        
        spawn_hero(commands, combatant_tex);
    }

    *turn_state = TurnState::StartOfTurn{ round_number: 1 }; 
}

#[system]
fn spawn_initial_combatants(commands: &mut CommandBuffer, #[resource] combatant_textures: &CombatantTextures) {
    spawn_hero(commands, combatant_textures);
    spawn_orc(commands, combatant_textures);
}

#[system]
fn begin_battle(#[resource] game_state: &mut GameState, #[resource] turn_state: &mut TurnState) {
    *game_state = GameState::InBattle;
    *turn_state = TurnState::StartOfTurn{ round_number: 1 }
}