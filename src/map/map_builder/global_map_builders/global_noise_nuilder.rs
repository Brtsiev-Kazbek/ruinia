use crate::prelude::*;

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
