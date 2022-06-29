use crate::prelude::*;
use bracket_lib::prelude::Rect;

pub struct DoglegCorridors {}

impl MetaLocalMapBuilder for DoglegCorridors {
    #[allow(dead_code)]
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        self.corridors(rng, build_data);
    }
}

impl DoglegCorridors {
    #[allow(dead_code)]
    pub fn new() -> Box<DoglegCorridors> {
        Box::new(DoglegCorridors{})
    }

    fn corridors(&mut self, rng : &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        let rooms : Vec<Rect>;
        if let Some(rooms_builder) = &build_data.rooms {
            rooms = rooms_builder.clone();
        } else {
            panic!("Dogleg Corridors require a builder with room structures");
        }

        for (i,room) in rooms.iter().enumerate() {
            if i > 0 {
                let new_coords = room.center();
                let prev_coords = rooms[i as usize -1].center();
                if rng.range(0,2) == 1 {
                    apply_horizontal_tunnel(&mut build_data.map, prev_coords.x, new_coords.x, prev_coords.y);
                    apply_vertical_tunnel(&mut build_data.map, prev_coords.y, new_coords.y, new_coords.x);
                } else {
                    apply_vertical_tunnel(&mut build_data.map, prev_coords.y, new_coords.y, prev_coords.x);
                    apply_horizontal_tunnel(&mut build_data.map, prev_coords.x, new_coords.x, new_coords.y);
                }
                build_data.take_snapshot();
            }
        }
    }
}