use crate::prelude::*;

#[system]
pub fn draw_cards(#[resource] card_zones: &mut CardZones, #[resource] turn_state: &TurnState) {
    match turn_state {
        TurnState::StartOfTurn{round_number} => {
            if *round_number == 1 {
                card_zones.deck.shuffle(&mut thread_rng());
            }
        }
        _ => {}
    }

    (0..5).for_each(|_| {
        if card_zones.deck.len() < 1 {
            card_zones.deck.append(&mut card_zones.discard);
            card_zones.deck.shuffle(&mut thread_rng());
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
