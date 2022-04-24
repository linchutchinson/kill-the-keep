use crate::prelude::*;

#[system(for_each)]
#[read_component(StatusEffect)]
#[read_component(DamageMultiplier)]
pub fn apply_damage_multipliers(ecs: &mut SubWorld, message: &mut DealDamageMessage) {
    let mut status_query = <(&StatusEffect, &DamageMultiplier)>::query();

    status_query
        .iter(ecs)
        .filter(|(status, _)| { status.target == message.target })
        .for_each(|(_, damage_multiplier)| {
            message.amount = (damage_multiplier.multiplier * message.amount as f32) as i32;
        });

}

#[system(for_each)]
#[write_component(Health)]
pub fn deal_damage(ecs: &mut SubWorld, entity: &Entity, message: &DealDamageMessage, commands: &mut CommandBuffer) {
    let mut target_ref = ecs.entry_mut(message.target).unwrap();

    if let Ok(health) = target_ref.get_component_mut::<Health>() {
        let mut damage_to_deal = message.amount;

        if health.block >= damage_to_deal {
            health.block -= damage_to_deal;
        } else {
            damage_to_deal -= health.block;
            health.block = 0;
            health.current -= damage_to_deal;
        }
    }
    
    commands.remove(*entity);
}