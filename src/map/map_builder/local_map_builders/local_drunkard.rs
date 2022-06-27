use crate::prelude::*;

#[derive(PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum DrunkSpawnMode { StartingPoint, Random }

pub struct DrunkardSettings {
    pub spawn_mode : DrunkSpawnMode,
    pub drunken_lifetime : i32,
    pub floor_percent: f32,
    pub brush_size: i32,
    pub symmetry: Symmetry
}

pub struct DrunkardsWalkBuilder {
    settings : DrunkardSettings
}

impl MetaLocalMapBuilder for DrunkardsWalkBuilder {
    #[allow(dead_code)]
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        self.build(rng, build_data);
    }
}

impl DrunkardsWalkBuilder {
    #[allow(dead_code)]
    pub fn new(settings: DrunkardSettings) -> DrunkardsWalkBuilder {
        DrunkardsWalkBuilder{
            settings
        }
    }

    #[allow(dead_code)]
    pub fn open_area() -> Box<DrunkardsWalkBuilder> {
        Box::new(DrunkardsWalkBuilder{
            settings : DrunkardSettings{
                spawn_mode: DrunkSpawnMode::StartingPoint,
                drunken_lifetime: 400,
                floor_percent: 0.5,
                brush_size: 1,
                symmetry: Symmetry::None
            }
        })
    }

    #[allow(dead_code)]
    pub fn open_halls() -> Box<DrunkardsWalkBuilder> {
        Box::new(DrunkardsWalkBuilder{
            settings : DrunkardSettings{
                spawn_mode: DrunkSpawnMode::Random,
                drunken_lifetime: 400,
                floor_percent: 0.5,
                brush_size: 1,
                symmetry: Symmetry::None
            },
        })
    }

    #[allow(dead_code)]
    pub fn winding_passages() -> Box<DrunkardsWalkBuilder> {
        Box::new(DrunkardsWalkBuilder{
            settings : DrunkardSettings{
                spawn_mode: DrunkSpawnMode::Random,
                drunken_lifetime: 100,
                floor_percent: 0.4,
                brush_size: 1,
                symmetry: Symmetry::None
            },
        })
    }

    #[allow(dead_code)]
    pub fn fat_passages() -> Box<DrunkardsWalkBuilder> {
        Box::new(DrunkardsWalkBuilder{
            settings : DrunkardSettings{
                spawn_mode: DrunkSpawnMode::Random,
                drunken_lifetime: 100,
                floor_percent: 0.4,
                brush_size: 2,
                symmetry: Symmetry::None
            },
        })
    }

    #[allow(dead_code)]
    pub fn fearful_symmetry() -> Box<DrunkardsWalkBuilder> {
        Box::new(DrunkardsWalkBuilder{
            settings : DrunkardSettings{
                spawn_mode: DrunkSpawnMode::Random,
                drunken_lifetime: 100,
                floor_percent: 0.4,
                brush_size: 1,
                symmetry: Symmetry::Both
            },
        })
    }
    
    fn build(&mut self, rng : &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        // Set a central starting point
        let starting_position = PointL(Point{ x: LOCAL_MAP_WIDTH / 2, y: LOCAL_MAP_HEIGHT / 2 });
        let start_idx = local_map_idx(starting_position.0.x, starting_position.0.y);
        build_data.map.tiles[start_idx] = LocalTileType::Floor;

        let total_tiles = LOCAL_MAP_WIDTH * LOCAL_MAP_HEIGHT;
        let desired_floor_tiles = (self.settings.floor_percent * total_tiles as f32) as usize;
        let mut floor_tile_count = build_data.map.tiles.iter().filter(|a| **a == LocalTileType::Floor).count();
        let mut digger_count = 0;
        while floor_tile_count  < desired_floor_tiles {
            let mut did_something = false;
            let mut drunk_x;
            let mut drunk_y;
            match self.settings.spawn_mode {
                DrunkSpawnMode::StartingPoint => {
                    drunk_x = starting_position.0.x;
                    drunk_y = starting_position.0.y;
                }
                DrunkSpawnMode::Random => {
                    if digger_count == 0 {
                        drunk_x = starting_position.0.x;
                        drunk_y = starting_position.0.y;
                    } else {
                        drunk_x = rng.roll_dice(1, LOCAL_MAP_WIDTH - 3) + 1;
                        drunk_y = rng.roll_dice(1, LOCAL_MAP_HEIGHT - 3) + 1;
                    }
                }
            }
            let mut drunk_life = self.settings.drunken_lifetime;

            while drunk_life > 0 {
                let drunk_idx = local_map_idx(drunk_x, drunk_y);
                if build_data.map.tiles[drunk_idx] == LocalTileType::Wall {
                    did_something = true;
                }
                paint(&mut build_data.map, self.settings.symmetry, self.settings.brush_size, drunk_x, drunk_y);
                build_data.map.tiles[drunk_idx] = LocalTileType::DownStairs;

                let stagger_direction = rng.roll_dice(1, 4);
                match stagger_direction {
                    1 => { if drunk_x > 2 { drunk_x -= 1; } }
                    2 => { if drunk_x < LOCAL_MAP_WIDTH-2 { drunk_x += 1; } }
                    3 => { if drunk_y > 2 { drunk_y -=1; } }
                    _ => { if drunk_y < LOCAL_MAP_HEIGHT-2 { drunk_y += 1; } }
                }

                drunk_life -= 1;
            }
            if did_something { 
                build_data.take_snapshot(); 
            }

            digger_count += 1;
            for t in build_data.map.tiles.iter_mut() {
                if *t == LocalTileType::DownStairs {
                    *t = LocalTileType::Floor;
                }
            }
            floor_tile_count = build_data.map.tiles.iter().filter(|a| **a == LocalTileType::Floor).count();
        }
    }
}