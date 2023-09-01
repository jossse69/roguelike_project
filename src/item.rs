// item.rs

use bracket_lib::prelude::*;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use crate::entity::Entity;
#[derive(Debug, Deserialize)]
pub struct ItemData {
    pub name: String,
    pub value: i32,
    pub description: String,
    pub visuals: Visuals,
    pub onuse: Option<OnUse>,
}

#[derive(Debug, Deserialize)]
pub struct Visuals {
    pub glyph: char,
    pub color: (u8, u8, u8),
}

#[derive(Debug, Deserialize)]
pub struct OnUse {
    pub r#type: String,
    #[serde(default)]
    pub hunger_restoration: i32,
    #[serde(default)]
    pub thirst_quenching: i32,
    // Add other possible effects here
}

pub struct Item {
    pub entity: Entity,
    pub data: ItemData,
}

impl Item {
    pub fn new(x: i32, y: i32, filename: &str) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(filename)?;
        let data: ItemData = serde_json::from_str(&contents)?;
        let entity = Entity::new(x, y, data.visuals.glyph as FontCharType, data.visuals.color, BLACK);

        Ok(Item { entity, data })
    }

    pub fn draw(&self, ctx: &mut BTerm) {
        self.entity.draw(ctx);
    }

    // Implement item-specific methods here
}
