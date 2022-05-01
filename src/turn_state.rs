use crate::prelude::*;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum TurnState {
    StartOfBattle,
    StartOfTurn { round_number: i32 },
    PlayerTurn { round_number: i32 },
    EnemyTurn { round_number: i32 },
    BattleOver { player_victorious: bool },
}
