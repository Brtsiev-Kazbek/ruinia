use crate::prelude::*;

mod constraints;
mod common;
mod solver;

pub use common::*;
pub use constraints::*;
pub use solver::*;

pub struct WaveformCollapseBuilder {}

impl MetaLocalMapBuilder for WaveformCollapseBuilder {
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder)  {
        self.build(rng, build_data);
    }
}

impl WaveformCollapseBuilder {
    /// Constructor for waveform collapse.
    #[allow(dead_code)]
    pub fn new() -> Box<WaveformCollapseBuilder> {
        Box::new(WaveformCollapseBuilder{})
    }

    fn build(&mut self, rng : &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        const CHUNK_SIZE :i32 = 8;
        build_data.take_snapshot();

        let patterns = build_patterns(&build_data.map, CHUNK_SIZE, true, true);
        let constraints = patterns_to_constraints(patterns, CHUNK_SIZE);
        self.render_tile_gallery(&constraints, CHUNK_SIZE, build_data);
                
        build_data.map = LocalMap::new(build_data.map.depth);
        loop {
            let mut solver = Solver::new(constraints.clone(), CHUNK_SIZE, &build_data.map);
            while !solver.iteration(&mut build_data.map, rng) {
                build_data.take_snapshot();
            }
            build_data.take_snapshot();
            if solver.possible { break; } // If it has hit an impossible condition, try again
        }
        build_data.spawn_list.clear();
    }

    fn render_tile_gallery(&mut self, constraints: &[LocalMapChunk], chunk_size: i32, build_data : &mut LocalMapBuilder) {
        build_data.map = LocalMap::new(0);
        let mut counter = 0;
        let mut x = 1;
        let mut y = 1;
        while counter < constraints.len() {
            render_pattern_to_map(&mut build_data.map, &constraints[counter], chunk_size, x, y);

            x += chunk_size + 1;
            if x + chunk_size > LOCAL_MAP_WIDTH {
                // Move to the next row
                x = 1;
                y += chunk_size + 1;

                if y + chunk_size > LOCAL_MAP_HEIGHT {
                    // Move to the next page
                    build_data.take_snapshot();
                    build_data.map = LocalMap::new(0);

                    x = 1;
                    y = 1;
                }
            }

            counter += 1;
        }
        build_data.take_snapshot();
    }
}