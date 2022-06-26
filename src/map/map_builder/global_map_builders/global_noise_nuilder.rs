use crate::prelude::*;


pub struct GlobalNoiseBuilder {}

impl MetaGlobalMapBuilder for GlobalNoiseBuilder {
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut GlobalMapBuilder) {
        self.build(rng, build_data);
    }
}

impl GlobalNoiseBuilder {
    #[allow(dead_code)]
    pub fn new() -> Box<GlobalNoiseBuilder> {
        Box::new(GlobalNoiseBuilder {  })
    }

    fn build(&mut self, _rng: &mut RandomNumberGenerator, build_data: &mut GlobalMapBuilder) {

        let mut noise = FastNoise::seeded(_rng.next_u64());

        noise.set_noise_type(NoiseType::SimplexFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(5);
        noise.set_fractal_gain(0.6);
        noise.set_fractal_lacunarity(2.0);
        noise.set_frequency(2.0);

        for y in 0..GLOBAL_MAP_HEIGHT {
            for x in 0..GLOBAL_MAP_WIDTH {
                let n = noise.get_noise((x as f32) / 160.0, (y as f32) / 100.0);
                let idx = global_map_idx(x, y);
                // print!("{}", n);
                if n < -0.15 {
                    build_data.map.tiles[idx] = GlobalTileType::Water;
                } else if n > -0.15 && n < 0.0 {
                    build_data.map.tiles[idx] = GlobalTileType::Plain;
                } else if n > 0.0 && n < 0.2 {
                    build_data.map.tiles[idx] = GlobalTileType::Forest;
                } else {
                    build_data.map.tiles[idx] = GlobalTileType::Mountains;
                 }
            }
        }
    }
}