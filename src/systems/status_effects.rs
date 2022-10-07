use crate::prelude::*;

#[system(for_each)]
#[read_component(StatusEffect)]
#[write_component(Duration)]
pub fn apply_status(
    ecs: &mut SubWorld,
    entity: &Entity,
    _message: &Message,
    target: &Targeted,
    status: &InflictsStatus,
    commands: &mut CommandBuffer,
    #[resource] ui_tex: &UITextures,
) {
    let mut status_query = <(&StatusEffect, &mut Duration)>::query();

    if let Some((_, duration)) = status_query
        .iter_mut(ecs)
        .filter(|(effect, _)| effect.target == target.target && effect.status_type == status.status)
        .nth(0)
    {
        duration.rounds += status.amount;
    } else {
        let status_entity = commands.push((
            (),
            StatusEffect {
                target: target.target,
                status_type: status.status,
            },
            Duration {
                rounds: status.amount,
            },
        ));

        match status.status {
            Status::Vulnerability => {
                commands.add_component(status_entity, IncomingEffect);
                commands.add_component(status_entity, DamageMultiplier { multiplier: 1.5 });
                commands.add_component(
                    status_entity,
                    Sprite {
                        texture: ui_tex.vulnerability,
                    },
                )
            }

            Status::Weakness => {
                commands.add_component(status_entity, OutgoingEffect);
                commands.add_component(status_entity, DamageMultiplier { multiplier: 0.75 });
                commands.add_component(
                    status_entity,
                    Sprite {
                        texture: ui_tex.weakness,
                    },
                );
            }
        }
    }

    commands.remove(*entity);
}

#[system(for_each)]
pub fn reduce_remaining_duration_of_effects(
    entity: &Entity,
    _: &StatusEffect,
    duration: &mut Duration,
    commands: &mut CommandBuffer,
) {
    duration.rounds -= 1;

    if duration.rounds < 1 {
        commands.remove(*entity);
    }
}
