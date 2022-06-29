use crate::prelude::*;
use bracket_lib::prelude::Rect;

pub enum RoomSort { LEFTMOST, RIGHTMOST, TOPMOST, BOTTOMMOST, CENTRAL }

pub struct RoomSorter {
    sort_by : RoomSort
}

impl MetaLocalMapBuilder for RoomSorter {
    #[allow(dead_code)]
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        self.sorter(rng, build_data);
    }
}

impl RoomSorter {
    #[allow(dead_code)]
    pub fn new(sort_by : RoomSort) -> Box<RoomSorter> {
        Box::new(RoomSorter{ sort_by })
    }

    fn sorter(&mut self, _rng : &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        match self.sort_by {
            RoomSort::LEFTMOST => build_data.rooms.as_mut().unwrap().sort_by(|a,b| a.x1.cmp(&b.x1) ),
            RoomSort::RIGHTMOST => build_data.rooms.as_mut().unwrap().sort_by(|a,b| b.x2.cmp(&a.x2) ),
            RoomSort::TOPMOST => build_data.rooms.as_mut().unwrap().sort_by(|a,b| a.y1.cmp(&b.y1) ),
            RoomSort::BOTTOMMOST => build_data.rooms.as_mut().unwrap().sort_by(|a,b| b.y2.cmp(&a.y2) ),
            RoomSort::CENTRAL => {
                let map_center = Point::new( LOCAL_MAP_WIDTH / 2, LOCAL_MAP_HEIGHT / 2 );
                let center_sort = |a : &Rect, b : &Rect| {
                    let a_center = a.center();
                    let a_center_pt = Point::new(a_center.x, a_center.y);
                    let b_center = b.center();
                    let b_center_pt = Point::new(b_center.x, b_center.y);
                    let distance_a = DistanceAlg::Pythagoras.distance2d(a_center_pt, map_center);
                    let distance_b = DistanceAlg::Pythagoras.distance2d(b_center_pt, map_center);
                    distance_a.partial_cmp(&distance_b).unwrap()
                };
    
                build_data.rooms.as_mut().unwrap().sort_by(center_sort);
            }
        }
    }
}