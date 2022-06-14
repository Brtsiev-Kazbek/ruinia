use crate::prelude::*;

const NUM_GLOBAL_TILES: usize = (GLOBAL_MAP_WIDTH * GLOBAL_MAP_HEIGHT) as usize;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GlobalTileType {
    Forest,
    Mountains,
    Plain,
    Water,
    MonsterFortress,
    Town,
    Temple,
}

// Структура глобальной карты, где каждый тайл - локальная карта
#[derive(Clone, Debug, PartialEq)]
pub struct GlobalMap {
    pub tiles: Vec<GlobalTileType>,
}

pub fn global_map_idx(x: i32, y: i32) -> usize {
    ((y * GLOBAL_MAP_WIDTH) + x) as usize
}

impl GlobalMap {
    pub fn new() -> Self {
        Self {
            tiles: vec![GlobalTileType::Water; NUM_GLOBAL_TILES],
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < GLOBAL_MAP_WIDTH && point.y >= 0 && point.y < GLOBAL_MAP_HEIGHT
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(global_map_idx(point.x, point.y))
        }
    }
}
