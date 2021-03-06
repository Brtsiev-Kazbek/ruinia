use crate::prelude::*;

// pub struct GlobalStructuresBuilder {
//     map: GlobalMap,
//     rng: RandomNumberGenerator,
// }

// impl GlobalStructuresBuilder {
//     pub fn new(map: GlobalMap) -> Self {
//         let rng: RandomNumberGenerator = RandomNumberGenerator::new();

//         Self { map, rng }
//     }

//     pub fn build(&mut self) -> GlobalMap {
//         let mut changed_map = self.map.clone();

//         self.place_towns(&mut changed_map);
//         self.place_monster_fortress(&mut changed_map);
//         self.place_temples(&mut changed_map);

//         changed_map
//     }

//     pub fn place_towns(&mut self, map: &mut GlobalMap) {
//         let map_len = map.tiles.len();
//         let mut town_count = TOWN_COUNT as usize;
//         while town_count > 0 {
//             let random_tile_num = self.rng.range(0, map_len) as usize;

//             // TODO: REFACTOR THIS!!!
//             let selected_tile = self.map.tiles[random_tile_num];
//             if selected_tile != GlobalTileType::Water
//                 && selected_tile != GlobalTileType::Mountains
//                 && selected_tile != GlobalTileType::Town
//             {
//                 map.tiles[random_tile_num] = GlobalTileType::Town;
//                 town_count -= 1;
//             }
//         }
//     }

//     pub fn place_monster_fortress(&mut self, map: &mut GlobalMap) {
//         let map_len = map.tiles.len();
//         let mut fortress_count = FORTRESS_COUNT;
//         while fortress_count >= 1 {
//             let random_tile_num = self.rng.range(0, map_len) as usize;

//             // TODO: REFACTOR THIS!!!
//             let selected_tile = self.map.tiles[random_tile_num];
//             if selected_tile != GlobalTileType::Water
//                 && selected_tile != GlobalTileType::Mountains
//                 && selected_tile != GlobalTileType::Town
//                 && selected_tile != GlobalTileType::MonsterFortress
//             {
//                 map.tiles[random_tile_num] = GlobalTileType::MonsterFortress;
//                 fortress_count -= 1;
//             }
//         }
//     }

//     pub fn place_temples(&mut self, map: &mut GlobalMap) {
//         let map_len = map.tiles.len();
//         let mut temple_count = TEMPLE_COUNT;
//         while temple_count >= 1 {
//             let random_tile_num = self.rng.range(0, map_len) as usize;

//             // TODO: REFACTOR THIS!!!
//             let selected_tile = self.map.tiles[random_tile_num];
//             if selected_tile != GlobalTileType::Water
//                 && selected_tile != GlobalTileType::Mountains
//                 && selected_tile != GlobalTileType::Town
//                 && selected_tile != GlobalTileType::MonsterFortress
//                 && selected_tile != GlobalTileType::Temple
//             {
//                 map.tiles[random_tile_num] = GlobalTileType::Temple;
//                 temple_count -= 1;
//             }
//         }
//     }
// }

pub struct GlobalStructuresBuilder {}

impl MetaGlobalMapBuilder for GlobalStructuresBuilder {
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut GlobalMapBuilder) {
        self.build(rng, build_data);
    }
}

impl GlobalStructuresBuilder {
    pub fn new() -> Box<GlobalStructuresBuilder> {
        Box::new(GlobalStructuresBuilder {  })
    }

    fn build(&mut self, rng: &mut RandomNumberGenerator, build_data: &mut GlobalMapBuilder) {
        self.place_towns(rng, build_data);
        self.place_monster_fortress(rng, build_data);
        self.place_temples(rng, build_data);
    }

    pub fn place_towns(&mut self, rng: &mut RandomNumberGenerator, build_data: &mut GlobalMapBuilder) {
        let map_len = build_data.map.tiles.len();
        let mut town_count = TOWN_COUNT as usize;
        while town_count > 0 {
            let random_tile_num = rng.range(0, map_len) as usize;

            // TODO: REFACTOR THIS!!!
            let selected_tile = build_data.map.tiles[random_tile_num];
            if selected_tile != GlobalTileType::Water
                && selected_tile != GlobalTileType::Mountains
                && selected_tile != GlobalTileType::Town
            {
                build_data.map.tiles[random_tile_num] = GlobalTileType::Town;
                town_count -= 1;
            }
        }
    }

    pub fn place_monster_fortress(&mut self, rng: &mut RandomNumberGenerator, build_data: &mut GlobalMapBuilder) {
        let map_len = build_data.map.tiles.len();
        let mut fortress_count = FORTRESS_COUNT as usize;
        while fortress_count > 0 {
            let random_tile_num = rng.range(0, map_len) as usize;

            // TODO: REFACTOR THIS!!!
            let selected_tile = build_data.map.tiles[random_tile_num];
            if selected_tile != GlobalTileType::Water
                && selected_tile != GlobalTileType::Mountains
                && selected_tile != GlobalTileType::Town
                && selected_tile != GlobalTileType::MonsterFortress
            {
                build_data.map.tiles[random_tile_num] = GlobalTileType::MonsterFortress;
                fortress_count -= 1;
            }
        }
    }

    pub fn place_temples(&mut self, rng: &mut RandomNumberGenerator, build_data: &mut GlobalMapBuilder) {
        let map_len = build_data.map.tiles.len();
        let mut temple_count = TEMPLE_COUNT as usize;
        while temple_count > 0 {
            let random_tile_num = rng.range(0, map_len) as usize;

            // TODO: REFACTOR THIS!!!
            let selected_tile = build_data.map.tiles[random_tile_num];
            if selected_tile != GlobalTileType::Water
                && selected_tile != GlobalTileType::Mountains
                && selected_tile != GlobalTileType::Town
                && selected_tile != GlobalTileType::MonsterFortress
                && selected_tile != GlobalTileType::Temple
            {
                build_data.map.tiles[random_tile_num] = GlobalTileType::Temple;
                temple_count -= 1;
            }
        }
    }
}