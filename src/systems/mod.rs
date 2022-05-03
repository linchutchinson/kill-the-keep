use crate::prelude::*;
use legion::systems::Builder;

mod add_card_to_zone;
mod block;
mod card_drafting;
mod card_effect_messages;
mod card_playing;
mod card_selection;
mod damage;
mod death;
mod draw_cards;
mod drawing;
mod end_turn;
mod enemy_intents;
mod energy;
mod hand_drawing;
mod status_effects;

pub fn build_initialization_schedule() -> Schedule {
    Schedule::builder()
        .add_system(spawn_starter_deck_system())
        .add_system(spawn_initial_combatants_system())
        .add_system(spawn_random_enemies_system())
        .add_system(begin_battle_system())
        .build()
}

pub fn build_start_of_battle_schedule() -> Schedule {
    Schedule::builder()
        .add_system(spawn_random_enemies_system())
        .add_system(begin_battle_system())
        .build()
}

pub fn build_start_of_turn_schedule() -> Schedule {
    let mut builder = Schedule::builder();

    builder
        .add_system(enemy_intents::clear_enemy_intents_system())
        .flush()
        .add_system(position_heroes_system())
        .add_system(position_enemies_system())
        .add_system(status_effects::reduce_remaining_duration_of_effects_system())
        .add_system(draw_cards::discard_hand_system())
        .add_system(enemy_intents::create_enemy_intents_system())
        .flush()
        .add_system(enemy_intents::create_deal_damage_intents_system())
        .add_system(enemy_intents::create_status_intents_system())
        .add_system(enemy_intents::create_block_intents_system())
        .add_system(energy::refill_energy_system())
        .add_system(block::clear_player_block_system())
        .flush()
        .add_system(enemy_intents::clear_enemy_take_action_messages_system())
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
        .add_system(position_heroes_system())
        .add_system(position_enemies_system())
        .flush();

    add_render_systems_to_builder(&mut builder);

    builder
        .add_system(card_playing::select_card_targets_system())
        .add_system(card_selection::select_cards_system())
        .flush()
        .add_system(card_effect_messages::send_card_damage_system())
        .add_system(card_effect_messages::send_card_aoe_damage_system())
        .add_system(card_effect_messages::send_card_block_system())
        .add_system(card_effect_messages::send_card_status_system())
        .add_system(card_effect_messages::send_card_creation_system())
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
        .add_system(position_heroes_system())
        .add_system(position_enemies_system())
        .add_system(block::clear_enemy_block_system())
        .flush();

    add_render_systems_to_builder(&mut builder);

    builder
        .add_system(enemy_intents::resolve_enemy_damage_intents_system())
        .add_system(enemy_intents::resolve_enemy_status_intents_system())
        .add_system(enemy_intents::resolve_enemy_block_intents_system())
        .flush();

    add_combat_resolution_systems_to_builder(&mut builder);

    builder
        .add_system(end_turn::end_turn_system())
        .add_system(death::check_for_deaths_system())
        .build()
}

pub fn build_end_of_battle_schedule() -> Schedule {
    Schedule::builder().add_system(end_battle_system()).build()
}

//Card Drafting
pub fn build_choose_rewards_schedule() -> Schedule {
    Schedule::builder()
        .add_thread_local(drawing::draw_bg_system())
        .add_thread_local(card_drafting::draw_card_choices_system())
        .flush()
        .add_system(card_drafting::select_card_to_draft_system())
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
        .add_system(add_card_to_zone::handle_add_card_messages_system())
        .add_system(status_effects::apply_status_system())
}

#[system(for_each)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Vec2)]
fn position_heroes(entity: &Entity, pos: &mut Vec2, _: &Player, ecs: &mut SubWorld) {
    *pos = get_player_pos();
}

#[system]
#[read_component(Enemy)]
#[write_component(Vec2)]
fn position_enemies(ecs: &mut SubWorld) {
    let mut enemy_query = <(&Enemy, &mut Vec2)>::query();

    enemy_query
        .iter_mut(ecs)
        .enumerate()
        .for_each(|(idx, (enemy, mut pos))| {
            *pos = get_enemy_pos(idx as i32);
        });
}

#[system]
#[read_component(Player)]
#[read_component(Enemy)]
fn end_battle(
    ecs: &mut SubWorld,
    #[resource] turn_state: &mut TurnState,
    #[resource] game_state: &mut GameState,
    #[resource] zones: &mut CardZones,
    #[resource] db: &mut CardDB,
    commands: &mut CommandBuffer,
    #[resource] combatant_tex: &CombatantTextures,
) {
    let player_victorious = match turn_state {
        TurnState::BattleOver { player_victorious } => player_victorious,

        _ => {
            panic!();
        }
    };

    let mut enemy_query = <(Entity, &Enemy)>::query();
    enemy_query.iter(ecs).for_each(|(entity, _)| {
        commands.remove(*entity);
    });

    if !*player_victorious {
        let mut player_query = <(Entity, &Player)>::query();
        player_query.iter(ecs).for_each(|(entity, _)| {
            commands.remove(*entity);
        });

        zones.clear_all();
        *game_state = GameState::Initialization;
    } else {
        let rewards = db
            .draw_random(Rarity::Common, 3)
            .expect("Failed to draw random cards from database.");

        commands.push(((), CardChoice { cards: rewards }));
        *game_state = GameState::ChooseRewards;
    }
}

#[system]
fn spawn_initial_combatants(
    commands: &mut CommandBuffer,
    #[resource] combatant_textures: &CombatantTextures,
) {
    spawn_hero(commands, combatant_textures);
}

#[system]
fn spawn_random_enemies(
    commands: &mut CommandBuffer,
    #[resource] combatant_textures: &CombatantTextures,
) {
    let mut enemies_to_spawn = thread_rng().gen_range(1..=3);

    (0..enemies_to_spawn).for_each(|idx| {
        let enemy_type: i32 = thread_rng().gen_range(0..3);
        match enemy_type {
            0 => {
                spawn_orc(commands, combatant_textures);
            }
            1 => spawn_crow(commands, combatant_textures),
            _ => spawn_spider(commands, combatant_textures),
        }
    });
}

#[system]
fn begin_battle(#[resource] game_state: &mut GameState, #[resource] turn_state: &mut TurnState) {
    *game_state = GameState::InBattle;
    *turn_state = TurnState::StartOfTurn { round_number: 1 }
}

#[system]
fn spawn_starter_deck(
    #[resource] zones: &mut CardZones,
    #[resource] db: &mut CardDB,
    commands: &mut CommandBuffer,
) {
    (0..5).for_each(|_| {
        add_card_to_deck(commands, zones, db, 1);
    });

    (0..4).for_each(|_| add_card_to_deck(commands, zones, db, 2));

    add_card_to_deck(commands, zones, db, 3);
}
