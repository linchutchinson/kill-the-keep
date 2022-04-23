use crate::prelude::*;

pub struct Energy {
    pub current: i32,
    pub max: i32,
}

impl Energy {
    pub fn new() -> Self {
        Self {
            current: 3,
            max: 3,
        }
    }
}
