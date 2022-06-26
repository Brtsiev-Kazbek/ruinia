use crate::prelude::*;

pub struct SimpleGlobalMapBuilder {}

impl InitialGlobalMapBuilder for SimpleGlobalMapBuilder {
    #[allow(dead_code)]
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut GlobalMapBuilder) {
        build_data.map.tiles.fill(GlobalTileType::Plain);
    }
}

impl SimpleGlobalMapBuilder {
    pub fn new() -> Box<SimpleGlobalMapBuilder> {
        Box::new(SimpleGlobalMapBuilder {  })
    }
}