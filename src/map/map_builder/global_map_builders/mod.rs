use crate::prelude::*;
use global_noise_nuilder::*;
use global_structures_builder::*;

mod global_noise_nuilder;
mod global_structures_builder;

pub fn generate_global_map(map: GlobalMap) -> GlobalMap {
    let mut noise_map_builder = GlobalNoiseBuilder::new(map);
    let noise_map = noise_map_builder.build();

    let mut structures_map_builder = GlobalStructuresBuilder::new(noise_map);
    let structures_map = structures_map_builder.build();

    structures_map
}
