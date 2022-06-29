use crate::prelude::*;

pub struct BasicLocalMapBuilder {}

impl InitialLocalMapBuilder for BasicLocalMapBuilder {
    #[allow(dead_code)]
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        println!("[BasicLocalMapBuilder]");
        build_data.map.tiles.fill(LocalTileType::Wall);
    }
}

impl BasicLocalMapBuilder {
    pub fn new() -> Box<BasicLocalMapBuilder> {
        Box::new(BasicLocalMapBuilder {  })
    }
}