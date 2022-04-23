use crate::prelude::*;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum TurnState {
    StartOfTurn,
    PlayerTurn,
    EnemyTurn,
}