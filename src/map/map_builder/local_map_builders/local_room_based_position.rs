use crate::prelude::*;

pub struct RoomBasedStartingPosition {}

impl MetaLocalMapBuilder for RoomBasedStartingPosition {
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder)  {
        self.build(rng, build_data);
    }
}

impl RoomBasedStartingPosition {
    #[allow(dead_code)]
    pub fn new() -> Box<RoomBasedStartingPosition> {
        Box::new(RoomBasedStartingPosition{})
    }

    fn build(&mut self, _rng : &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        if let Some(rooms) = &build_data.rooms {
            let start_pos = rooms[0].center();
            build_data.starting_position = Some(PointL(Point{ x: start_pos.x, y: start_pos.y }));
        } else {
            panic!("Room Based Staring Position only works after rooms have been created");
        }
    }
}