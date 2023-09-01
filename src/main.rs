// main.rs

mod entity;
mod player;
mod ui;
mod mobs;
mod map;
mod progen;
mod item;  // Import the item module

use bracket_lib::{prelude::*, color};
use entity::Entity;
use mobs::Mob;
use player::Player;
use ui::UI;
use map::{Map, TileType, SCREEN_WIDTH};
use item::Item;  // Import the Item struct

struct State {
    player: Player,
    ui: UI,
    mobs: Vec<Mob>,
    map: Map,
    items: Vec<Item>,  // Add items vector
}

impl State {
    fn new() -> Self {
        let mut mobs = Vec::new();

        if let Ok(goblin) = Mob::new(20, 10, "data/mobs/goblin.json") {
            mobs.push(goblin);
        }

        let mut map = Map::new(map::SCREEN_WIDTH as i32, map::SCREEN_HEIGHT as i32);
        
        // Generate the dungeon map
        progen::generate_dungeon(&mut map);

        // Find the up stairs to place the player
        let mut player_loc = Point::new(0, 0);
        for i in 0..map.tiles.len() {
            if map.tiles[i] == TileType::UpStairs {
                player_loc.x = i as i32 % map.width;
                player_loc.y = i as i32 / map.width;
                break;
            }
        }

        // Create the Milk item and add it to the items vector
        let milk = Item::new(10, 10, "data/items/drink/Milk.json").unwrap();
        let mut items = Vec::new();
        items.push(milk);

        State {
            player: Player::new(player_loc.x, player_loc.y),
            ui: UI::new(),
            mobs,
            map,
            items,  // Initialize the items vector
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        if let Some(VirtualKeyCode::Escape) = ctx.key {
            ctx.quitting = true;
        }

        self.player.update(ctx, &mut self.map);
        ctx.cls();
        self.ui.add_message("Hello world!");

        self.map.render(ctx);

        // Draw items
        for item in &self.items {
            item.draw(ctx);
        }

        // Draw mobs
        for mob in &self.mobs {
            mob.draw(ctx);
        }
        self.player.draw(ctx);

        

        self.ui.draw(ctx, &self.player);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Game")
        .build()?;

    let state = State::new();

    main_loop(context, state)
}
