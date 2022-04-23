use crate::prelude::*;

mod drawing;
mod hand_drawing;
mod card_selection;
mod card_playing;
mod end_turn;
mod draw_cards;
mod enemy_intents;
mod energy;

pub fn build_start_of_turn_schedule() -> Schedule {
    Schedule::builder()
            .add_system(position_characters_system())
            .add_system(draw_cards::discard_hand_system())
            .add_system(enemy_intents::create_enemy_intents_system())
            .add_system(energy::refill_energy_system())
            .flush()
            .add_thread_local(drawing::draw_bg_system())
            .add_thread_local(drawing::draw_characters_system())
            .add_thread_local(drawing::draw_healthbars_system())
            .add_thread_local(hand_drawing::render_cards_in_hand_system())
            .add_thread_local(enemy_intents::draw_enemy_intents_system())
            .add_thread_local(drawing::draw_targeting_cursor_system())
            .add_thread_local(hand_drawing::render_card_zones_system())
            .add_thread_local(drawing::draw_energy_system())
            .flush()
            .add_system(draw_cards::draw_cards_system())
            .flush()
            .add_system(end_turn::end_turn_system())
            .build()
}

pub fn build_player_turn_schedule() -> Schedule {
    Schedule::builder()
            .add_system(position_characters_system())
            .flush()
            .add_thread_local(drawing::draw_bg_system())
            .add_thread_local(drawing::draw_characters_system())
            .add_thread_local(drawing::draw_healthbars_system())
            .add_thread_local(hand_drawing::render_cards_in_hand_system())
            .add_thread_local(enemy_intents::draw_enemy_intents_system())
            .add_thread_local(drawing::draw_targeting_cursor_system())
            .add_thread_local(hand_drawing::render_card_zones_system())
            .add_thread_local(drawing::draw_energy_system())
            .flush()
            .add_system(card_playing::select_card_targets_system())
            .add_system(card_selection::select_cards_system())
            .flush()
            .add_system(card_playing::play_card_system())
            .flush()
            .add_system(end_turn::end_turn_system())
            .build()
}

pub fn build_enemy_turn_schedule() -> Schedule {
    Schedule::builder()
            .add_system(position_characters_system())
            .flush()
            .add_thread_local(drawing::draw_bg_system())
            .add_thread_local(drawing::draw_characters_system())
            .add_thread_local(drawing::draw_healthbars_system())
            .add_thread_local(hand_drawing::render_cards_in_hand_system())
            .add_thread_local(enemy_intents::draw_enemy_intents_system())
            .add_thread_local(drawing::draw_targeting_cursor_system())
            .add_thread_local(hand_drawing::render_card_zones_system())
            .add_thread_local(drawing::draw_energy_system())
            .flush()
            .add_system(enemy_intents::resolve_enemy_intents_system())
            .flush()
            .add_system(end_turn::end_turn_system())
            .build()
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


