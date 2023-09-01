mod entity;
mod player;
mod ui;
mod mobs;
mod map;
mod progen;  // Import the progen module

use bracket_lib::{prelude::*, color};
use entity::Entity;
use mobs::Mob;
use player::Player;
use ui::UI;
use map::{Map, TileType};

struct State {
    player: Player,
    ui: UI,
    mobs: Vec<Mob>,
    map: Map,
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

        State {
            player: Player::new(40, 25),
            ui: UI::new(),
            mobs,
            map,
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

        self.player.draw(ctx);

        for mob in &self.mobs {
            mob.draw(ctx);
        }

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
