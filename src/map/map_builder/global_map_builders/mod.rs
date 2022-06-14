use crate::prelude::*;

pub fn generate_global_map(map: GlobalMap) -> GlobalMap {
    let mut noise_map_builder = GlobalNoiseBuilder::new(map);
    let noise_map = noise_map_builder.build();

    let mut structures_map_builder = GlobalStructuresBuilder::new(noise_map);
    let structures_map = structures_map_builder.build();

    structures_map
}

pub struct GlobalNoiseBuilder {
    map: GlobalMap,
    rng: RandomNumberGenerator,
    noise: FastNoise,
}

impl GlobalNoiseBuilder {
    pub fn new(map: GlobalMap) -> Self {
        let mut rng = RandomNumberGenerator::new();
        let mut noise = FastNoise::seeded(rng.next_u64());

        noise.set_noise_type(NoiseType::SimplexFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(5);
        noise.set_fractal_gain(0.6);
        noise.set_fractal_lacunarity(2.0);
        noise.set_frequency(2.0);

        Self { map, noise, rng }
    }

    pub fn build(&mut self) -> GlobalMap {
        let mut changed_map = self.map.clone();
        for y in 0..GLOBAL_MAP_HEIGHT {
            for x in 0..GLOBAL_MAP_WIDTH {
                let n = self.noise.get_noise((x as f32) / 160.0, (y as f32) / 100.0);
                let idx = global_map_idx(x, y);
                // print!("{}", n);
                if n < -0.15 {
                    changed_map.tiles[idx] = GlobalTileType::Water;
                } else if n > -0.15 && n < 0.0 {
                    changed_map.tiles[idx] = GlobalTileType::Plain;
                } else if n > 0.0 && n < 0.2 {
                    changed_map.tiles[idx] = GlobalTileType::Forest;
                } else {
                    changed_map.tiles[idx] = GlobalTileType::Mountains;
                }
            }
        }

        changed_map
    }
}

pub struct GlobalStructuresBuilder {
    map: GlobalMap,
    rng: RandomNumberGenerator,
}

impl GlobalStructuresBuilder {
    pub fn new(map: GlobalMap) -> Self {
        let rng: RandomNumberGenerator = RandomNumberGenerator::new();

        Self { map, rng }
    }

    pub fn build(&mut self) -> GlobalMap {
        let mut changed_map = self.map.clone();

        self.place_towns(&mut changed_map);
        self.place_monster_fortress(&mut changed_map);
        self.place_temples(&mut changed_map);

        changed_map
    }

    pub fn place_towns(&mut self, map: &mut GlobalMap) {
        let map_len = map.tiles.len();
        let mut town_count = TOWN_COUNT as usize;
        while town_count > 0 {
            let random_tile_num = self.rng.range(0, map_len) as usize;

            // TODO: REFACTOR THIS!!!
            let selected_tile = self.map.tiles[random_tile_num];
            if selected_tile != GlobalTileType::Water
                && selected_tile != GlobalTileType::Mountains
                && selected_tile != GlobalTileType::Town
            {
                map.tiles[random_tile_num] = GlobalTileType::Town;
                town_count -= 1;
            }
        }
    }

    pub fn place_monster_fortress(&mut self, map: &mut GlobalMap) {
        let map_len = map.tiles.len();
        let mut fortress_count = FORTRESS_COUNT;
        while fortress_count >= 1 {
            let random_tile_num = self.rng.range(0, map_len) as usize;

            // TODO: REFACTOR THIS!!!
            let selected_tile = self.map.tiles[random_tile_num];
            if selected_tile != GlobalTileType::Water
                && selected_tile != GlobalTileType::Mountains
                && selected_tile != GlobalTileType::Town
                && selected_tile != GlobalTileType::MonsterFortress
            {
                map.tiles[random_tile_num] = GlobalTileType::MonsterFortress;
                fortress_count -= 1;
            }
        }
    }

    pub fn place_temples(&mut self, map: &mut GlobalMap) {
        let map_len = map.tiles.len();
        let mut temple_count = TEMPLE_COUNT;
        while temple_count >= 1 {
            let random_tile_num = self.rng.range(0, map_len) as usize;

            // TODO: REFACTOR THIS!!!
            let selected_tile = self.map.tiles[random_tile_num];
            if selected_tile != GlobalTileType::Water
                && selected_tile != GlobalTileType::Mountains
                && selected_tile != GlobalTileType::Town
                && selected_tile != GlobalTileType::MonsterFortress
                && selected_tile != GlobalTileType::Temple
            {
                map.tiles[random_tile_num] = GlobalTileType::Temple;
                temple_count -= 1;
            }
        }
    }
}
