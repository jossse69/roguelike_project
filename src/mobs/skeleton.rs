// mobs/skeleton.rs

use crate::entity::Entity;
use crate::mobs::utils::{MobAI, MobController};  // Make sure you adjust this import based on your module structure

pub struct SkeletonAI {
    controller: MobController,
}

impl SkeletonAI {
    pub fn new(controller: MobController) -> Self {
        SkeletonAI { controller }
    }
}

impl MobAI for SkeletonAI {
    fn take_turn(&mut self, player: &Entity) {
        // Implement AI behavior specific to skeletons
        // Example: Move towards the player and attack
    }
}
