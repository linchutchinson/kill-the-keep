use crate::prelude::*;

#[system(for_each)]
#[write_component(Health)]
pub fn apply_block(
    ecs: &mut SubWorld, 
    entity: &Entity, 
    block_message: &AddBlockMessage, 
    commands: &mut CommandBuffer
) {
        let mut target_ref = ecs.entry_mut(block_message.target).unwrap();

        if let Ok(mut health) = target_ref.get_component_mut::<Health>() {
            health.block += block_message.amount;
        }

        commands.remove(*entity);
}

#[system(for_each)]
pub fn clear_block(health: &mut Health) {
    health.block = 0;
}