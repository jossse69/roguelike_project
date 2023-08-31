// player.rs

use bracket_lib::prelude::*;
use crate::entity::Entity;

pub struct Player {
    entity: Entity,
    pub hp: i32,
    pub max_hp: i32,
    pub score: i32,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        let entity = Entity::new(x, y, to_cp437('@'), YELLOW, BLACK);
        Player { entity, hp: 100, max_hp: 100, score: 0 }
    }

    pub fn draw(&self, ctx: &mut BTerm) {
        self.entity.draw(ctx);
    }

    pub fn update(&mut self, ctx: &mut BTerm) {
        // Handle player movement
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Numpad8 => self.move_by(0, -1),
                VirtualKeyCode::Numpad2 => self.move_by(0, 1),
                VirtualKeyCode::Numpad4 => self.move_by(-1, 0),
                VirtualKeyCode::Numpad6 => self.move_by(1, 0),
                VirtualKeyCode::Numpad7 => self.move_by(-1, -1),
                VirtualKeyCode::Numpad9 => self.move_by(1, -1),
                VirtualKeyCode::Numpad1 => self.move_by(-1, 1),
                VirtualKeyCode::Numpad3 => self.move_by(1, 1),
                _ => {}
            }
        }
    }

    fn move_by(&mut self, dx: i32, dy: i32) {
        self.entity.x += dx;
        self.entity.y += dy;
    }

    // Add player-specific methods and behaviors here
}
