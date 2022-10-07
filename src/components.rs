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
    pub card_type: CardType,
}

pub struct SelectTarget;

pub struct DealsDamage {
    pub amount: i32,
}

pub struct DealBlock;

pub struct AddsBlock {
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

pub struct Targeted {
    pub target: Entity,
}

pub struct AllEnemies;

pub struct EnemyIntent {
    pub enemy: Entity,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Status {
    Vulnerability,
    Weakness,
}

pub struct InflictsStatus {
    pub status: Status,
    pub amount: i32,
}

pub struct StatusEffect {
    pub target: Entity,
    pub status_type: Status,
}

pub struct Duration {
    pub rounds: i32,
}

pub struct DamageMultiplier {
    pub multiplier: f32,
}

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

pub struct InflictWeakness {
    pub amount: i32,
}

pub struct CardChoice {
    pub cards: Vec<CardData>,
}

#[derive(Copy, Clone, Debug)]
pub enum CardZone {
    Discard,
}

pub struct Message {
    pub source: Entity,
}

#[derive(Copy, Clone)]
pub struct AddCardToZone {
    pub zone: CardZone,
    pub id: i32,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CardType {
    Attack,
    Skill,
    Power,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Requirement {
    AllCardsInHandAre(CardType),
}

pub struct PlayConditions {
    pub requirements: Vec<Requirement>,
}
