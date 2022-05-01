use crate::prelude::*;

#[system(for_each)]
#[read_component(EnemyActionOption)]
pub fn create_enemy_intents(entity: &Entity, enemy: &Enemy, commands: &mut CommandBuffer) {
    let rand_val: f32 = thread_rng().gen();
    let mut current_pct = 0.0;
    let mut selected_attack: Option<Entity> = None;

    enemy
        .attack_options
        .iter()
        .for_each(|(attack_entity, pct)| {
            if let Some(_attack_already_selected) = selected_attack {
            } else {
                if rand_val < pct + current_pct {
                    // Select this attack
                    selected_attack = Some(*attack_entity);
                } else {
                    current_pct += pct;
                }
            }
        });

    if let Some(attack) = selected_attack {
        commands.push((
            (),
            TakeEnemyActionMessage {
                enemy: *entity,
                action: attack,
            },
        ));
    }
}

#[system(for_each)]
#[read_component(DamageRange)]
#[read_component(MultipleAttack)]
pub fn create_deal_damage_intents(
    ecs: &mut SubWorld,
    entity: &Entity,
    message: &TakeEnemyActionMessage,
    commands: &mut CommandBuffer,
) {
    let selected_action = ecs.entry_ref(message.action).unwrap();

    if let Ok(damage_range) = selected_action.get_component::<DamageRange>() {
        let mut times = 1;

        if let Ok(multiple_attack) = selected_action.get_component::<MultipleAttack>() {
            times = multiple_attack.times;
        }

        (0..times).for_each(|_| {
            let damage = thread_rng().gen_range(damage_range.min..damage_range.max);

            commands.push((
                (),
                EnemyIntent {
                    enemy: message.enemy,
                },
                DealsDamage { amount: damage },
            ));
        });
    }
}

#[system(for_each)]
#[read_component(InflictVulnerability)]
pub fn create_inflict_vulnerability_intents(
    ecs: &mut SubWorld,
    entity: &Entity,
    message: &TakeEnemyActionMessage,
    commands: &mut CommandBuffer,
) {
    let selected_action = ecs.entry_ref(message.action).unwrap();

    if let Ok(inflict_vulnerability) = selected_action.get_component::<InflictVulnerability>() {
        commands.push((
            (),
            EnemyIntent {
                enemy: message.enemy,
            },
            InflictsStatus,
            InflictVulnerability {
                amount: inflict_vulnerability.amount,
            },
        ));
    }
}

#[system(for_each)]
#[read_component(InflictWeakness)]
pub fn create_inflict_weakness_intents(
    ecs: &mut SubWorld,
    entity: &Entity,
    message: &TakeEnemyActionMessage,
    commands: &mut CommandBuffer,
) {
    let selected_action = ecs.entry_ref(message.action).unwrap();

    if let Ok(inflict_vulnerability) = selected_action.get_component::<InflictWeakness>() {
        commands.push((
            (),
            EnemyIntent {
                enemy: message.enemy,
            },
            InflictsStatus,
            InflictWeakness {
                amount: inflict_vulnerability.amount,
            },
        ));
    }
}

#[system(for_each)]
#[read_component(BlockRange)]
pub fn create_block_intents(
    ecs: &mut SubWorld,
    entity: &Entity,
    message: &TakeEnemyActionMessage,
    commands: &mut CommandBuffer,
) {
    let selected_action = ecs.entry_ref(message.action).unwrap();

    if let Ok(block_range) = selected_action.get_component::<BlockRange>() {
        let block = thread_rng().gen_range(block_range.min..block_range.max);

        commands.push((
            (),
            EnemyIntent {
                enemy: message.enemy,
            },
            AddsBlock { amount: block },
        ));
    }
}

#[system(for_each)]
pub fn clear_enemy_take_action_messages(
    entity: &Entity,
    _: &TakeEnemyActionMessage,
    commands: &mut CommandBuffer,
) {
    commands.remove(*entity);
}

#[system(for_each)]
#[read_component(EnemyIntent)]
#[read_component(DealsDamage)]
#[read_component(InflictsStatus)]
#[read_component(AddsBlock)]
pub fn draw_enemy_intents(
    ecs: &mut SubWorld,
    entity: &Entity,
    enemy: &Enemy,
    pos: &Vec2,
    sprite: &Sprite,
    #[resource] ui_tex: &UITextures,
) {
    let mut intent_query = <(Entity, &EnemyIntent)>::query();
    let render_pos = *pos - (Vec2::Y * sprite.texture.height());

    let mut attack_number = 0;
    let mut attack_damage = 0;

    let mut will_block = false;
    let mut will_inflict_status = false;

    intent_query
        .iter(ecs)
        .filter(|(_, intent)| intent.enemy == *entity)
        .for_each(|(intent_entity, _)| {
            let intent_ref = ecs.entry_ref(*intent_entity).unwrap();
            if let Ok(adds_block) = intent_ref.get_component::<AddsBlock>() {
                will_block = true;
            }

            if let Ok(inflicts_status) = intent_ref.get_component::<InflictsStatus>() {
                will_inflict_status = true;
            }

            if let Ok(deals_damage) = intent_ref.get_component::<DealsDamage>() {
                attack_number += 1;
                attack_damage = deals_damage.amount;
            }
        });

    if will_block {
        draw_rectangle(
            render_pos.x - 4.,
            render_pos.y - 4.,
            40.,
            40.,
            Color::new(0.4, 0.4, 0.4, 1.0),
        );
    } else {
        if will_inflict_status {
            draw_rectangle(
                render_pos.x - 4.,
                render_pos.y - 4.,
                40.,
                40.,
                Color::new(0., 0.4, 0.0, 1.0),
            );
        }

        if attack_number > 0 {
            draw_texture(
                ui_tex.attack_intent,
                render_pos.x,
                render_pos.y,
                Color::new(1.0, 0.0, 0.0, 1.0),
            );

            let mut attack_string = format!("{}", attack_damage);

            if attack_number > 1 {
                attack_string = format!("{} x{}", attack_damage, attack_number);
            }

            draw_text(&attack_string, render_pos.x, render_pos.y, 32.0, TEXT_COLOR);
        }
    }
}

#[system(for_each)]
#[read_component(Player)]
pub fn resolve_enemy_intents(
    ecs: &mut SubWorld,
    entity: &Entity,
    intent: &EnemyIntent,
    damage: &DealsDamage,
    commands: &mut CommandBuffer,
) {
    if let Some((player_entity, _)) = <(Entity, &Player)>::query().iter(ecs).nth(0) {
        commands.push((
            (),
            DealDamageMessage {
                source: intent.enemy,
                target: *player_entity,
                amount: damage.amount,
            },
        ));
    }
}

#[system(for_each)]
#[read_component(Player)]
pub fn resolve_enemy_invulnerability_intents(
    ecs: &mut SubWorld,
    entity: &Entity,
    _: &EnemyIntent,
    vuln: &InflictVulnerability,
    commands: &mut CommandBuffer,
) {
    if let Some((player_entity, _)) = <(Entity, &Player)>::query().iter(ecs).nth(0) {
        commands.push((
            (),
            ApplyVulnerabilityMessage {
                target: *player_entity,
                amount: vuln.amount,
            },
        ));
    }
}

#[system(for_each)]
#[read_component(Player)]
pub fn resolve_enemy_weakness_intents(
    ecs: &mut SubWorld,
    entity: &Entity,
    _: &EnemyIntent,
    weak: &InflictWeakness,
    commands: &mut CommandBuffer,
) {
    if let Some((player_entity, _)) = <(Entity, &Player)>::query().iter(ecs).nth(0) {
        commands.push((
            (),
            ApplyWeaknessMessage {
                target: *player_entity,
                amount: weak.amount,
            },
        ));
    }
}

#[system(for_each)]
#[read_component(Player)]
pub fn resolve_enemy_block_intents(
    ecs: &mut SubWorld,
    entity: &Entity,
    intent: &EnemyIntent,
    block: &AddsBlock,
    commands: &mut CommandBuffer,
) {
    if let Some((player_entity, _)) = <(Entity, &Player)>::query().iter(ecs).nth(0) {
        commands.push((
            (),
            AddBlockMessage {
                target: intent.enemy,
                amount: block.amount,
            },
        ));
    }
}

#[system(for_each)]
pub fn clear_enemy_intents(entity: &Entity, _: &EnemyIntent, commands: &mut CommandBuffer) {
    commands.remove(*entity);
}
