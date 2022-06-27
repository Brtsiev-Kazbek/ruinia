use crate::prelude::*;
const NUM_LOCAL_TILES: usize = (LOCAL_MAP_WIDTH * LOCAL_MAP_HEIGHT) as usize;


// Структура глобальной карты, где каждый тайл - локальная карта
#[derive(Clone, Debug, PartialEq)]
pub struct LocalMap {
    pub tiles: Vec<LocalTileType>,
    pub blocked : Vec<bool>,
    pub tile_content : Vec<Vec<Entity>>,
    pub visible_tiles : Vec<bool>,
    pub depth: i32
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LocalTileType {
    Floor,
    Wall,
    DownStairs
}

pub fn local_map_idx(x: i32, y: i32) -> usize {
    ((y * LOCAL_MAP_WIDTH) + x) as usize
}

impl LocalMap {
    pub fn new(depth: i32) -> Self {
        Self {
            tiles: vec![LocalTileType::Floor; NUM_LOCAL_TILES],
            blocked: vec![false; NUM_LOCAL_TILES],
            tile_content : vec![Vec::new(); NUM_LOCAL_TILES],
            visible_tiles : vec![false; NUM_LOCAL_TILES],
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

    pub fn populate_blocked(&mut self) {
        for (i,tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == LocalTileType::Wall;
        }
    }

    fn is_exit_valid(&self, x:i32, y:i32) -> bool {
        if x < 1 || x > LOCAL_MAP_WIDTH-1 || y < 1 || y > LOCAL_MAP_HEIGHT-1 { return false; }
        let idx = local_map_idx(x, y);
        !self.blocked[idx]
    }

    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }
}

impl BaseMap for LocalMap {

    fn is_opaque(&self, idx:usize) -> bool {
        self.tiles[idx] == LocalTileType::Wall
    }

    fn get_pathing_distance(&self, idx1:usize, idx2:usize) -> f32 {
        let w = LOCAL_MAP_WIDTH as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        DistanceAlg::Pythagoras.distance2d(p1, p2)
    }

    fn get_available_exits(&self, idx:usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let x = idx as i32 % LOCAL_MAP_WIDTH;
        let y = idx as i32 / LOCAL_MAP_WIDTH;
        let w = LOCAL_MAP_WIDTH as usize;

        // Cardinal directions
        if self.is_exit_valid(x-1, y) { exits.push((idx-1, 1.0)) };
        if self.is_exit_valid(x+1, y) { exits.push((idx+1, 1.0)) };
        if self.is_exit_valid(x, y-1) { exits.push((idx-w, 1.0)) };
        if self.is_exit_valid(x, y+1) { exits.push((idx+w, 1.0)) };

        // Diagonals
        if self.is_exit_valid(x-1, y-1) { exits.push(((idx-w)-1, 1.45)); }
        if self.is_exit_valid(x+1, y-1) { exits.push(((idx-w)+1, 1.45)); }
        if self.is_exit_valid(x-1, y+1) { exits.push(((idx+w)-1, 1.45)); }
        if self.is_exit_valid(x+1, y+1) { exits.push(((idx+w)+1, 1.45)); }

        exits
    }
}