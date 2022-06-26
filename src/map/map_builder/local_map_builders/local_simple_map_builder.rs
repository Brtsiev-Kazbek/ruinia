use crate::prelude::*;
use bracket_lib::prelude::Rect;

pub struct SimpleLocalMapBuilder {}

impl MetaLocalMapBuilder for SimpleLocalMapBuilder {
    #[allow(dead_code)]
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        self.rooms_and_corridors(rng, build_data);
    }
}

impl SimpleLocalMapBuilder {
    #[allow(dead_code)]
    pub fn new() -> Box<SimpleLocalMapBuilder> {
        Box::new(SimpleLocalMapBuilder {  })
    }

    fn rooms_and_corridors(&mut self, rng : &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        const MAX_ROOMS : i32 = 50;
        const MIN_SIZE : i32 = 10;
        const MAX_SIZE : i32 = 20;
        let mut rooms : Vec<Rect> = Vec::new();

        let mut i = 0;
        while i < MAX_ROOMS {
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

                if !rooms.is_empty() {
                    let new_x = new_room.center().x;
                    let new_y = new_room.center().y;

                    let prev_x = rooms[i as usize -1].center().x;
                    let prev_y = rooms[i as usize -1].center().y;

                    if rng.range(0,2) == 1 {
                        apply_horizontal_tunnel(&mut build_data.map, prev_x, new_x, prev_y);
                        apply_vertical_tunnel(&mut build_data.map, prev_y, new_y, new_x);
                    } else {
                        apply_vertical_tunnel(&mut build_data.map, prev_y, new_y, prev_x);
                        apply_horizontal_tunnel(&mut build_data.map, prev_x, new_x, new_y);
                    }
                }

                rooms.push(new_room);
                i+=1;
                build_data.take_snapshot();
            }
        }
        build_data.rooms = Some(rooms);
    }
}