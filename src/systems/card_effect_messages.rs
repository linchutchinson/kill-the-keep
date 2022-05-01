use crate::prelude::*;

#[system(for_each)]
#[read_component(Card)]
#[read_component(DealsDamage)]
pub fn send_card_damage(
    ecs: &mut SubWorld,
    message: &PlayCardMessage,
    targeted_message: &PlayTargetedCardMessage,
    commands: &mut CommandBuffer,
) {
    let card_ref = ecs.entry_ref(message.card).unwrap();

    if let Ok(damage) = card_ref.get_component::<DealsDamage>() {
        commands.push((
            (),
            DealDamageMessage {
                source: message.source,
                target: targeted_message.target,
                amount: damage.amount,
            },
        ));
    }
}

#[system(for_each)]
#[read_component(Card)]
#[read_component(InflictVulnerability)]
pub fn send_card_vulnerability(
    ecs: &mut SubWorld,
    message: &PlayCardMessage,
    targeted_message: &PlayTargetedCardMessage,
    commands: &mut CommandBuffer,
) {
    let card_ref = ecs.entry_ref(message.card).unwrap();

    if let Ok(vuln) = card_ref.get_component::<InflictVulnerability>() {
        commands.push((
            (),
            ApplyVulnerabilityMessage {
                target: targeted_message.target,
                amount: vuln.amount,
            },
        ));
    }
}

#[system(for_each)]
#[read_component(Card)]
#[read_component(AddsBlock)]
#[read_component(Player)]
pub fn send_card_block(
    ecs: &mut SubWorld,
    message: &PlayCardMessage,
    commands: &mut CommandBuffer,
) {
    let card_ref = ecs.entry_ref(message.card).unwrap();

    if let Ok(block) = card_ref.get_component::<AddsBlock>() {
        if let Some((player_entity, _)) = <(Entity, &Player)>::query().iter(ecs).nth(0) {
            commands.push((
                (),
                AddBlockMessage {
                    target: *player_entity,
                    amount: block.amount,
                },
            ));
        }
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
        commands.push(((), Message, *card_creation));
    }
}
