use std::cmp::{min, max};

use crate::prelude::*;
use bracket_lib::prelude::Rect;


#[derive(PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum Symmetry { None, Horizontal, Vertical, Both }


pub fn paint(map: &mut LocalMap, mode: Symmetry, brush_size: i32, x: i32, y:i32) {
    match mode {
        Symmetry::None => apply_paint(map, brush_size, x, y),
        Symmetry::Horizontal => {
            let center_x = LOCAL_MAP_WIDTH / 2;
            if x == center_x {
                apply_paint(map, brush_size, x, y);
            } else {
                let dist_x = i32::abs(center_x - x);
                apply_paint(map, brush_size, center_x + dist_x, y);
                apply_paint(map, brush_size, center_x - dist_x, y);
            }
        }
        Symmetry::Vertical => {
            let center_y = LOCAL_MAP_HEIGHT / 2;
            if y == center_y {
                apply_paint(map, brush_size, x, y);
            } else {
                let dist_y = i32::abs(center_y - y);
                apply_paint(map, brush_size, x, center_y + dist_y);
                apply_paint(map, brush_size, x, center_y - dist_y);
            }
        }
        Symmetry::Both => {
            let center_x = LOCAL_MAP_WIDTH / 2;
            let center_y = LOCAL_MAP_HEIGHT / 2;
            if x == center_x && y == center_y {
                apply_paint(map, brush_size, x, y);
            } else {
                let dist_x = i32::abs(center_x - x);
                apply_paint(map, brush_size, center_x + dist_x, y);
                apply_paint(map, brush_size, center_x - dist_x, y);
                let dist_y = i32::abs(center_y - y);
                apply_paint(map, brush_size, x, center_y + dist_y);
                apply_paint(map, brush_size, x, center_y - dist_y);
            }
        }
    }
}

fn apply_paint(map: &mut LocalMap, brush_size: i32, x: i32, y: i32) {
    match brush_size {
        1 => {
            let digger_idx = local_map_idx(x, y);
            map.tiles[digger_idx] = LocalTileType::Floor;
        }

        _ => {
            let half_brush_size = brush_size / 2;
            for brush_y in y-half_brush_size .. y+half_brush_size {
                for brush_x in x-half_brush_size .. x+half_brush_size {
                    if brush_x > 1 && brush_x < LOCAL_MAP_WIDTH-1 && brush_y > 1 && brush_y < LOCAL_MAP_HEIGHT-1 {
                        let idx = local_map_idx(brush_x, brush_y);
                        map.tiles[idx] = LocalTileType::Floor;
                    }
                }
            }
        }
    }
}


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

pub fn apply_horizontal_tunnel(map : &mut LocalMap, x1:i32, x2:i32, y:i32) -> Vec<usize> {
    let mut corridor = Vec::new();
    for x in min(x1,x2) ..= max(x1,x2) {
        let idx = local_map_idx(x, y);
        if idx > 0 && idx < LOCAL_MAP_WIDTH as usize * LOCAL_MAP_HEIGHT as usize && map.tiles[idx as usize] != LocalTileType::Floor {
            map.tiles[idx as usize] = LocalTileType::Floor;
            corridor.push(idx as usize);
        }
    }
    corridor
}

pub fn apply_vertical_tunnel(map : &mut LocalMap, y1:i32, y2:i32, x:i32) -> Vec<usize> {
    let mut corridor = Vec::new();
    for y in min(y1,y2) ..= max(y1,y2) {
        let idx = local_map_idx(x, y);
        if idx > 0 && idx < LOCAL_MAP_WIDTH as usize * LOCAL_MAP_HEIGHT as usize && map.tiles[idx as usize] != LocalTileType::Floor {
            corridor.push(idx);
            map.tiles[idx as usize] = LocalTileType::Floor;
        }
    }
    corridor
}

pub fn draw_corridor(map: &mut LocalMap, x1:i32, y1:i32, x2:i32, y2:i32) -> Vec<usize> {
    let mut corridor = Vec::new();
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
        if map.tiles[idx] != LocalTileType::Floor {
            corridor.push(idx);
            map.tiles[idx] = LocalTileType::Floor;
        }
    }

    corridor
}