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
    commands.push(
        (
            Vec2::ZERO,
            Enemy,
            Health {
                current: 30,
                max: 30,
                block: 0,
            },
            Sprite {
                texture: combatant_tex.orc,
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
