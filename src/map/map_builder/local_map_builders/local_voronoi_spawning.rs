use std::collections::HashMap;

use crate::prelude::*;

pub struct VoronoiSpawning {}

impl MetaLocalMapBuilder for VoronoiSpawning {
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder)  {
        println!("[VoronoiSpawning]");
        self.build(rng, build_data);
    }
}

impl VoronoiSpawning {
    #[allow(dead_code)]
    pub fn new() -> Box<VoronoiSpawning> {
        Box::new(VoronoiSpawning{})
    }

    #[allow(clippy::map_entry)]
    fn build(&mut self, rng : &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        let mut noise_areas : HashMap<i32, Vec<usize>> = HashMap::new();
        let mut noise = FastNoise::seeded(rng.roll_dice(1, 65536) as u64);
        noise.set_noise_type(NoiseType::Cellular);
        noise.set_frequency(0.08);
        noise.set_cellular_distance_function(CellularDistanceFunction::Manhattan);

        for y in 1 .. LOCAL_MAP_HEIGHT-1 {
            for x in 1 .. LOCAL_MAP_WIDTH-1 {
                let idx = local_map_idx(x, y);
                if build_data.map.tiles[idx] == LocalTileType::Floor {
                    let cell_value_f = noise.get_noise(x as f32, y as f32) * 10240.0;
                    let cell_value = cell_value_f as i32;

                    if noise_areas.contains_key(&cell_value) {
                        noise_areas.get_mut(&cell_value).unwrap().push(idx);
                    } else {
                        noise_areas.insert(cell_value, vec![idx]);
                    }
                }
            }
        }

        // Spawn the entities
        for area in noise_areas.iter() {
            // TODO: Implement this!
            // spawner::spawn_region(&build_data.map, rng, area.1, build_data.map.depth, &mut build_data.spawn_list);
        }
    }
}