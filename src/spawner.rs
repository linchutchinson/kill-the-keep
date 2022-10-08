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

pub fn spawn_red_louse(commands: &mut CommandBuffer, combatant_tex: &CombatantTextures) {
    let (min_hp, max_hp) = (10, 15);
    let actual_hp = thread_rng().gen_range(min_hp..max_hp);
    spawn_louse(actual_hp, commands, combatant_tex);
}

pub fn spawn_green_louse(commands: &mut CommandBuffer, combatant_tex: &CombatantTextures) {
    let (min_hp, max_hp) = (11, 17);
    let actual_hp = thread_rng().gen_range(min_hp..max_hp);
    spawn_louse(actual_hp, commands, combatant_tex);
}

fn spawn_louse(hp: i32, commands: &mut CommandBuffer, combatant_tex: &CombatantTextures) {
    let simple_attack = commands.push((
        (),
        EnemyActionOption,
        DealsDamage(thread_rng().gen_range(5..7)),
    ));

    commands.push((
        Vec2::ZERO,
        Enemy {
            attack_options: vec![(simple_attack, 0.75)],
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
