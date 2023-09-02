// ui.rs

use bracket_lib::prelude::*;
use crate::player::Player;

const MESSAGE_LOG_MAX_LINES: usize = 5; // Maximum lines in the message log

pub struct UI {
    message_log: Vec<String>,
    popup_windows: Vec<PopupWindow>,
}

impl UI {
    pub fn new() -> Self {
        UI {
            message_log: Vec::new(),
            popup_windows: Vec::new(),
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

    pub fn create_popup(&mut self, x: i32, y: i32, width: i32, height: i32, title: &str) {
        let popup = PopupWindow::new(x, y, width, height, title);
        self.popup_windows.push(popup);
    }

    pub fn update_popup_title(&mut self, index: usize, title: &str) {
        if let Some(popup) = self.popup_windows.get_mut(index) {
            popup.update_title(title);
        }
    }

    pub fn draw_popups(&self, ctx: &mut BTerm) {
        for popup in &self.popup_windows {
            popup.draw(ctx);
        }
    }

    pub fn draw(&self, ctx: &mut BTerm, player: &Player) {
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
            47,
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
        ctx.print_color(
            1,
            2,
            RGB::named(GREEN2),
            RGB::named(BLACK),
            format!("HP: {}/{}", player.hp, player.max_hp),
        );
        ctx.print_color(
            13,
            2,
            RGB::named(GOLD),
            RGB::named(BLACK),
            format!("score: {}", player.score),
        );
        ctx.print_color(
            22,
            2,
            RGB::named(WHEAT),
            RGB::named(BLACK),
            format!("hunger: {}", player.hunger),
        );
        ctx.print_color(
            34,
            2,
            RGB::named(CYAN),
            RGB::named(BLACK),
            format!("thirst: {}", player.thirst),
        );

        // Draw popup windows
        self.draw_popups(ctx);
    }
}

pub struct PopupWindow {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    title: String,
    content: Vec<String>,
}

impl PopupWindow {
    pub fn new(x: i32, y: i32, width: i32, height: i32, title: &str) -> Self {
        PopupWindow {
            x,
            y,
            width,
            height,
            title: title.to_string(),
            content: Vec::new(),
        }
    }

    pub fn add_content(&mut self, content: &str) {
        self.content.push(content.to_string());
    }

    pub fn update_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    pub fn draw(&self, ctx: &mut BTerm) {
        // Draw the popup window background
        ctx.draw_box(
            self.x,
            self.y,
            self.width,
            self.height,
            RGB::named(WHITE),
            RGB::named(BLACK),
        );

        // Draw the title
        ctx.print_color(
            self.x + 1,
            self.y,
            RGB::named(WHITE),
            RGB::named(BLACK),
            &self.title,
        );

        // Draw content within the window
        for (i, content) in self.content.iter().enumerate() {
            ctx.print(self.x + 1, self.y + 1 + i as i32, content);
        }
    }
}
