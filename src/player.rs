// player.rs

use bracket_lib::prelude::*;
use crate::entity::Entity;
use crate::item::Item;
use crate::map::{Map, self, TileType};
use crate::ui::{UI, PopupWindow, self};

pub struct Player {
    pub entity: Entity,
    pub hp: i32,
    pub max_hp: i32,
    pub score: i32,
    pub hunger: i32,
    pub thirst: i32,
    pub inventory: Vec<Item>,
    pub selected_item: Option<usize>, // Track the selected inventory item
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        let entity = Entity::new(x, y, to_cp437('@'), YELLOW, BLACK);
        let mut inv = Vec::new();

        // Push some items to the inventory
        inv.push(Item::new(0, 0, "data/items/drink/Milk.json").unwrap());
        inv.push(Item::new(0, 0, "data/items/books/The_Bible.json").unwrap());

        Player {
            entity,
            hp: 100,
            max_hp: 100,
            score: 0,
            hunger: 100,
            thirst: 100,
            inventory: inv,
            selected_item: Some(0), // Select the first item by default
        }
    }

    pub fn draw(&self, ctx: &mut BTerm, map: &Map) {
        self.entity.draw(ctx, map);
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map, ui: &mut UI) {
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
                VirtualKeyCode::E => self.open_inventory(ui),
                // Add more keybindings for item selection and inspection
                VirtualKeyCode::Up => self.select_previous_item(ui),
                VirtualKeyCode::Down => self.select_next_item(ui),
                VirtualKeyCode::I => self.inspect_selected_item(ui, ctx),
                _ => {}
            }
        }
    }

    fn move_by(&mut self, dx: i32, dy: i32, map: &Map) {
        // Wall at where we're moving, don't move
        if map.get_tile(self.entity.x + dx, self.entity.y + dy) == TileType::Wall {
            return;
        }

        // Move by the given amount
        self.entity.x += dx;
        self.entity.y += dy;
    }

    fn open_inventory(&mut self, ui: &mut UI) {
        // Create an inventory popup
        ui.create_popup(10, 10, 40, 20, "Inventory");
    
        // Add inventory items to the popup content
        for (index, item) in self.inventory.iter().enumerate() {
            if let Some(active_popup_index) = ui.active_popup {
                if let Some(popup) = ui.popup_windows.get_mut(active_popup_index) {
                    // Add arrow symbol for the selected item
                    let content = if Some(index) == self.selected_item {
                        format!("> {}", &item.data.name)
                    } else {
                        item.data.name.clone()
                    };
                    popup.add_content(&content);
                }
            }
        }
    
        // Update the selected item to the first item in the inventory
        self.selected_item = Some(0);
    }

    fn select_previous_item(&mut self, ui: &mut UI) {
        if let Some(selected_item) = self.selected_item {
            if selected_item > 0 {
                self.selected_item = Some(selected_item - 1);
                // update iventory popup
                self.update_inventory(ui);
            }
        }
    }

    fn select_next_item(&mut self, ui: &mut UI) {
        if let Some(selected_item) = self.selected_item {
            if selected_item < self.inventory.len() - 1 {
                self.selected_item = Some(selected_item + 1);
                // update iventory popup
                self.update_inventory(ui);
            }
        }
    }

    fn inspect_selected_item(&self, ui: &mut UI, ctx: &mut BTerm) {
        if let Some(selected_item) = self.selected_item {
            if let Some(item) = self.inventory.get(selected_item) {
                item.inspect(ctx, ui);
            }
        }
    }

    fn update_inventory(&mut self, ui: &mut UI) {
        // TODO: add arrow to inventory item to show its selected in the popup
        
        // Update the inventory popup content with arrow symbol for the selected item
        if let Some(selected_item) = self.selected_item {
            if let Some(active_popup_index) = ui.active_popup {
                if let Some(popup) = ui.popup_windows.get_mut(active_popup_index) {
                    for (index, content) in popup.content.iter_mut().enumerate() {
                        if index == selected_item {
                            // Add arrow symbol to the selected item
                            *content = format!("> {}", content);
                        } else {
                            // Remove arrow symbol from other items
                            *content = content.trim_start_matches("> ").to_string();
                        }
                    }
                }
            }
        }
    }
}
