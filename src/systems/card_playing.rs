use crate::prelude::*;

#[system(for_each)]
#[read_component(SelectTarget)]
#[read_component(Vec2)]
#[read_component(Enemy)]
#[read_component(Sprite)]
#[read_component(Player)]
pub fn select_card_targets(ecs: &mut SubWorld, card_entity: &Entity, _: &Selected, commands: &mut CommandBuffer) {
    if is_mouse_button_released(MouseButton::Left) {
        let mut player_query = <(Entity, &Player)>::query();

        let player_entity = player_query.iter(ecs).nth(0).unwrap().0;

        if let Ok(_targeted_card) = ecs.entry_ref(*card_entity).unwrap().get_component::<SelectTarget>() {
            let mut targets_query = <(Entity, &Vec2, &Enemy, &Sprite)>::query();

            let mut target: Option<&Entity> = None;
            let mouse_pos = mouse_position();
    
            targets_query
                .iter(ecs)
                .for_each(|(entity, pos, _, sprite)| {
                    let tl = Vec2::new(pos.x - sprite.texture.width() * 0.5, pos.y - sprite.texture.height());
                    let br = tl + Vec2::new(sprite.texture.width(), sprite.texture.height());
    
                    if mouse_pos.0 > tl.x && mouse_pos.0 < br.x
                        && mouse_pos.1 > tl.y && mouse_pos.1 < br.y
                    {
                        target = Some(entity);
                    }
                });
            
            if let Some(target) = target {
                println!("Playing card on a target!");
                commands.push(((), PlayCardMessage{ source: *player_entity, card:  *card_entity },  PlayTargetedCardMessage{ target: *target }));
            }
        } else {
            println!("Playing a non-targeted card.");
            commands.push(((), PlayCardMessage{ source: *player_entity, card:  *card_entity }));
        }
    }
}

#[system(for_each)]
#[read_component(Card)]
#[read_component(EnergyCost)]
#[write_component(Health)]
pub fn play_card(
    entity: &Entity, 
    message: &PlayCardMessage, 
    ecs: &mut SubWorld, 
    commands: &mut CommandBuffer, 
    #[resource] card_zones: &mut CardZones,
    #[resource] energy: &mut Energy
) {
    let new_hand = card_zones.hand.iter()
    .filter(|card| message.card != **card)
    .map(|card| *card);

    card_zones.hand = Vec::from_iter(new_hand);

    card_zones.discard.push(message.card);

    if let Ok(energy_cost) = ecs.entry_ref(message.card).unwrap().get_component::<EnergyCost>() {
        energy.current -= energy_cost.amount;
    }

    commands.remove(*entity);
}
