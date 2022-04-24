use crate::prelude::*;

#[system(for_each)]
#[read_component(StatusEffect)]
#[read_component(Vulnerability)]
#[write_component(Duration)]
pub fn apply_vulnerability(ecs: &mut SubWorld, entity: &Entity, message: &ApplyVulnerabilityMessage, commands: &mut CommandBuffer, #[resource] ui_tex: &UITextures) {    
    let mut vuln_query = <(&StatusEffect, &Vulnerability, &mut Duration)>::query();

    if let Some((_, _, duration)) = vuln_query
        .iter_mut(ecs)
        .filter(|(effect, _, _)| {effect.target == message.target})
        .nth(0) {
            duration.rounds += message.amount;
    } else {
        commands.push(((), 
        StatusEffect{ target: message.target }, 
        DamageMultiplier{ multiplier: 1.5 }, 
        Duration{ rounds: message.amount }, 
        Vulnerability, 
        Sprite{ texture: ui_tex.vulnerability, }));
    }

    commands.remove(*entity);
}

#[system(for_each)]
pub fn reduce_remaining_duration_of_effects(entity: &Entity, _: &StatusEffect, duration: &mut Duration, commands: &mut CommandBuffer) {
    duration.rounds -= 1;

    if duration.rounds < 1 {
        commands.remove(*entity);
    }
}
