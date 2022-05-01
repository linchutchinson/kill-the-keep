use crate::prelude::*;

pub struct CardZones {
    pub hand: Vec<Entity>,
    pub deck: Vec<Entity>,
    pub discard: Vec<Entity>,
}

impl CardZones {
    pub fn new() -> Self {
        Self {
            hand: Vec::new(),
            deck: Vec::new(),
            discard: Vec::new(),
        }
    }

    pub fn clear_all(&mut self) {
        self.hand = Vec::new();
        self.deck = Vec::new();
        self.discard = Vec::new();
    }
}
