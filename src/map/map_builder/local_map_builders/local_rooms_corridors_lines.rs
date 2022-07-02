use crate::prelude::*;
use bracket_lib::prelude::Rect;

use std::collections::HashSet;

pub struct StraightLineCorridors {}

impl MetaLocalMapBuilder for StraightLineCorridors {
    #[allow(dead_code)]
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        self.corridors(rng, build_data);
    }
}

impl StraightLineCorridors {
    #[allow(dead_code)]
    pub fn new() -> Box<StraightLineCorridors> {
        Box::new(StraightLineCorridors{})
    }

    fn corridors(&mut self, _rng : &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        let rooms : Vec<Rect>;
        if let Some(rooms_builder) = &build_data.rooms {
            rooms = rooms_builder.clone();
        } else {
            panic!("Straight Line Corridors require a builder with room structures");
        }
    
        let mut connected : HashSet<usize> = HashSet::new();
        let mut corridors : Vec<Vec<usize>> = Vec::new();
        for (i,room) in rooms.iter().enumerate() {
            let mut room_distance : Vec<(usize, f32)> = Vec::new();
            let room_center = room.center();
            let room_center_pt = Point::new(room_center.x, room_center.y);
            for (j,other_room) in rooms.iter().enumerate() {
                if i != j && !connected.contains(&j) {
                    let other_center = other_room.center();
                    let other_center_pt = Point::new(other_center.x, other_center.y);
                    let distance = DistanceAlg::Pythagoras.distance2d(
                        room_center_pt,
                        other_center_pt
                    );
                    room_distance.push((j, distance));
                }
            }
    
            if !room_distance.is_empty() {
                room_distance.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap() );
                let dest_center = rooms[room_distance[0].0].center();
                let line = line2d(
                    LineAlg::Bresenham, 
                    room_center_pt, 
                    Point::new(dest_center.x, dest_center.y)
                );
                let mut corridor = Vec::new();
                for cell in line.iter() {
                    let idx = local_map_idx(cell.x, cell.y);
                    if build_data.map.tiles[idx] != LocalTileType::Floor {
                        build_data.map.tiles[idx] = LocalTileType::Floor;
                        corridor.push(idx);
                    }
                }
                corridors.push(corridor);
                connected.insert(i);
                build_data.take_snapshot();
            }
        }
        build_data.corridors = Some(corridors);
    }
}