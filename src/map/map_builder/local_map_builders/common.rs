use std::cmp::{min, max};

use crate::prelude::*;
use bracket_lib::prelude::Rect;


pub fn apply_room_to_map(map : &mut LocalMap, room : &Rect) {
    for y in room.y1 +1 ..= room.y2 {
        for x in room.x1 + 1 ..= room.x2 {
            let idx = local_map_idx(x, y);
            if idx > 0 && idx < ((LOCAL_MAP_WIDTH * LOCAL_MAP_HEIGHT)-1) as usize {
                map.tiles[idx] = LocalTileType::Floor;
            }
        }
    }
}

pub fn draw_corridor(map: &mut LocalMap, x1:i32, y1:i32, x2:i32, y2:i32) {
    let mut x = x1;
    let mut y = y1;

    while x != x2 || y != y2 {
        if x < x2 {
            x += 1;
        } else if x > x2 {
            x -= 1;
        } else if y < y2 {
            y += 1;
        } else if y > y2 {
            y -= 1;
        }

        let idx = local_map_idx(x, y);
        map.tiles[idx] = LocalTileType::Floor;
    }
}

pub fn apply_horizontal_tunnel(map : &mut LocalMap, x1:i32, x2:i32, y:i32) {
    for x in min(x1,x2) ..= max(x1,x2) {
        let idx = local_map_idx(x, y);
        if idx > 0 && idx < LOCAL_MAP_WIDTH as usize * LOCAL_MAP_HEIGHT as usize {
            map.tiles[idx as usize] = LocalTileType::Floor;
        }
    }
}

pub fn apply_vertical_tunnel(map : &mut LocalMap, y1:i32, y2:i32, x:i32) {
    for y in min(y1,y2) ..= max(y1,y2) {
        let idx = local_map_idx(x, y);
        if idx > 0 && idx < LOCAL_MAP_WIDTH as usize * LOCAL_MAP_HEIGHT as usize {
            map.tiles[idx as usize] = LocalTileType::Floor;
        }
    }
}