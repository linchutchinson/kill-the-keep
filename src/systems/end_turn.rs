use crate::prelude::*;

#[system]
pub fn end_turn(#[resource] turn_state: &mut TurnState) {
    match turn_state {
        TurnState::StartOfTurn{round_number} => {
            *turn_state = TurnState::PlayerTurn{ round_number: *round_number };
        },
        TurnState::PlayerTurn{round_number} => {
            if is_key_pressed(KeyCode::F) {
                *turn_state = TurnState::EnemyTurn{ round_number: *round_number };
            }
        },
        TurnState::EnemyTurn{round_number} => {
            *turn_state = TurnState::StartOfTurn{ round_number: *round_number + 1 };
        },
        
        _ => {
            
        }
    }
}
