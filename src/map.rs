// map.rs

use bracket_lib::prelude::*;

pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
    pub rooms: Vec<Rect>,
    pub revealed_tiles: Vec<bool>, // Track revealed tiles
    pub visible_tiles: Vec<bool>,  // Track visible tiles
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height
    }

    fn point2d_to_index(&self, pt: Point) -> usize {
        let bounds = self.dimensions();
        ((pt.y * bounds.x) + pt.x)
            .try_into()
            .expect("Not a valid usize. Did something go negative?")
    }

    fn index_to_point2d(&self, idx: usize) -> Point {
        let bounds = self.dimensions();
        let w: usize = bounds
            .x
            .try_into()
            .expect("Not a valid usize. Did something go negative?");
        Point::new(idx % w, idx / w)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let pos = self.index_to_point2d(idx);
        let directions = [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ];

        for &dir in directions.iter() {
            let new_pos = (pos.x + dir.0, pos.y + dir.1);
            if self.in_bounds(Point::new(new_pos.0, new_pos.1)) {
                let new_idx = self.point2d_to_index(Point::new(new_pos.0, new_pos.1));
                if !self.is_opaque(new_idx) {
                    exits.push((new_idx, 1.0));
                }
            }
        }
        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let pos1 = self.index_to_point2d(idx1);
        let pos2 = self.index_to_point2d(idx2);
        DistanceAlg::Pythagoras.distance2d(pos1, pos2)
    }
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        let tiles = vec![TileType::Wall; (width * height) as usize];
        let revealed_tiles = vec![false; (width * height) as usize];
        let visible_tiles = vec![false; (width * height) as usize];

        Map {
            tiles,
            width,
            height,
            rooms: Vec::new(),
            revealed_tiles,
            visible_tiles,
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = map_idx(x, y);
                let glyph = match self.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                    TileType::UpStairs => to_cp437('^'),
                    TileType::DownStairs => to_cp437('v'),
                };
                let color = match self.tiles[idx] {
                    TileType::Floor => DARKGREY,
                    TileType::Wall => LIGHTSLATEGRAY,
                    TileType::UpStairs => ORANGE,
                    TileType::DownStairs => ORANGE,
                };
                // if it's been revealed render it with color
                if self.visible_tiles[idx] {
                    ctx.set(x, y, color, BLACK, glyph );
                }
                else if self.revealed_tiles[idx] {
                    ctx.set(x, y, DARKBLUE, BLACK, glyph);
                }
                else {
                    continue;
                }
            }
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> TileType {
        self.tiles[map_idx(x, y)]
    }

    pub fn update_fov(&mut self, player_x: i32, player_y: i32) {
        let player_idx = map_idx(player_x, player_y);
        let player_pos = Point::new(player_x, player_y);
        let fov = field_of_view_set(player_pos, 10, self);
    
        for idx in 0..self.tiles.len() {
            let tile_pos = self.index_to_point2d(idx); // Convert idx to a Point object
            let tile_visible = fov.contains(&tile_pos); // Check if tile_pos is contained in the field of view
    
            if tile_visible {
                self.visible_tiles[idx] = true;
                self.revealed_tiles[idx] = true;
            } else {
                self.visible_tiles[idx] = false;
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
    UpStairs,
    DownStairs,
}
