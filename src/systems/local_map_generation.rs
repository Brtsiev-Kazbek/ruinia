use crate::prelude::*;

pub fn local_map_generation(mut commands: Commands) {

    // TODO: Implement match CurrentGlobalTileType => special generation

    let mut rng = RandomNumberGenerator::new();
    let mut local_map_builder = LocalMapBuilderChain::new(0);

    local_map_builder.start_with(BasicLocalMapBuilder::new());
    // local_map_builder.with(BspLocalMapBuilder::new());
    local_map_builder.with(SimpleLocalMapBuilder::new());

    local_map_builder.build_map(&mut rng);
    
    commands.insert_resource(local_map_builder.build_data.map);
}
