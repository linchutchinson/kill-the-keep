use crate::prelude::*;

#[system(for_each)]
pub fn handle_add_card_messages(
    entity: &Entity,
    _msg: &Message,
    add_card: &AddCardToZone,
    commands: &mut CommandBuffer,
    #[resource] card_db: &mut CardDB,
    #[resource] zones: &mut CardZones,
) {
    let card = card_db.get_card_from_id(add_card.id);

    let card_entity = card
        .spawn_as_entity(commands)
        .expect("Failed to spawn card to add to zone.");

    match add_card.zone {
        CardZone::Discard => {
            zones.discard.push(card_entity);
        }

        _ => {
            eprintln!(
                "Did not implement adding cards to zone: {:?}",
                add_card.zone
            );
        }
    }

    commands.remove(*entity);
}
