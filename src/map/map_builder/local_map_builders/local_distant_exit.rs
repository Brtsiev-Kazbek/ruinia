use crate::prelude::*;

pub struct DistantExit {}

impl MetaLocalMapBuilder for DistantExit {
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder)  {
        self.build(rng, build_data);
    }
}

impl DistantExit {
    #[allow(dead_code)]
    pub fn new() -> Box<DistantExit> {
        Box::new(DistantExit{})
    }

    fn build(&mut self, _rng : &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        let starting_pos = build_data.starting_position.as_ref().unwrap().clone();
        let start_idx = local_map_idx(
            starting_pos.0.x, 
            starting_pos.0.y
        );
        build_data.map.populate_blocked();
        let map_starts : Vec<usize> = vec![start_idx];
        let dijkstra_map = DijkstraMap::new(LOCAL_MAP_WIDTH as usize, LOCAL_MAP_HEIGHT as usize, &map_starts , &build_data.map, 1000.0);
        let mut exit_tile = (0, 0.0f32);
        for (i, tile) in build_data.map.tiles.iter_mut().enumerate() {
            if *tile == LocalTileType::Floor {
                let distance_to_start = dijkstra_map.map[i];
                if distance_to_start != std::f32::MAX {
                    // If it is further away than our current exit candidate, move the exit
                    if distance_to_start > exit_tile.1 {
                        exit_tile.0 = i;
                        exit_tile.1 = distance_to_start;
                    }
                }
            }
        }

        // Place a staircase
        let stairs_idx = exit_tile.0;
        build_data.map.tiles[stairs_idx] = LocalTileType::DownStairs;
        build_data.take_snapshot();
    }
}