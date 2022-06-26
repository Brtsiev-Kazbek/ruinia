use crate::prelude::*;
const NUM_LOCAL_TILES: usize = (LOCAL_MAP_WIDTH * LOCAL_MAP_HEIGHT) as usize;


// Структура глобальной карты, где каждый тайл - локальная карта
#[derive(Clone, Debug, PartialEq)]
pub struct LocalMap {
    pub tiles: Vec<LocalTileType>,
    pub depth: i32
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LocalTileType {
    Floor,
    Wall
}

pub fn local_map_idx(x: i32, y: i32) -> usize {
    ((y * LOCAL_MAP_WIDTH) + x) as usize
}

impl LocalMap {
    pub fn new(depth: i32) -> Self {
        Self {
            tiles: vec![LocalTileType::Floor; NUM_LOCAL_TILES],
            depth: depth
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < LOCAL_MAP_WIDTH && point.y >= 0 && point.y < LOCAL_MAP_HEIGHT
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(local_map_idx(point.x, point.y))
        }
    }
}