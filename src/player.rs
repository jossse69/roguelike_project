// player.rs

use bracket_lib::prelude::*;
use crate::entity::Entity;
use crate::map::{Map, self, TileType};
pub struct Player {
    entity: Entity,
    pub hp: i32,
    pub max_hp: i32,
    pub score: i32,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        let entity = Entity::new(x, y, to_cp437('@'), YELLOW, BLACK);
        Player { entity, hp: 100, max_hp: 100, score: 0}
    }

    pub fn draw(&self, ctx: &mut BTerm) {
        self.entity.draw(ctx);
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map) {
        // Handle player movement
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Numpad8 => self.move_by(0, -1, map),
                VirtualKeyCode::Numpad2 => self.move_by(0, 1, map),
                VirtualKeyCode::Numpad4 => self.move_by(-1, 0, map),
                VirtualKeyCode::Numpad6 => self.move_by(1, 0, map),
                VirtualKeyCode::Numpad7 => self.move_by(-1, -1, map),
                VirtualKeyCode::Numpad9 => self.move_by(1, -1, map),
                VirtualKeyCode::Numpad1 => self.move_by(-1, 1, map),
                VirtualKeyCode::Numpad3 => self.move_by(1, 1, map),
                _ => {}
            }
        }
    }

    fn move_by(&mut self, dx: i32, dy: i32, map: &Map) {
        //wall at were moving, dont move
        if map.get_tile(self.entity.x + dx, self.entity.y + dy) == TileType::Wall {
            return;
        }

        // Move by the given amount
        self.entity.x += dx;
        self.entity.y += dy;
    }

    // Add player-specific methods and behaviors here
}
