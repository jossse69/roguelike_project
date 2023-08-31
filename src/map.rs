// map.rs

use bracket_lib::prelude::*;

pub struct Map {
    tiles: Vec<TileType>,
    width: i32,
    height: i32,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        let tiles = vec![TileType::Floor; (width * height) as usize];
        Map {
            tiles,
            width,
            height,
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => {
                        ctx.set(x, y, DARKGREY, BLACK, to_cp437('.'));
                    }
                    TileType::Wall => {
                        ctx.set(x, y, WHITESMOKE, BLACK, to_cp437('#'));
                    }
                }
            }
        }
    }
}

pub fn map_idx(x: i32, y: i32) -> usize {
    (y as usize * SCREEN_WIDTH) + x as usize
}

pub const SCREEN_WIDTH: usize = 80;
pub const SCREEN_HEIGHT: usize = 50;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}
