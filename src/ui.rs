use bracket_lib::prelude::*;
use rand::seq::index;
use crate::player::Player;

const MESSAGE_LOG_MAX_LINES: usize = 5; // Maximum lines in the message log

pub struct UI {
    pub message_log: Vec<String>,
    pub popup_windows: Vec<PopupWindow>,
    pub active_popup: Option<usize>, // Track the index of the active popup, if any
    pub previous_popup: Option<usize>,
}

impl UI {
    pub fn new() -> Self {
        UI {
            message_log: Vec::new(),
            popup_windows: Vec::new(),
            active_popup: None,
            previous_popup: None,
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


        //add the previous popup to the previous popup list
        if let Some(active_popup_index) = self.active_popup {
            self.previous_popup = Some(active_popup_index);
        }

        // Add the new popup to the list
        self.popup_windows.push(popup);

        // Set the active popup to the newly created one
        self.active_popup = Some(self.popup_windows.len() - 1);
    }

    pub fn remove_active_popup(&mut self) {
        if let Some(active_popup_index) = self.active_popup {
            self.popup_windows.remove(active_popup_index);
            //set the active popup to previous popup
            self.active_popup = self.previous_popup
        }
    }


    pub fn remove_popup(&mut self, index: usize) {
        self.popup_windows.remove(index);

        // Reset the active popup to None if the removed popup was the active one
        if let Some(active_popup_index) = self.active_popup {
            if index == active_popup_index {
                self.active_popup = None;
            }
        }
    }

    pub fn draw(&mut self, ctx: &mut BTerm, player: &Player) {
        // Handle player input to close the active popup on "Escape" key press
        if ctx.key == Some(VirtualKeyCode::Escape) {
            if let Some(active_popup_index) = self.active_popup {
                self.remove_active_popup();
            }
        }

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
        for popup in &self.popup_windows {
            popup.draw(ctx);
        }
    }
}

pub struct PopupWindow {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub title: String,
    pub content: Vec<String>,
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

    pub fn draw(&self, ctx: &mut BTerm) {
        // Draw popup window background
        ctx.draw_box(
            self.x,
            self.y,
            self.width,
            self.height,
            RGB::named(WHITE),
            RGB::named(BLACK),
        );

        // Draw popup window title
        ctx.print(self.x + 1, self.y + 1, &self.title);

        // Draw popup window content
        for (i, line) in self.content.iter().enumerate() {
            ctx.print(self.x + 3, self.y + 4 + i as i32, line);
        }


        
    }

    pub fn update_content(&mut self, index: usize, content: &str) {
        // Ensure the index is within bounds before updating
        if index < self.content.len() {
            self.content[index] = content.to_string();
        }
    }    
}
