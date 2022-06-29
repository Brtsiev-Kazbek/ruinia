use crate::prelude::*;

pub struct CullUnreachable {}

impl MetaLocalMapBuilder for CullUnreachable {
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder)  {
        println!("[CullUnreachable]");
        self.build(rng, build_data);
    }
}

impl CullUnreachable {
    #[allow(dead_code)]
    pub fn new() -> Box<CullUnreachable> {
        Box::new(CullUnreachable{})
    }

    fn build(&mut self, _rng : &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        // GET A STARTING_POSITION FIRST!
        let starting_pos = build_data.starting_position.as_ref().unwrap().clone();
        let start_idx = local_map_idx(
            starting_pos.0.x, 
            starting_pos.0.y
        );
        build_data.map.populate_blocked();
        let map_starts : Vec<usize> = vec![start_idx];
        let dijkstra_map = DijkstraMap::new(LOCAL_MAP_WIDTH as usize, LOCAL_MAP_HEIGHT as usize, &map_starts , &build_data.map, 1000.0);
        for (i, tile) in build_data.map.tiles.iter_mut().enumerate() {
            if *tile == LocalTileType::Floor {
                let distance_to_start = dijkstra_map.map[i];
                // We can't get to this tile - so we'll make it a wall
                if distance_to_start == std::f32::MAX {
                    *tile = LocalTileType::Wall;
                }
            }
        }
    }
}