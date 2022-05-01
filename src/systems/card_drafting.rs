use crate::prelude::*;

#[system(for_each)]
pub fn draw_card_choices(choice: &CardChoice) {
    choice
        .cards
        .iter()
        .enumerate()
        .for_each(|(idx, card_data)| {
            draw_text(
                &card_data.name,
                WINDOW_WIDTH as f32 / 2.0,
                64.0 + (32.0 * idx as f32),
                24.0,
                TEXT_COLOR,
            );
        })
}

#[system]
#[read_component(CardChoice)]
pub fn select_card_to_draft(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] zones: &mut CardZones,
    #[resource] gstate: &mut GameState,
    #[resource] tstate: &mut TurnState,
) {
    let mut choice_query = <(Entity, &CardChoice)>::query();
    let choices = choice_query.iter(ecs).nth(0).unwrap().1;

    let selected_card = match get_last_key_pressed() {
        Some(KeyCode::Key1) => Some(0),
        Some(KeyCode::Key2) => Some(1),
        Some(KeyCode::Key3) => Some(2),
        _ => None,
    };

    if let Some(card_idx) = selected_card {
        let card_entity = choices.cards[card_idx].spawn_as_entity(commands).unwrap();
        zones.discard.push(card_entity);

        choice_query.iter(ecs).for_each(|(entity, _)| {
            commands.remove(*entity);
        });

        *gstate = GameState::InBattle;
        *tstate = TurnState::StartOfBattle;
    }
}
