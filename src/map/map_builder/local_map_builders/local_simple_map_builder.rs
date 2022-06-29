use crate::prelude::*;
use bracket_lib::prelude::Rect;

pub struct SimpleLocalMapBuilder {}

impl MetaLocalMapBuilder for SimpleLocalMapBuilder {
    #[allow(dead_code)]
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        self.build_rooms(rng, build_data);
    }
}

impl SimpleLocalMapBuilder {
    #[allow(dead_code)]
    pub fn new() -> Box<SimpleLocalMapBuilder> {
        Box::new(SimpleLocalMapBuilder{})
    }

    fn build_rooms(&mut self, rng : &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        const MAX_ROOMS : i32 = 30;
        const MIN_SIZE : i32 = 6;
        const MAX_SIZE : i32 = 10;
        let mut rooms : Vec<Rect> = Vec::new();

        for i in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, LOCAL_MAP_WIDTH - w - 1) - 1;
            let y = rng.roll_dice(1, LOCAL_MAP_HEIGHT - h - 1) - 1;
            let new_room = Rect::with_size(x, y, w, h);
            let mut ok = true;
            for other_room in rooms.iter() {
                if new_room.intersect(other_room) { ok = false }
            }
            if ok {
                apply_room_to_map(&mut build_data.map, &new_room);
                build_data.take_snapshot();

                rooms.push(new_room);
                build_data.take_snapshot();
            }
        }
        build_data.rooms = Some(rooms);
    }
}