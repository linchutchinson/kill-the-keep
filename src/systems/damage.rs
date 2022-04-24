use crate::prelude::*;

#[system(for_each)]
#[read_component(StatusEffect)]
#[read_component(DamageMultiplier)]
#[read_component(IncomingEffect)]
#[read_component(OutgoingEffect)]
pub fn apply_damage_multipliers(ecs: &mut SubWorld, message: &mut DealDamageMessage) {
    let mut status_query = <(Entity, &StatusEffect, &DamageMultiplier)>::query();
    let mut final_damage = message.amount;

    //Outgoing Status
    status_query
        .iter(ecs)
        .filter(|(_, status, _)| { status.target == message.source })
        .filter(|(entity, _, _)| { ecs.entry_ref(**entity).unwrap().get_component::<OutgoingEffect>().is_ok() })
        .for_each(|(_, _, damage_multiplier)| {
            final_damage = (damage_multiplier.multiplier * final_damage as f32) as i32;
        });

    //Incoming Status
    status_query
        .iter(ecs)
        .filter(|(_, status, _)| { status.target == message.target })
        .filter(|(entity, _, _)| { ecs.entry_ref(**entity).unwrap().get_component::<IncomingEffect>().is_ok() })
        .for_each(|(_, _, damage_multiplier)| {
            final_damage = (damage_multiplier.multiplier * final_damage as f32) as i32;
        });

    message.amount = final_damage;

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