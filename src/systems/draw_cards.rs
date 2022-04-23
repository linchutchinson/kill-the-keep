use crate::prelude::*;

#[system]
pub fn draw_cards(#[resource] card_zones: &mut CardZones) {
    (0..5).for_each(|_| {
        if card_zones.deck.len() < 1 {
            card_zones.deck.append(&mut card_zones.discard);
        }

        if let Some(card) = card_zones.deck.pop() {
            card_zones.hand.push(card);
        }
    });
}

#[system]
pub fn discard_hand(#[resource] card_zones: &mut CardZones) {
    card_zones.discard.append(&mut card_zones.hand);
}
