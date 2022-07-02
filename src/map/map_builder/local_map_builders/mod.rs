use crate::prelude::*;
use bracket_lib::prelude::Rect;

mod local_bsp_dungeon_builder;
mod local_basic_map_builder;
mod local_simple_map_builder;
mod local_room_based_spawner;
mod local_room_based_position;
mod local_room_based_stairs;
mod local_bsp_interior_builder;
mod local_cellular_automata_builder;
mod local_area_starting_points;
mod local_cull_unreachable;
mod local_voronoi_spawning;
mod local_distant_exit;
mod local_drunkard;
mod local_dla_builder;
mod local_maze_builder;
mod local_voronoi_builder;
mod local_waveform_collapse;
mod local_room_exploder;
mod local_room_corner_rounding;
mod local_rooms_corridors_dogleg;
mod local_rooms_corridors_bsp;
mod local_room_sorter;
mod local_room_draw;
mod local_rooms_corridors_nearest;
mod local_rooms_corridors_lines;
mod local_room_corridor_spawner;
mod common;

pub use local_bsp_dungeon_builder::*;
pub use local_basic_map_builder::*;
pub use local_simple_map_builder::*;
pub use local_room_based_spawner::*;
pub use local_room_based_position::*;
pub use local_room_based_stairs::*;
pub use local_bsp_interior_builder::*;
pub use local_cellular_automata_builder::*;
pub use local_area_starting_points::*;
pub use local_cull_unreachable::*;
pub use local_voronoi_spawning::*;
pub use local_distant_exit::*;
pub use local_drunkard::*;
pub use local_dla_builder::*;
pub use local_maze_builder::*;
pub use local_voronoi_builder::*;
pub use local_waveform_collapse::*;
pub use local_room_exploder::*;
pub use local_room_corner_rounding::*;
pub use local_rooms_corridors_dogleg::*;
pub use local_rooms_corridors_bsp::*;
pub use local_room_sorter::*;
pub use local_room_draw::*;
pub use local_rooms_corridors_nearest::*;
pub use local_rooms_corridors_lines::*;
pub use local_room_corridor_spawner::*;
pub use common::*;

pub struct LocalMapBuilder {
    pub spawn_list: Vec<(usize, String)>,
    pub map: LocalMap,
    pub starting_position: Option<PointL>,
    pub rooms: Option<Vec<Rect>>,
    pub corridors: Option<Vec<Vec<usize>>>,
    pub history: Vec<LocalMap>
}

impl LocalMapBuilder {
    fn take_snapshot(&mut self) {
        let snapshot = self.map.clone();

        self.history.push(snapshot);
    }
}

pub struct LocalMapBuilderChain {
    starter: Option<Box<dyn InitialLocalMapBuilder>>,
    builders: Vec<Box<dyn MetaLocalMapBuilder>>,
    pub build_data: LocalMapBuilder
}

impl LocalMapBuilderChain {
    pub fn new(depth: i32) -> LocalMapBuilderChain {
        LocalMapBuilderChain {
            starter: None,
            builders: Vec::new(),
            build_data: LocalMapBuilder {
                spawn_list: Vec::new(),
                map: LocalMap::new(depth),
                starting_position: None,
                rooms: None,
                corridors: None,
                history: Vec::new()
            }
        }
    }

    pub fn start_with(&mut self, starter: Box<dyn InitialLocalMapBuilder>) {
        match self.starter {
            None => self.starter = Some(starter),
            Some(_) => panic!("You can only have one starting builder.")
        };
    }

    pub fn with(&mut self, metabuilder: Box<dyn MetaLocalMapBuilder>) {
        self.builders.push(metabuilder);
    }

    pub fn build_map(&mut self, rng: &mut RandomNumberGenerator) {
        match &mut self.starter {
            None => panic!("Cannot run a map builder chain without a starting build system."),
            Some(starter) => {
                starter.build_map(rng, &mut self.build_data);
            }
        }

        for metabuilder in self.builders.iter_mut() {
            metabuilder.build_map(rng, &mut self.build_data);
        }
    }

    pub fn spawn_entities(&mut self, ecs: World) {
        for entity in self.build_data.spawn_list.iter() {
            // TODO: Implement this
            // spawner::spawn_entity(ecs, &(&entity.0, &entity.1));
        }
    }
}

pub trait InitialLocalMapBuilder {
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder);
}

pub trait MetaLocalMapBuilder {    
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder);
}