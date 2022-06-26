use crate::prelude::*;
use bracket_lib::prelude::Rect;

pub struct BspLocalMapBuilder {
    rects: Vec<Rect>
}

// impl InitialLocalMapBuilder for BspLocalMapBuilder {
//     #[allow(dead_code)]
//     fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
//         self.build(rng, build_data);
//     }
// }

impl MetaLocalMapBuilder for BspLocalMapBuilder {
    #[allow(dead_code)]
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        self.build(rng, build_data);
    }
}

impl BspLocalMapBuilder {
    #[allow(dead_code)]
    pub fn new() -> Box<BspLocalMapBuilder> {
        Box::new(BspLocalMapBuilder {
            rects: Vec::new(),
        })
    }

    fn build(&mut self, rng : &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        let mut rooms : Vec<Rect> = Vec::new();
        self.rects.clear();
        self.rects.push( Rect::with_exact(2, 2, LOCAL_MAP_WIDTH-5, LOCAL_MAP_HEIGHT-5) );
        let first_room = self.rects[0];
        self.add_subrects(first_room); // Divide the first room

        // Up to 240 times, we get a random rectangle and divide it. If its possible to squeeze a
        // room in there, we place it and add it to the rooms list.
        let mut n_rooms = 0;
        while n_rooms < 240 {
            let rect = self.get_random_rect(rng);
            let candidate = self.get_random_sub_rect(rect, rng);

            if self.is_possible(candidate, &build_data.map) {
                apply_room_to_map(&mut build_data.map, &candidate);
                rooms.push(candidate);
                self.add_subrects(rect);
                build_data.take_snapshot();
            }

            n_rooms += 1;
        }

        // Now we sort the rooms
        rooms.sort_by(|a,b| a.x1.cmp(&b.x1) );

        // Now we want corridors
        for i in 0..rooms.len()-1 {
            let room = rooms[i];
            let next_room = rooms[i+1];
            let start_x = room.x1 + (rng.roll_dice(1, i32::abs(room.x1 - room.x2))-1);
            let start_y = room.y1 + (rng.roll_dice(1, i32::abs(room.y1 - room.y2))-1);
            let end_x = next_room.x1 + (rng.roll_dice(1, i32::abs(next_room.x1 - next_room.x2))-1);
            let end_y = next_room.y1 + (rng.roll_dice(1, i32::abs(next_room.y1 - next_room.y2))-1);
            draw_corridor(&mut build_data.map, start_x, start_y, end_x, end_y);
            build_data.take_snapshot();
        }
        build_data.rooms = Some(rooms);
    }

    fn add_subrects(&mut self, rect : Rect) {
        let width = i32::abs(rect.x1 - rect.x2);
        let height = i32::abs(rect.y1 - rect.y2);
        let half_width = i32::max(width / 2, 1);
        let half_height = i32::max(height / 2, 1);

        self.rects.push(Rect::with_exact( rect.x1, rect.y1, half_width, half_height ));
        self.rects.push(Rect::with_exact( rect.x1, rect.y1 + half_height, half_width, half_height ));
        self.rects.push(Rect::with_exact( rect.x1 + half_width, rect.y1, half_width, half_height ));
        self.rects.push(Rect::with_exact( rect.x1 + half_width, rect.y1 + half_height, half_width, half_height ));
    }

    fn get_random_rect(&mut self, rng : &mut RandomNumberGenerator) -> Rect {
        if self.rects.len() == 1 { return self.rects[0]; }
        let idx = (rng.roll_dice(1, self.rects.len() as i32)-1) as usize;
        self.rects[idx]
    }

    fn get_random_sub_rect(&self, rect : Rect, rng : &mut RandomNumberGenerator) -> Rect {
        let mut result = rect;
        let rect_width = i32::abs(rect.x1 - rect.x2);
        let rect_height = i32::abs(rect.y1 - rect.y2);

        let w = i32::max(3, rng.roll_dice(1, i32::min(rect_width, 10))-1) + 1;
        let h = i32::max(3, rng.roll_dice(1, i32::min(rect_height, 10))-1) + 1;

        result.x1 += rng.roll_dice(1, 6)-1;
        result.y1 += rng.roll_dice(1, 6)-1;
        result.x2 = result.x1 + w;
        result.y2 = result.y1 + h;

        result
    }

    fn is_possible(&self, rect : Rect, map : &LocalMap) -> bool {
        let mut expanded = rect;
        expanded.x1 -= 2;
        expanded.x2 += 2;
        expanded.y1 -= 2;
        expanded.y2 += 2;

        let mut can_build = true;

        for y in expanded.y1 ..= expanded.y2 {
            for x in expanded.x1 ..= expanded.x2 {
                if x > LOCAL_MAP_WIDTH-2 { can_build = false; }
                if y > LOCAL_MAP_HEIGHT-2 { can_build = false; }
                if x < 1 { can_build = false; }
                if y < 1 { can_build = false; }
                if can_build {
                    let idx = local_map_idx(x, y);
                    if map.tiles[idx] != LocalTileType::Wall { 
                        can_build = false; 
                    }
                }
            }
        }
        can_build
    }
}