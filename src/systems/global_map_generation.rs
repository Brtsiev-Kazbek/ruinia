use crate::prelude::*;

pub fn global_map_generation(mut commands: Commands) {
    let global_map = generate_global_map(GlobalMap::new());
    commands.insert_resource(global_map);
}
