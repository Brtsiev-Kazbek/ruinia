use crate::prelude::*;

pub struct CellularAutomataBuilder {}

impl MetaLocalMapBuilder for CellularAutomataBuilder {
    #[allow(dead_code)]
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        self.build(rng, build_data);
    }
}

impl CellularAutomataBuilder {
    #[allow(dead_code)]
    pub fn new() -> Box<CellularAutomataBuilder> {
        Box::new(CellularAutomataBuilder{})
    }

    #[allow(clippy::map_entry)]
    fn build(&mut self, rng : &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        // First we completely randomize the map, setting 55% of it to be floor.
        for y in 1..LOCAL_MAP_HEIGHT-1 {
            for x in 1..LOCAL_MAP_WIDTH-1 {
                let roll = rng.roll_dice(1, 100);
                let idx = local_map_idx(x, y);
                if roll > 55 { build_data.map.tiles[idx] = LocalTileType::Floor } 
                else { build_data.map.tiles[idx] = LocalTileType::Wall }
            }
        }
        build_data.take_snapshot();

        // Now we iteratively apply cellular automata rules
        for _i in 0..15 {
            let mut newtiles = build_data.map.tiles.clone();

            for y in 1..LOCAL_MAP_HEIGHT-1 {
                for x in 1..LOCAL_MAP_WIDTH-1 {
                    let idx = local_map_idx(x, y);
                    let mut neighbors = 0;
                    if build_data.map.tiles[idx - 1] == LocalTileType::Wall { neighbors += 1; }
                    if build_data.map.tiles[idx + 1] == LocalTileType::Wall { neighbors += 1; }
                    if build_data.map.tiles[idx - LOCAL_MAP_HEIGHT as usize] == LocalTileType::Wall { neighbors += 1; }
                    if build_data.map.tiles[idx + LOCAL_MAP_HEIGHT as usize] == LocalTileType::Wall { neighbors += 1; }
                    if build_data.map.tiles[idx - (LOCAL_MAP_HEIGHT as usize - 1)] == LocalTileType::Wall { neighbors += 1; }
                    if build_data.map.tiles[idx - (LOCAL_MAP_HEIGHT as usize + 1)] == LocalTileType::Wall { neighbors += 1; }
                    if build_data.map.tiles[idx + (LOCAL_MAP_HEIGHT as usize - 1)] == LocalTileType::Wall { neighbors += 1; }
                    if build_data.map.tiles[idx + (LOCAL_MAP_HEIGHT as usize + 1)] == LocalTileType::Wall { neighbors += 1; }

                    if neighbors > 4 || neighbors == 0 {
                        newtiles[idx] = LocalTileType::Wall;
                    }
                    else {
                        newtiles[idx] = LocalTileType::Floor;
                    }
                }
            }

            build_data.map.tiles = newtiles.clone();
            build_data.take_snapshot();
        }
    }
}