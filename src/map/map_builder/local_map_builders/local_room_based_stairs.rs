pub use crate::prelude::*;

pub struct RoomBasedStairs {}

impl MetaLocalMapBuilder for RoomBasedStairs {
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder)  {
        self.build(rng, build_data);
    }
}

impl RoomBasedStairs {
    #[allow(dead_code)]
    pub fn new() -> Box<RoomBasedStairs> {
        Box::new(RoomBasedStairs{})
    }

    fn build(&mut self, _rng : &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        if let Some(rooms) = &build_data.rooms {
            let stairs_position = rooms[rooms.len()-1].center();
            let stairs_idx = local_map_idx(stairs_position.x, stairs_position.y);
            build_data.map.tiles[stairs_idx] = LocalTileType::DownStairs;
            build_data.take_snapshot();
        } else {
            panic!("Room Based Stairs only works after rooms have been created");
        }
    }
}