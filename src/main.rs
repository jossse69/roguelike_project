mod entity;
mod player;
mod ui;
mod mob;
mod map;
mod progen;
mod item;
mod ui_component;
mod mobs_loader;

use bracket_lib::{prelude::*, color};
use entity::Entity;
use mob::Mob;
use mobs_loader::MobLoader;
use player::Player;
use ui::UI;
use map::{Map, TileType, SCREEN_WIDTH};
use item::Item;
use rand::Rng;
struct State {
    player: Player,
    ui: UI,
    mobs: Vec<Mob>, // Add a vector to store mobs
    map: Map,
    items: Vec<Item>,
}

impl State {
    fn new() -> Self {
        let mut mobs = Vec::new();
        let mut rng = rand::thread_rng();

        // Initialize the MobLoader and load mobs
        let mob_loader = MobLoader::new();
        let loaded_mobs = mob_loader.load_mobs();

        // Add loaded mobs to the vector
        mobs.extend(loaded_mobs);

        let mut map = Map::new(map::SCREEN_WIDTH as i32, map::SCREEN_HEIGHT as i32);

        // Generate the dungeon map
        progen::generate_dungeon(&mut map);

        for mob in mobs.iter_mut() {
            // loop until it finds a random tile that is a walkable tile
            let mut found = false;
            while !found {
                let x = rng.gen_range(0..map.width);
                let y = rng.gen_range(0..map.height);
        
                // Check if the tile is walkable
                if map.get_tile(x, y) == TileType::Floor {
                    found = true;
                    // Place the mob
                    mob.entity.x = x;
                    mob.entity.y = y;
                }
            }
        }

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
            items,
        }
    }
}


impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.player.update(ctx, &mut self.map, &mut self.ui);
        self.map.update_fov(self.player.entity.x, self.player.entity.y);
        ctx.cls();
        self.ui.add_message("Hello world!");
    
        self.map.render(ctx);
    
        // Draw items
        for item in &self.items {
            item.draw(ctx, &self.map);
        }
    
        // Draw and execute mobs 
        for mob in &mut self.mobs {
            mob.execute_ai();
            mob.draw(ctx, &self.map);
        }
    
        self.player.draw(ctx, &self.map);
    
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
