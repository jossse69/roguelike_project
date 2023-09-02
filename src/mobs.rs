// mobs.rs

use serde::Deserialize;
use std::fs;
use bracket_lib::prelude::*;
use crate::{entity::Entity, map::Map};  // Import the Entity struct

#[derive(Debug, Deserialize, Clone)]
pub struct MobData {
    pub name: String,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub description: String,
    pub visuals: Visuals,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Visuals {
    pub glyph: char,
    pub color: (u8, u8, u8),  // Use a tuple for color
}

pub fn load_mob_data(filename: &str) -> Result<MobData, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(filename)?;
    let mob_data: MobData = serde_json::from_str(&contents)?;
    Ok(mob_data)
}

pub struct Mob {
    entity: Entity,
    mob_data: MobData,  // Add mob_data field
    HP: i32,
    max_HP: i32,
    attack: i32,
    defense: i32,
    speed: i32,
}

impl Mob {
    pub fn new(x: i32, y: i32, filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mob_data = load_mob_data(filename)?;
        let entity = Entity::new(x, y, mob_data.visuals.glyph as u16, mob_data.visuals.color, BLACK); // use the as u16

        let mob = Mob {
            entity,
            mob_data: mob_data.clone(),  // Clone mob_data
            HP: mob_data.hp,
            max_HP: mob_data.hp,
            attack: mob_data.attack,
            defense: mob_data.defense,
            speed: mob_data.speed,
        };

        Ok(mob)
    }

    pub fn draw(&self, ctx: &mut BTerm, map: &Map) {
        self.entity.draw(ctx, map);
    }

    // Add mob-specific methods here
}
