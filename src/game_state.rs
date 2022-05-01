use crate::prelude::*;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum GameState {
    Initialization,
    InBattle,
    ChooseRewards,
}
