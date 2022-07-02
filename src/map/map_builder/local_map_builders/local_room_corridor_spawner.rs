use crate::prelude::*;


pub struct CorridorSpawner {}

impl MetaLocalMapBuilder for CorridorSpawner {
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder)  {
        self.build(rng, build_data);
    }
}

impl CorridorSpawner {
    #[allow(dead_code)]
    pub fn new() -> Box<CorridorSpawner> {
        Box::new(CorridorSpawner{})
    }

    fn build(&mut self, rng : &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        if let Some(corridors) = &build_data.corridors {
            for c in corridors.iter() {
                let depth = build_data.map.depth;

                // TODO: Implement this
            //     spawner::spawn_region(&build_data.map, 
            //         rng, 
            //         &c, 
            //         depth, 
            //         &mut build_data.spawn_list);
            }
        } else {
            panic!("Corridor Based Spawning only works after corridors have been created");
        }
    }
}