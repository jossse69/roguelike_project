// item.rs

use bracket_lib::prelude::*;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use crate::entity::Entity;
use crate::map::Map;
use crate::ui::{self, PopupWindow, UI};
#[derive(Debug, Deserialize, Clone)]
pub struct ItemData {
    pub name: String,
    pub value: i32,
    pub description: String,
    pub onuse: Option<OnUse>,
    pub itemtype: String,
}


#[derive(Debug, Deserialize, Clone)]
pub struct OnUse {
    pub r#type: String,
    #[serde(default)]
    pub hunger_restoration: i32,
    #[serde(default)]
    pub thirst_quenching: i32,
    // Add other possible effects here
}

#[derive(Clone)]
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

    pub fn inspect(&self, ctx: &mut BTerm, ui: &mut UI) {
        // Create a new popup window
        let mut popup = PopupWindow::new(20, 10, 40, 20, "Item Description");
    
        // Add the item's description to the popup window content
        popup.add_content(&self.data.description);
    
        // Add the popup window to the UI
        ui.create_popup(20, 10, 40, 20, "Item Description");
        ui.popup_windows.push(popup);
    }

    // Implement item-specific methods here
}
