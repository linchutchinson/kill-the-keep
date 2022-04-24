use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn check_for_deaths(ecs: &mut SubWorld, #[resource] turn_state: &mut TurnState) {
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
        .for_each(|(_, health)| {
            if health.current > 0 {
                all_enemies_dead = false;
            }
        });
    
    if all_enemies_dead {
        *turn_state = TurnState::BattleOver{ player_victorious: true };
    }
}