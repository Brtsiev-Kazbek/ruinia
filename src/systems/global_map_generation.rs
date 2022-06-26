use crate::prelude::*;

pub fn global_map_generation(mut commands: Commands) {
    let mut rng = RandomNumberGenerator::new();
    let mut global_map_builder = GlobalMapBuilderChain::new();

    global_map_builder.start_with(SimpleGlobalMapBuilder::new());
    global_map_builder.with(GlobalNoiseBuilder::new());
    global_map_builder.with(GlobalStructuresBuilder::new());
    global_map_builder.build_map(&mut rng);
    
    commands.insert_resource(global_map_builder.build_data.map);
}
