use crate::prelude::*;

pub struct UITextures {
    pub crosshair: Texture2D,
    pub attack_intent: Texture2D,
    pub vulnerability: Texture2D,
}

impl UITextures {
    pub fn new() -> Self {
        Self {
            crosshair: Texture2D::from_file_with_format(
                include_bytes!("../assets/sprites/targeting_icon.png"),
                None,
            ),
            attack_intent: Texture2D::from_file_with_format(
                include_bytes!("../assets/sprites/attack_intent_icon.png"),
                None,
            ),
            vulnerability: Texture2D::from_file_with_format(
                include_bytes!("../assets/sprites/vulnerability_icon.png"),
                None,
            ),
        }
    }
}
