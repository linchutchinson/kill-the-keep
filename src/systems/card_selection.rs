use crate::prelude::*;

#[system]
#[read_component(Card)]
#[read_component(Selected)]
pub fn select_cards(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] card_zones: &CardZones, #[resource] energy: &mut Energy) {
    let mut cards = <(Entity, &Card)>::query();
    
    if is_mouse_button_released(MouseButton::Left) {
        cards
            .iter(ecs)
            .filter(|(entity, _)| ecs.entry_ref(**entity).unwrap().get_component::<Selected>().is_ok())
            .for_each(|(entity, _)| {
                println!("Deselected a card!");
                commands.remove_component::<Selected>(*entity)
            });
    }
    
    let card_count = card_zones.hand.iter().count();

    let mut closest_card: Option<&Entity> = None;
    let mut closest_x = 999999.9;

    let mouse_pos = mouse_position();

    if mouse_pos.1 >= screen_height() - CARD_HEIGHT * 0.5 {
        card_zones.hand
        .iter()
        .enumerate()
        .for_each(|(idx, entity)| {
            let pos_x = calculate_card_center_x(idx as i32, card_count as i32);
            let x_offset = (mouse_pos.0 - pos_x).abs();
            if x_offset < (CARD_WIDTH / 2.0) && x_offset < closest_x {
                closest_card = Some(entity);
                closest_x = x_offset;
            }
        });
    }
    
    if let Some(hovered_card) = closest_card{
        if energy.current > 0 && is_mouse_button_pressed(MouseButton::Left) {
            commands.add_component(*hovered_card, Selected);
        }
    }
}