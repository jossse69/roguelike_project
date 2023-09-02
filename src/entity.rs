// entity.rs

use bracket_lib::prelude::*;
use crate::map::Map;

#[derive(Debug, Clone)]
pub struct Entity {
    pub x: i32,
    pub y: i32,
    pub glyph: FontCharType,
    pub fg_color: RGB,
    pub bg_color: RGB,
}

impl Entity {
    pub fn new(x: i32, y: i32, glyph: FontCharType, fg_color: (u8, u8, u8), bg_color: (u8, u8, u8)) -> Self {
        Entity {
            x,
            y,
            glyph,
            fg_color: RGB::from_u8(fg_color.0, fg_color.1, fg_color.2),
            bg_color: RGB::from_u8(bg_color.0, bg_color.1, bg_color.2),
        }
    }

    pub fn draw(&self, ctx: &mut BTerm, map: &Map) {

        //if not on visible tile
        if !map.get_tile_visbility(self.x, self.y) {
            return;
        }

        ctx.set(self.x, self.y, self.fg_color, self.bg_color, self.glyph);
    }
}
