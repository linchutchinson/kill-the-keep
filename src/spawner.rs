use crate::prelude::*;

pub fn spawn_hero(ecs: &mut World) {
    ecs.push(
        (
            Vec2::ZERO,
            Player,
            Health {
                current: 80,
                max: 80,
            },
            Sprite {
                texture: Texture2D::from_file_with_format(
                    include_bytes!("../assets/sprites/knight.png"),
                    None,
                ),
            }
        )
    );
}

pub fn spawn_orc(ecs: &mut World) {
    ecs.push(
        (
            Vec2::ZERO,
            Enemy,
            Health {
                current: 30,
                max: 30,
            },
            Sprite {
                texture: Texture2D::from_file_with_format(
                    include_bytes!("../assets/sprites/orc.png"),
                    None,
                ),
            }
        )
    );
}

pub fn spawn_card_in_deck(ecs : &mut World, resources: &mut Resources) {
    let num = thread_rng().gen_range(0..100);
    let entity = ecs.push(
        (
            Card {
                name: format!("Attack {}", num).to_owned()
            },
        )
    );

    resources.get_mut::<CardZones>().unwrap().deck.push(entity);
}
