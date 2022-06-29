use crate::prelude::*;
use bracket_lib::prelude::Rect;


pub struct RoomCornerRounder {}

impl MetaLocalMapBuilder for RoomCornerRounder {
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder)  {
        self.build(rng, build_data);
    }
}

impl RoomCornerRounder {
    #[allow(dead_code)]
    pub fn new() -> Box<RoomCornerRounder> {
        Box::new(RoomCornerRounder{})
    }

    fn fill_if_corner(&mut self, x: i32, y: i32, build_data : &mut LocalMapBuilder) {
        let w = LOCAL_MAP_WIDTH;
        let h = LOCAL_MAP_HEIGHT;
        let idx = local_map_idx(x, y);
        let mut neighbor_walls = 0;
        if x > 0 && build_data.map.tiles[idx-1] == LocalTileType::Wall { neighbor_walls += 1; }
        if y > 0 && build_data.map.tiles[idx-w as usize] == LocalTileType::Wall { neighbor_walls += 1; }
        if x < w-2 && build_data.map.tiles[idx+1] == LocalTileType::Wall { neighbor_walls += 1; }
        if y < h-2 && build_data.map.tiles[idx+w as usize] == LocalTileType::Wall { neighbor_walls += 1; }

        if neighbor_walls == 2 {
            build_data.map.tiles[idx] = LocalTileType::Wall;
        }
    }

    fn build(&mut self, _rng : &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        let rooms : Vec<Rect>;
        if let Some(rooms_builder) = &build_data.rooms {
            rooms = rooms_builder.clone();
        } else {
            panic!("Room Rounding require a builder with room structures");
        }

        for room in rooms.iter() {
            self.fill_if_corner(room.x1+1, room.y1+1, build_data);
            self.fill_if_corner(room.x2, room.y1+1, build_data);
            self.fill_if_corner(room.x1+1, room.y2, build_data);
            self.fill_if_corner(room.x2, room.y2, build_data);

            build_data.take_snapshot();
        }
    }
}