// progen.rs

use bracket_lib::prelude::*;
use super::map::{Map, TileType, SCREEN_WIDTH, SCREEN_HEIGHT};

pub fn generate_dungeon(map: &mut Map) {
    map.tiles.iter_mut().for_each(|tile| *tile = TileType::Wall);

    let mut rng = RandomNumberGenerator::new();

    // Create rooms
        // Create rooms
    for _ in 0..20 {
        let width = rng.range(2, 10);
        let height = rng.range(2, 10);
        let x = rng.range(1, SCREEN_WIDTH - width - 1);
        let y = rng.range(1, SCREEN_HEIGHT - height - 1);
        let room = create_room(map, x as i32, y as i32, width as i32, height as i32);
        map.rooms.push(room);
    }

    // Create corridors
    for i in 0..map.rooms.len() - 1 {
        let (x1, y1) = calculate_center(&map.rooms[i]);
        let (x2, y2) = calculate_center(&map.rooms[i + 1]);
        create_corridor(map, x1, y1, x2, y2);
    }

    // Add stairs
    add_stairs(map);

}

fn create_room(map: &mut Map, x: i32, y: i32, width: i32, height: i32) -> Rect {
    for i in x..x + width {
        for j in y..y + height {
            if i == x || i == x + width - 1 || j == y || j == y + height - 1 {
                continue;
            }
            let idx = map_idx(i, j);
            map.tiles[idx] = TileType::Floor;
        }
    }
    let room = Rect {
        x1: x,
        y1: y,
        x2: x + width,
        y2: y + height,
    };
    map.rooms.push(room);
    room // Return the created room
}



fn create_corridor(map: &mut Map, x1: i32, y1: i32, x2: i32, y2: i32) {
    let mut x = x1;
    let mut y = y1;
    while x != x2 {
        let idx = map_idx(x, y);
        map.tiles[idx] = TileType::Floor;
        if x < x2 {
            x += 1;
        } else {
            x -= 1;
        }
    }
    while y != y2 {
        let idx = map_idx(x, y);
        map.tiles[idx] = TileType::Floor;
        if y < y2 {
            y += 1;
        } else {
            y -= 1;
        }
    }
}

pub fn add_stairs(map: &mut Map) {
    let mut rng = RandomNumberGenerator::new();

    let idx1 = rng.range(0, map.rooms.len());
    let idx2 = rng.range(0, map.rooms.len());

    let (x1, y1) = calculate_center(&map.rooms[idx1]);
    let (x2, y2) = calculate_center(&map.rooms[idx2]);

    let idx1_center = map_idx(x1, y1);
    let idx2_center = map_idx(x2, y2);

    map.tiles[idx1_center] = TileType::UpStairs;
    map.tiles[idx2_center] = TileType::DownStairs;
}


fn map_idx(x: i32, y: i32) -> usize {
    (y as usize * SCREEN_WIDTH) + x as usize
}

fn calculate_center(rect: &Rect) -> (i32, i32) {
    let x = rect.x1 + (rect.x2 - rect.x1) / 2;
    let y = rect.y1 + (rect.y2 - rect.y1) / 2;
    (x, y)
}