use crate::prelude::*;

#[system]
pub fn refill_energy(#[resource] energy: &mut Energy) {
    energy.current = energy.max;
}