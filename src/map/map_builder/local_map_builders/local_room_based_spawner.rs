use crate::prelude::*;

pub struct RoomBasedSpawner {}

impl MetaLocalMapBuilder for RoomBasedSpawner {
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder)  {
        self.build(rng, build_data);
    }
}

impl RoomBasedSpawner {
    #[allow(dead_code)]
    pub fn new() -> Box<RoomBasedSpawner> {
        Box::new(RoomBasedSpawner{})
    }

    fn build(&mut self, rng : &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        if let Some(rooms) = &build_data.rooms {
            for room in rooms.iter().skip(1) {
                // TODO: Implement this
                // spawner::spawn_room(&build_data.map, rng, room, build_data.map.depth, &mut build_data.spawn_list);
            }
        } else {
            panic!("Room Based Spawning only works after rooms have been created");
        }
    }
}