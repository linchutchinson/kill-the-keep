use crate::prelude::*;

#[system(for_each)]
#[read_component(Vec2)]
#[read_component(Enemy)]
#[read_component(Sprite)]
pub fn select_card_targets(ecs: &mut SubWorld, card_entity: &Entity, _: &Selected, commands: &mut CommandBuffer) {
    if is_mouse_button_released(MouseButton::Left) {
        let mut query = <(Entity, &Vec2, &Enemy, &Sprite)>::query();

        let mut target: Option<&Entity> = None;
        let mouse_pos = mouse_position();

        query
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
            commands.push(((), PlayCardMessage{ card:  *card_entity, target: *target }));
        }
    }
}

#[system(for_each)]
#[read_component(Card)]
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

    if let Ok(mut enemy) = ecs.entry_mut(message.target) {
        let mut health = enemy.get_component_mut::<Health>().unwrap();
        health.current -= 6;
    }

    card_zones.discard.push(message.card);
    energy.current -= 1;

    commands.remove(*entity);
}
