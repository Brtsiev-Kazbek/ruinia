mod local_map_builder;

use crate::prelude::*;
use bracket_lib::prelude::Rect;

pub struct LocalMapBuilder {
    pub spawn_list: Vec<(usize, String)>,
    pub map: LocalMap,
    pub starting_position: Option<PointL>,
    pub rooms: Option<Vec<Rect>>,
    pub history: Vec<LocalMap>
}

impl LocalMapBuilder {
    fn take_snapshot(&mut self) {
        let snapshot = self.map.clone();

        self.history.push(snapshot);
    }
}

pub struct BuilderChain {
    starter: Option<Box<dyn InitialMapBuilder>>,
    builders: Vec<Box<dyn MetaMapBuilder>>,
    pub build_data: LocalMapBuilder
}

impl BuilderChain {
    pub fn new(new_depth: i32) -> BuilderChain {
        BuilderChain {
            starter: None,
            builders: Vec::new(),
            build_data: LocalMapBuilder {
                spawn_list: Vec::new(),
                map: LocalMap::new(new_depth),
                starting_position: None,
                rooms: None,
                history: Vec::new()
            }
        }
    }

    pub fn start_with(&mut self, starter: Box<dyn InitialMapBuilder>) {
        match self.starter {
            None => self.starter = Some(starter),
            Some(_) => panic!("You can only have one starting builder.")
        };
    }

    pub fn with(&mut self, metabuilder: Box<dyn MetaMapBuilder>) {
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

pub trait InitialMapBuilder {
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder);
}

pub trait MetaMapBuilder {    
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder);
}