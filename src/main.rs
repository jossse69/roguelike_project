// main.rs

mod entity;
mod player;
mod ui;  // Import the ui module

use bracket_lib::{prelude::*, color};
use entity::Entity;
use player::Player;
use ui::UI;  // Import the UI struct

struct State {
    player: Player,
    ui: UI,  // Include the UI in the State
    // Add more entities here as needed
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(40, 25),
            ui: UI::new(),  // Initialize the UI
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
        self.ui.draw(ctx);  // Draw the UI
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
