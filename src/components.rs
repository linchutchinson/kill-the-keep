use crate::prelude::*;

pub struct Player;

pub struct Enemy;

pub struct Health {
    pub current: i32,
    pub max: i32,
    pub block: i32,
}

pub struct Sprite {
    pub texture: Texture2D,
}

pub struct Card {
    pub name: String,
}

pub struct SelectTarget;

pub struct DealsDamage {
    pub amount: i32,
}

pub struct AddsBlock {
    pub amount: i32,
}

pub struct InflictVulnerability {
    pub amount: i32,
}

pub struct EnergyCost {
    pub amount: i32,
}

pub struct Selected;

pub struct PlayCardMessage {
    pub card: Entity,
}

pub struct PlayTargetedCardMessage {
    pub card: Entity,
    pub target: Entity,
}

pub struct EnemyAttackIntent {
    pub enemy: Entity,
    pub damage: i32,
}

pub struct DealDamageMessage {
    pub target: Entity,
    pub amount: i32,
}

pub struct AddBlockMessage {
    pub target: Entity,
    pub amount: i32,
}

pub struct ApplyVulnerabilityMessage {
    pub target: Entity,
    pub amount: i32,
}

pub struct StatusEffect {
    pub target: Entity,
}

pub struct Duration {
    pub rounds: i32,
}

pub struct DamageMultiplier {
    pub multiplier: f32,
}

pub struct Vulnerability;
