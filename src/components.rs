use crate::prelude::*;

pub struct Player;

pub struct Enemy;

pub struct Health {
    pub current: i32,
    pub max: i32,
}

pub struct Sprite {
    pub texture: Texture2D,
}

pub struct Card {
    pub name: String,
}

pub struct Selected;

pub struct PlayCardMessage {
    pub card: Entity,
    pub target: Entity,
}

pub struct EnemyAttackIntent {
    pub enemy: Entity,
    pub damage: i32,
}
