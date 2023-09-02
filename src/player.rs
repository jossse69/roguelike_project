// player.rs

use bracket_lib::prelude::*;
use crate::entity::Entity;
use crate::item::Item;
use crate::map::{Map, self, TileType};
use crate::ui::{UI, PopupWindow};

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
                VirtualKeyCode::Up => self.select_previous_item(),
                VirtualKeyCode::Down => self.select_next_item(),
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
        let mut inventory_popup = PopupWindow::new(10, 10, 40, 20, "Inventory");

        // Add inventory items to the popup content
        for item in &self.inventory {
            inventory_popup.add_content(&item.data.name);
        }

        // Update the selected item to the first item in the inventory
        self.selected_item = Some(0);

        ui.popup_windows.push(inventory_popup);
    }

    fn select_previous_item(&mut self) {
        if let Some(selected_item) = self.selected_item {
            if selected_item > 0 {
                self.selected_item = Some(selected_item - 1);
            }
        }
    }

    fn select_next_item(&mut self) {
        if let Some(selected_item) = self.selected_item {
            if selected_item < self.inventory.len() - 1 {
                self.selected_item = Some(selected_item + 1);
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
}
