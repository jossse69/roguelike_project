// main.rs

mod entity;
mod player;
mod ui;  // Import the ui module

use bracket_lib::{prelude::*, color};
use entity::Entity;
use mobs::Mob;
use player::Player;
use ui::UI;  // Import the UI struct
mod mobs;
struct State {
    player: Player,
    ui: UI,  // Include the UI in the State
    mobs: Vec<Mob>,  // Add a vector to store mobs
    // Add more entities here as needed
}

impl State {
    fn new() -> Self {
        let mut mobs = Vec::new();

        // Create and add mobs here
        if let Ok(goblin) = Mob::new(20, 10, "data/mobs/goblin.json") {
            mobs.push(goblin);
        }

        State {
            player: Player::new(40, 25),
            ui: UI::new(),  // Initialize the UI
            mobs,
            // Initialize other entities here
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        if let Some(VirtualKeyCode::Escape) = ctx.key {
            ctx.quitting = true;
        }

        self.player.update(ctx);
        ctx.cls();
        self.ui.add_message("Hello world!");
        self.player.draw(ctx);

        for mob in &self.mobs {
            mob.draw(ctx);  // Draw mobs
        }

        self.ui.draw(ctx, &self.player);  // Draw the UI
        // Draw other entities here
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Game")
        .build()?;

    let state = State::new();

    main_loop(context, state)
}
