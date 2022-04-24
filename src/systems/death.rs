use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(EnemyIntent)]
pub fn check_for_deaths(ecs: &mut SubWorld, #[resource] turn_state: &mut TurnState, commands: &mut CommandBuffer) {
    let mut player_query = <(&Player, &Health)>::query();
    let mut all_players_dead = true;

    player_query
        .iter(ecs)
        .for_each(|(_, health)| {
            if health.current > 0 {
                all_players_dead = false;
            }
        });
    
    if all_players_dead {
        *turn_state = TurnState::BattleOver{ player_victorious: false };
        return;
    }

    let mut enemy_query = <(Entity, &Health)>::query();
    let mut all_enemies_dead = true;

    enemy_query
        .iter(ecs)
        .filter(|(entity, _)| {
            !ecs.entry_ref(**entity).unwrap().get_component::<Player>().is_ok()
        })
        .for_each(|(entity, health)| {
            if health.current > 0 {
                all_enemies_dead = false;
            } else {
                commands.remove(*entity);

                let mut intent_query = <(Entity, &EnemyIntent)>::query();

                intent_query.iter(ecs)
                    .filter(|(intent_entity, enemy_intent)| {
                        enemy_intent.enemy == *entity
                    })
                    .for_each(|(intent_entity, _)| {
                        commands.remove(*intent_entity);
                    });
            }
        });
    
    if all_enemies_dead {
        *turn_state = TurnState::BattleOver{ player_victorious: true };
    }
}