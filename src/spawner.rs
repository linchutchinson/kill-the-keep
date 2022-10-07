use crate::prelude::*;

pub fn spawn_hero(commands: &mut CommandBuffer, combatant_tex: &CombatantTextures) {
    commands.push((
        Vec2::ZERO,
        Player,
        Health {
            current: 80,
            max: 80,
            block: 0,
        },
        Sprite {
            texture: combatant_tex.hero,
        },
    ));
}

pub fn spawn_orc(commands: &mut CommandBuffer, combatant_tex: &CombatantTextures) {
    let simple_attack = commands.push(((), EnemyActionOption, DamageRange { min: 6, max: 8 }));
    let debuff_attack = commands.push((
        (),
        EnemyActionOption,
        DamageRange { min: 10, max: 14 },
        InflictsStatus {
            status: Status::Vulnerability,
            amount: 2,
        },
    ));
    let block = commands.push(((), EnemyActionOption, BlockRange { min: 5, max: 8 }));

    let hp = thread_rng().gen_range(14..18);

    commands.push((
        Vec2::ZERO,
        Enemy {
            attack_options: vec![(simple_attack, 0.4), (block, 0.4), (debuff_attack, 0.2)],
        },
        Health {
            current: hp,
            max: hp,
            block: 0,
        },
        Sprite {
            texture: combatant_tex.orc,
        },
    ));
}

pub fn spawn_spider(commands: &mut CommandBuffer, combatant_tex: &CombatantTextures) {
    let simple_attack = commands.push(((), EnemyActionOption, DamageRange { min: 3, max: 5 }));
    let debuff_attack = commands.push((
        (),
        EnemyActionOption,
        DamageRange { min: 5, max: 10 },
        InflictsStatus {
            status: Status::Weakness,
            amount: 2,
        },
    ));
    let block = commands.push(((), EnemyActionOption, BlockRange { min: 3, max: 6 }));

    let hp = thread_rng().gen_range(14..18);

    commands.push((
        Vec2::ZERO,
        Enemy {
            attack_options: vec![(simple_attack, 0.3), (block, 0.4), (debuff_attack, 0.3)],
        },
        Health {
            current: hp,
            max: hp,
            block: 0,
        },
        Sprite {
            texture: combatant_tex.spider,
        },
    ));
}

pub fn spawn_crow(commands: &mut CommandBuffer, combatant_tex: &CombatantTextures) {
    let multiple_attack = commands.push((
        (),
        EnemyActionOption,
        DamageRange { min: 1, max: 2 },
        MultipleAttack { times: 5 },
    ));
    let block = commands.push(((), EnemyActionOption, BlockRange { min: 6, max: 9 }));

    let hp = thread_rng().gen_range(10..15);

    commands.push((
        Vec2::ZERO,
        Enemy {
            attack_options: vec![(multiple_attack, 0.5), (block, 0.5)],
        },
        Health {
            current: hp,
            max: hp,
            block: 0,
        },
        Sprite {
            texture: combatant_tex.crow,
        },
    ));
}

pub fn add_card_to_deck(
    commands: &mut CommandBuffer,
    zones: &mut CardZones,
    db: &mut CardDB,
    card_id: i32,
) {
    let card_data = db.get_card_from_id(card_id);

    let entity = card_data
        .spawn_as_entity(commands)
        .expect("Failed to parse card data.");

    zones.deck.push(entity);
}
