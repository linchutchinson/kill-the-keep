use crate::prelude::*;

#[system(for_each)]
#[read_component(Card)]
#[read_component(DealsDamage)]
#[read_component(DealBlock)]
#[read_component(Health)]
pub fn send_card_damage(
    ecs: &mut SubWorld,
    message: &PlayCardMessage,
    targeted: &Targeted,
    commands: &mut CommandBuffer,
) {
    let card_ref = ecs.entry_ref(message.card).unwrap();
    let mut damage = None;

    if let Ok(damage_cmp) = card_ref.get_component::<DealsDamage>() {
        damage = Some(damage_cmp.amount);
    }

    if let Ok(_deal_block) = card_ref.get_component::<DealBlock>() {
        let player_ref = ecs.entry_ref(message.source).unwrap();

        if let Ok(health) = player_ref.get_component::<Health>() {
            damage = Some(health.block);
        }
    }

    if damage.is_some() {
        commands.push((
            (),
            Message {
                source: message.source,
            },
            Targeted {
                target: targeted.target,
            },
            DealsDamage {
                amount: damage.unwrap(),
            },
        ));
    }
}

#[system(for_each)]
#[read_component(Card)]
#[read_component(DealsDamage)]
#[read_component(AllEnemies)]
#[read_component(Enemy)]
pub fn send_card_aoe_damage(
    ecs: &mut SubWorld,
    message: &PlayCardMessage,
    commands: &mut CommandBuffer,
) {
    let mut enemy_query = <(Entity, &Enemy)>::query();
    let card_ref = ecs.entry_ref(message.card).unwrap();

    if let Ok(damage) = card_ref.get_component::<DealsDamage>() {
        if let Ok(_) = card_ref.get_component::<AllEnemies>() {
            enemy_query.iter(ecs).for_each(|(enemy_entity, _)| {
                commands.push((
                    (),
                    Message {
                        source: message.source,
                    },
                    Targeted {
                        target: *enemy_entity,
                    },
                    DealsDamage {
                        amount: damage.amount,
                    },
                ));
            })
        }
    }
}

#[system(for_each)]
#[read_component(Card)]
#[read_component(InflictsStatus)]
pub fn send_card_status(
    ecs: &mut SubWorld,
    message: &PlayCardMessage,
    targeted: &Targeted,
    commands: &mut CommandBuffer,
) {
    let card_ref = ecs.entry_ref(message.card).unwrap();

    if let Ok(status_to_inflict) = card_ref.get_component::<InflictsStatus>() {
        commands.push((
            (),
            Message {
                source: message.source,
            },
            Targeted {
                target: targeted.target,
            },
            InflictsStatus {
                status: status_to_inflict.status,
                amount: status_to_inflict.amount,
            },
        ));
    }
}

#[system(for_each)]
#[read_component(AddsBlock)]
pub fn send_card_block(
    ecs: &mut SubWorld,
    message: &PlayCardMessage,
    commands: &mut CommandBuffer,
) {
    let card_ref = ecs.entry_ref(message.card).unwrap();

    if let Ok(block) = card_ref.get_component::<AddsBlock>() {
        commands.push((
            (),
            Message {
                source: message.source,
            },
            AddsBlock {
                amount: block.amount,
            },
        ));
    }
}

#[system(for_each)]
#[read_component(Card)]
#[read_component(AddCardToZone)]
#[read_component(Player)]
pub fn send_card_creation(
    ecs: &mut SubWorld,
    message: &PlayCardMessage,
    commands: &mut CommandBuffer,
) {
    let card_ref = ecs.entry_ref(message.card).unwrap();

    if let Ok(card_creation) = card_ref.get_component::<AddCardToZone>() {
        commands.push((
            (),
            Message {
                source: message.source,
            },
            *card_creation,
        ));
    }
}
