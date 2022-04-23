use crate::prelude::*;

#[system]
pub fn end_turn(#[resource] turn_state: &mut TurnState) {
    match turn_state { 
        TurnState::StartOfTurn => {
            *turn_state = TurnState::PlayerTurn;
        },
        TurnState::PlayerTurn => {
            if is_key_pressed(KeyCode::F) {
                *turn_state = TurnState::EnemyTurn;
            }
        },
        TurnState::EnemyTurn => {
            *turn_state = TurnState::StartOfTurn;
        }
    }
}
