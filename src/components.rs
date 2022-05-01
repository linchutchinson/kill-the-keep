use crate::prelude::*;

pub struct Player;

pub struct Enemy {
    pub attack_options: Vec<(Entity, f32)>,
}

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
    pub source: Entity,
}

pub struct PlayTargetedCardMessage {
    pub target: Entity,
}

pub struct EnemyIntent {
    pub enemy: Entity,
}

pub struct DealDamageMessage {
    pub source: Entity,
    pub target: Entity,
    pub amount: i32,
}

pub struct InflictsStatus;

pub struct AddBlockMessage {
    pub target: Entity,
    pub amount: i32,
}

pub struct ApplyVulnerabilityMessage {
    pub target: Entity,
    pub amount: i32,
}

pub struct ApplyWeaknessMessage {
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

pub struct EnemyActionOption;

pub struct DamageRange {
    pub min: i32,
    pub max: i32,
}

pub struct TakeEnemyActionMessage {
    pub enemy: Entity,
    pub action: Entity,
}

pub struct BlockRange {
    pub min: i32,
    pub max: i32,
}

pub struct MultipleAttack {
    pub times: i32,
}

pub struct IncomingEffect;

pub struct OutgoingEffect;

pub struct Weakness;

pub struct InflictWeakness {
    pub amount: i32,
}

pub struct CardChoice {
    pub cards: Vec<CardData>,
}
