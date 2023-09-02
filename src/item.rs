// item.rs

use bracket_lib::prelude::*;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use crate::entity::Entity;
use crate::map::Map;
#[derive(Debug, Deserialize)]
pub struct ItemData {
    pub name: String,
    pub value: i32,
    pub description: String,
    pub onuse: Option<OnUse>,
    pub itemtype: String,
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

        //let glyph dependent on the item type on item data
        let glyph = match data.itemtype.as_str() {
            "armor" => to_cp437('+'),
            "book" => to_cp437('&'),
            "food" => to_cp437('*'),
            "drink" => to_cp437('-'),
            "mellee" => to_cp437('!'),
            "ranged" => to_cp437(':'),
            _ => to_cp437(' '),
        };

        let entity = Entity::new(x, y, glyph as FontCharType, GOLD, BLACK);

        Ok(Item { entity, data })
    }

    pub fn draw(&self, ctx: &mut BTerm, map: &Map) {
        self.entity.draw(ctx, map);
    }

    // Implement item-specific methods here
}
