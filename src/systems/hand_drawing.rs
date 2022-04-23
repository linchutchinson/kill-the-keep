use crate::prelude::*;

const CARD_BORDER_COLOR: Color = Color::new(138. / 255., 198. / 255., 208. / 255., 1.0);

#[system]
#[read_component(Card)]
pub fn render_cards_in_hand(ecs: &mut SubWorld, #[resource] card_zones: &CardZones) {
    let card_count = card_zones.hand.iter().count();

    card_zones.hand
        .iter()
        .enumerate()
        .for_each(|(idx, entity)| {
            let pos_x = calculate_card_center_x(idx as i32, card_count as i32);
            if let Ok(card) = ecs.entry_ref(*entity).unwrap().get_component::<Card>() {
                render_card(pos_x, card);
            }
        });
}

#[system]
pub fn render_card_zones(#[resource] card_zones: &CardZones) {
    draw_text(&format!("Cards in Deck: {}", card_zones.deck.len()), 16.0, 32.0, 48.0, TEXT_COLOR);
    draw_text(&format!("Cards in Discard: {}", card_zones.discard.len()), 16.0, 32.0 + 48.0, 48.0, TEXT_COLOR);
}

const CARD_BORDER_SIZE: f32 = 8.0;

fn render_card(center_x: f32, card: &Card) {
    let origin_x = center_x - CARD_WIDTH * 0.5;
    let origin_y = screen_height() - (CARD_HEIGHT / 2.0);
    draw_rectangle(origin_x, origin_y, CARD_WIDTH, CARD_HEIGHT, CARD_BORDER_COLOR);
    draw_rectangle(origin_x + CARD_BORDER_SIZE, origin_y + CARD_BORDER_SIZE, CARD_WIDTH - (CARD_BORDER_SIZE * 2.0), CARD_HEIGHT - CARD_BORDER_SIZE * 2.0, TEXT_COLOR);
    draw_text(&card.name, origin_x, origin_y, 32.0, TEXT_COLOR);
}
