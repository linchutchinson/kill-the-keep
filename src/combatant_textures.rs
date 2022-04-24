use crate::prelude::*;

pub struct CombatantTextures {
    pub hero: Texture2D,
    pub orc: Texture2D,
    pub spider: Texture2D,
    pub crow: Texture2D,
}

impl CombatantTextures {
    pub fn new() -> Self {
        Self {
            hero: Texture2D::from_file_with_format(
                include_bytes!("../assets/sprites/knight.png"),
                None,
            ),
            orc: Texture2D::from_file_with_format(
                include_bytes!("../assets/sprites/orc.png"),
                None,
            ),
            spider: Texture2D::from_file_with_format(
                include_bytes!("../assets/sprites/spider.png"),
                None,
            ),
            crow: Texture2D::from_file_with_format(
                include_bytes!("../assets/sprites/crow.png"),
                None,
            ),
        }
    }
}
