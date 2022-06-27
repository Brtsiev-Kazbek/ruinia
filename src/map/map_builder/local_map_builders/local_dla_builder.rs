use crate::prelude::*;

#[derive(PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum DLAAlgorithm { WalkInwards, WalkOutwards, CentralAttractor }

pub struct DLABuilder {
    algorithm : DLAAlgorithm,
    brush_size: i32,
    symmetry: Symmetry,
    floor_percent: f32,
}


impl MetaLocalMapBuilder for DLABuilder {
    #[allow(dead_code)]
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        self.build(rng, build_data);
    }
}

impl DLABuilder {
    #[allow(dead_code)]
    pub fn new() -> Box<DLABuilder> {
        Box::new(DLABuilder{
            algorithm: DLAAlgorithm::WalkInwards,
            brush_size: 2,
            symmetry: Symmetry::None,
            floor_percent: 0.25,
        })
    }

    #[allow(dead_code)]
    pub fn walk_inwards() -> Box<DLABuilder> {
        Box::new(DLABuilder{
            algorithm: DLAAlgorithm::WalkInwards,
            brush_size: 1,
            symmetry: Symmetry::None,
            floor_percent: 0.25,
        })
    }

    #[allow(dead_code)]
    pub fn walk_outwards() -> Box<DLABuilder> {
        Box::new(DLABuilder{
            algorithm: DLAAlgorithm::WalkOutwards,
            brush_size: 2,
            symmetry: Symmetry::None,
            floor_percent: 0.25,
        })
    }

    #[allow(dead_code)]
    pub fn central_attractor() -> Box<DLABuilder> {
        Box::new(DLABuilder{
            algorithm: DLAAlgorithm::CentralAttractor,
            brush_size: 2,
            symmetry: Symmetry::None,
            floor_percent: 0.25,
        })
    }

    #[allow(dead_code)]
    pub fn insectoid() -> Box<DLABuilder> {
        Box::new(DLABuilder{
            algorithm: DLAAlgorithm::CentralAttractor,
            brush_size: 2,
            symmetry: Symmetry::Horizontal,
            floor_percent: 0.25,
        })
    }

    #[allow(clippy::map_entry)]
    fn build(&mut self, rng : &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        // Carve a starting seed
        let starting_position = PointL(Point{ x: LOCAL_MAP_WIDTH/2, y : LOCAL_MAP_HEIGHT/2 });
        let start_idx = local_map_idx(starting_position.0.x, starting_position.0.y);
        build_data.take_snapshot();
        build_data.map.tiles[start_idx] = LocalTileType::Floor;
        build_data.map.tiles[start_idx-1] = LocalTileType::Floor;
        build_data.map.tiles[start_idx+1] = LocalTileType::Floor;
        build_data.map.tiles[start_idx-LOCAL_MAP_WIDTH as usize] = LocalTileType::Floor;
        build_data.map.tiles[start_idx+LOCAL_MAP_WIDTH as usize] = LocalTileType::Floor;

        // Random walker
        let total_tiles = LOCAL_MAP_WIDTH * LOCAL_MAP_HEIGHT;
        let desired_floor_tiles = (self.floor_percent * total_tiles as f32) as usize;
        let mut floor_tile_count = build_data.map.tiles.iter().filter(|a| **a == LocalTileType::Floor).count();
        while floor_tile_count  < desired_floor_tiles {

            match self.algorithm {
                DLAAlgorithm::WalkInwards => {
                    let mut digger_x = rng.roll_dice(1, LOCAL_MAP_WIDTH - 3) + 1;
                    let mut digger_y = rng.roll_dice(1, LOCAL_MAP_HEIGHT - 3) + 1;
                    let mut prev_x = digger_x;
                    let mut prev_y = digger_y;
                    let mut digger_idx = local_map_idx(digger_x, digger_y);
                    while build_data.map.tiles[digger_idx] == LocalTileType::Wall {
                        prev_x = digger_x;
                        prev_y = digger_y;
                        let stagger_direction = rng.roll_dice(1, 4);
                        match stagger_direction {
                            1 => { if digger_x > 2 { digger_x -= 1; } }
                            2 => { if digger_x < LOCAL_MAP_WIDTH-2 { digger_x += 1; } }
                            3 => { if digger_y > 2 { digger_y -=1; } }
                            _ => { if digger_y < LOCAL_MAP_HEIGHT-2 { digger_y += 1; } }
                        }
                        digger_idx = local_map_idx(digger_x, digger_y);
                    }
                    paint(&mut build_data.map, self.symmetry, self.brush_size, prev_x, prev_y);
                }

                DLAAlgorithm::WalkOutwards => {
                    let mut digger_x = starting_position.0.x;
                    let mut digger_y = starting_position.0.y;
                    let mut digger_idx = local_map_idx(digger_x, digger_y);
                    while build_data.map.tiles[digger_idx] == LocalTileType::Floor {
                        let stagger_direction = rng.roll_dice(1, 4);
                        match stagger_direction {
                            1 => { if digger_x > 2 { digger_x -= 1; } }
                            2 => { if digger_x < LOCAL_MAP_WIDTH-2 { digger_x += 1; } }
                            3 => { if digger_y > 2 { digger_y -=1; } }
                            _ => { if digger_y < LOCAL_MAP_HEIGHT-2 { digger_y += 1; } }
                        }
                        digger_idx = local_map_idx(digger_x, digger_y);
                    }
                    paint(&mut build_data.map, self.symmetry, self.brush_size, digger_x, digger_y);
                }

                DLAAlgorithm::CentralAttractor => {
                    let mut digger_x = rng.roll_dice(1, LOCAL_MAP_WIDTH - 3) + 1;
                    let mut digger_y = rng.roll_dice(1, LOCAL_MAP_HEIGHT - 3) + 1;
                    let mut prev_x = digger_x;
                    let mut prev_y = digger_y;
                    let mut digger_idx = local_map_idx(digger_x, digger_y);

                    let mut path = line2d(
                        LineAlg::Bresenham, 
                        Point::new( digger_x, digger_y ), 
                        Point::new( starting_position.0.x, starting_position.0.y )
                    );

                    while build_data.map.tiles[digger_idx] == LocalTileType::Wall && !path.is_empty() {
                        prev_x = digger_x;
                        prev_y = digger_y;
                        digger_x = path[0].x;
                        digger_y = path[0].y;
                        path.remove(0);
                        digger_idx = local_map_idx(digger_x, digger_y);
                    }
                    paint(&mut build_data.map, self.symmetry, self.brush_size, prev_x, prev_y);
                }
            }

            build_data.take_snapshot();

            floor_tile_count = build_data.map.tiles.iter().filter(|a| **a == LocalTileType::Floor).count();
        }
    }    
}