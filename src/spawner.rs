use crate::prelude::*;

pub fn spawn_hero(commands: &mut CommandBuffer, combatant_tex: &CombatantTextures) {
    commands.push(
        (
            Vec2::ZERO,
            Player,
            Health {
                current: 80,
                max: 80,
                block: 0,
            },
            Sprite {
                texture: combatant_tex.hero,
            }
        )
    );
}

pub fn spawn_orc(commands: &mut CommandBuffer, combatant_tex: &CombatantTextures) {
    let simple_attack = commands.push(((), EnemyActionOption, DamageRange{ min: 3, max: 7 }));
    let debuff_attack = commands.push(((), EnemyActionOption, DamageRange{ min: 8, max: 14 }, InflictVulnerability{ amount: 2 }));
    let block = commands.push(((), EnemyActionOption, BlockRange{ min: 4, max: 8 }));

    commands.push(
        (
            Vec2::ZERO,
            Enemy{ attack_options: vec![(simple_attack, 0.4), (block, 0.4), (debuff_attack, 0.2),] },
            Health {
                current: 6,
                max: 6,
                block: 0,
            },
            Sprite {
                texture: combatant_tex.orc,
            }
        )
    );
}

pub fn spawn_spider(commands: &mut CommandBuffer, combatant_tex: &CombatantTextures) {
    let simple_attack = commands.push(((), EnemyActionOption, DamageRange{ min: 3, max: 7 }));
    let debuff_attack = commands.push(((), EnemyActionOption, DamageRange{ min: 5, max: 10 }, InflictWeakness{ amount: 2 }));
    let block = commands.push(((), EnemyActionOption, BlockRange{ min: 4, max: 8 }));

    commands.push(
        (
            Vec2::ZERO,
            Enemy{ attack_options: vec![(simple_attack, 0.4), (block, 0.4), (debuff_attack, 0.2),] },
            Health {
                current: 6,
                max: 6,
                block: 0,
            },
            Sprite {
                texture: combatant_tex.spider,
            }
        )
    );
}

pub fn spawn_crow(commands: &mut CommandBuffer, combatant_tex: &CombatantTextures) {
    let multiple_attack = commands.push(((), EnemyActionOption, DamageRange{ min: 1, max: 2 }, MultipleAttack{ times: 5 }));
    let block = commands.push(((), EnemyActionOption, BlockRange{ min: 4, max: 8 }));

    let hp = thread_rng().gen_range(10..15);

    commands.push(
        (
            Vec2::ZERO,
            Enemy{ attack_options: vec![(multiple_attack, 0.5), (block, 0.5)] },
            Health {
                current: hp,
                max: hp,
                block: 0,
            },
            Sprite {
                texture: combatant_tex.crow,
            }
        )
    );
}

pub fn spawn_strike(ecs : &mut World, resources: &mut Resources) {
    let entity = ecs.push(
        (
            Card {
                name: "Strike".to_owned()
            },
            EnergyCost {
                amount: 1,  
            },
            SelectTarget,
            DealsDamage{ amount: 6 },
        )
    );

    resources.get_mut::<CardZones>().unwrap().deck.push(entity);
}

pub fn spawn_defend(ecs : &mut World, resources: &mut Resources) {
    let entity = ecs.push(
        (
            Card {
                name: "Defend".to_owned()
            },
            EnergyCost {
                amount: 1,  
            },
            AddsBlock{ amount: 5 }, 
        )
    );

    resources.get_mut::<CardZones>().unwrap().deck.push(entity);
}

pub fn spawn_bash(ecs : &mut World, resources: &mut Resources) {
    let entity = ecs.push(
        (
            Card {
                name: "Bash".to_owned()
            },
            EnergyCost {
                amount: 2,  
            },
            SelectTarget,
            DealsDamage{ amount: 8 },
            InflictVulnerability{ amount: 2 },
        )
    );

    resources.get_mut::<CardZones>().unwrap().deck.push(entity);
}
