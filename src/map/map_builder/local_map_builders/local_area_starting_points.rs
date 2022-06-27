use crate::prelude::*;

#[allow(dead_code)]
pub enum XStart { LEFT, CENTER, RIGHT }

#[allow(dead_code)]
pub enum YStart { TOP, CENTER, BOTTOM }

pub struct AreaStartingPosition {
    x : XStart, 
    y : YStart
}

impl MetaLocalMapBuilder for AreaStartingPosition {
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder)  {
        self.build(rng, build_data);
    }
}

impl AreaStartingPosition {
    #[allow(dead_code)]
    pub fn new(x : XStart, y : YStart) -> Box<AreaStartingPosition> {
        Box::new(AreaStartingPosition{
            x, y
        })
    }

    fn build(&mut self, _rng : &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        let seed_x;
        let seed_y;

        match self.x {
            XStart::LEFT => seed_x = 1,
            XStart::CENTER => seed_x = LOCAL_MAP_WIDTH / 2,
            XStart::RIGHT => seed_x = LOCAL_MAP_WIDTH - 2
        }

        match self.y {
            YStart::TOP => seed_y = 1,
            YStart::CENTER => seed_y = LOCAL_MAP_HEIGHT / 2,
            YStart::BOTTOM => seed_y = LOCAL_MAP_HEIGHT - 2
        }

        let mut available_floors : Vec<(usize, f32)> = Vec::new();
        for (idx, tiletype) in build_data.map.tiles.iter().enumerate() {
            if *tiletype == LocalTileType::Floor {
                available_floors.push(
                    (
                        idx,
                        DistanceAlg::PythagorasSquared.distance2d(
                            Point::new(idx as i32 % LOCAL_MAP_WIDTH, idx as i32 / LOCAL_MAP_WIDTH),
                            Point::new(seed_x, seed_y)
                        )
                    )
                );
            }
        }
        if available_floors.is_empty() {
            panic!("No valid floors to start on");
        }

        available_floors.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());

        let start_x = available_floors[0].0 as i32 % LOCAL_MAP_WIDTH;
        let start_y = available_floors[0].0 as i32 / LOCAL_MAP_WIDTH;

        build_data.starting_position = Some(PointL(Point{x : start_x, y: start_y}));
    }
}