// main.rs

mod entity;
mod player;  // Import the player module

use bracket_lib::{prelude::*, color};
use entity::Entity;
use player::Player;  // Import the Player struct

struct State {
    player: Player,  // Use Player for the player character
    // Add more entities here as needed
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(40, 25),  // Initialize the player character
            // Initialize other entities here
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        if let Some(VirtualKeyCode::Escape) = ctx.key {
            ctx.quitting = true;
        }

        ctx.cls();

        // Draw the player character
        self.player.draw(ctx);
        // Draw other entities here

        // Update the player character
        self.player.update(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Game")
        .build()?;

    let state = State::new();

    main_loop(context, state)
}
