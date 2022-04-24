use crate::prelude::*;

pub struct CombatantTextures {
    pub hero: Texture2D,
    pub orc: Texture2D,
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
        }
    }
}
