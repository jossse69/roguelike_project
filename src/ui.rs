// ui.rs

use bracket_lib::prelude::*;

const MESSAGE_LOG_MAX_LINES: usize = 5; // Maximum lines in the message log

pub struct UI {
    message_log: Vec<String>,
}

impl UI {
    pub fn new() -> Self {
        UI {
            message_log: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: &str) {
        self.message_log.push(message.to_owned());

        // Remove oldest messages if log exceeds max lines
        if self.message_log.len() > MESSAGE_LOG_MAX_LINES {
            let num_to_remove = self.message_log.len() - MESSAGE_LOG_MAX_LINES;
            self.message_log.drain(0..num_to_remove);
        }
    }

    pub fn draw(&self, ctx: &mut BTerm) {
        // Draw message log with background
        ctx.draw_box(
            0,
            43,
            79,
            6,
            RGB::named(WHITE),
            RGB::named(BLACK),
        );
        for (i, message) in self.message_log.iter().enumerate() {
            ctx.print(1, 44 + i as i32, message);
        }

        // Draw player stats with background
        ctx.draw_box(
            0,
            0,
            17,
            3,
            RGB::named(WHITE),
            RGB::named(BLACK),
        );
        ctx.print_color(
            1,
            1,
            RGB::named(WHITE),
            RGB::named(BLACK),
            "Player Stats:",
        );
        // Add player stats here
    }
}
