// mobs/utils.rs

use crate::entity::Entity;
use crate::mobs::{Mob, MobData};  // Make sure you adjust this import based on your module structure

pub trait MobAI {
    fn take_turn(&mut self, player: &Entity);
}

pub struct MobController {
    mob: Mob,
}

impl MobController {
    pub fn new(mob: Mob) -> Self {
        MobController { mob }
    }
}

impl MobAI for MobController {
    fn take_turn(&mut self, player: &Entity) {
        // Implement AI behavior for the mob
        // Example: Move towards the player and attack
    }
}
