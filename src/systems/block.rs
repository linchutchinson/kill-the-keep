use crate::prelude::*;

#[system(for_each)]
#[write_component(Health)]
pub fn apply_block(
    ecs: &mut SubWorld,
    entity: &Entity,
    message: &Message,
    block: &AddsBlock,
    commands: &mut CommandBuffer,
) {
    let mut target_ref = ecs.entry_mut(message.source).unwrap();

    if let Ok(mut health) = target_ref.get_component_mut::<Health>() {
        health.block += block.amount;
    }

    commands.remove(*entity);
}

#[system(for_each)]
pub fn clear_player_block(health: &mut Health, _: &Player) {
    health.block = 0;
}

#[system(for_each)]
pub fn clear_enemy_block(health: &mut Health, _: &Enemy) {
    health.block = 0;
}
