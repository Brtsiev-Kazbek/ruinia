use crate::prelude::*;
pub use global_noise_nuilder::*;
pub use global_structures_builder::*;
pub use global_simple_builder::*;
pub use bracket_lib::prelude::Rect;


mod global_noise_nuilder;
mod global_structures_builder;
mod global_simple_builder;

pub struct GlobalMapBuilder {
    pub spawn_list: Vec<(usize, String)>,
    pub map: GlobalMap,
    pub starting_position: Option<PointL>,
    pub rooms: Option<Vec<Rect>>,
    pub history: Vec<GlobalMap>
}

impl GlobalMapBuilder {
    fn take_snapshot(&mut self) {
        let snapshot = self.map.clone();

        self.history.push(snapshot);
    }
}

pub struct GlobalMapBuilderChain {
    starter: Option<Box<dyn InitialGlobalMapBuilder>>,
    builders: Vec<Box<dyn MetaGlobalMapBuilder>>,
    pub build_data: GlobalMapBuilder
}

impl GlobalMapBuilderChain {
    pub fn new() -> GlobalMapBuilderChain {
        GlobalMapBuilderChain {
            starter: None,
            builders: Vec::new(),
            build_data: GlobalMapBuilder {
                spawn_list: Vec::new(),
                map: GlobalMap::new(),
                starting_position: None,
                rooms: None,
                history: Vec::new()
            }
        }
    }

    pub fn start_with(&mut self, starter: Box<dyn InitialGlobalMapBuilder>) {
        match self.starter {
            None => self.starter = Some(starter),
            Some(_) => panic!("You can only have one starting builder.")
        };
    }

    pub fn with(&mut self, metabuilder: Box<dyn MetaGlobalMapBuilder>) {
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

pub trait InitialGlobalMapBuilder {
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut GlobalMapBuilder);
}

pub trait MetaGlobalMapBuilder {    
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut GlobalMapBuilder);
}