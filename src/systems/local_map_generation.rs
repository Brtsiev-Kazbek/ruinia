use crate::prelude::*;

pub fn local_map_generation(mut commands: Commands) {

    // TODO: Implement match CurrentGlobalTileType => special generation

    let mut rng = RandomNumberGenerator::new();
    let mut local_map_builder = LocalMapBuilderChain::new(0);

    local_map_builder.start_with(BasicLocalMapBuilder::new());
    local_map_builder.with(BspInteriorBuilder::new());
    // local_map_builder.with(BspLocalMapBuilder::new());
    // local_map_builder.with(RoomSorter::new(RoomSort::BOTTOMMOST));
    // local_map_builder.with(BspCorridors::new());
    // local_map_builder.with(RoomCornerRounder::new());
    // local_map_builder.with(RoomExploder::new());
    local_map_builder.with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER));
    local_map_builder.with(CullUnreachable::new());
    local_map_builder.with(VoronoiSpawning::new());
    local_map_builder.with(DistantExit::new());

    // BSP TESTING
    // local_map_builder.start_with(BasicLocalMapBuilder::new());
    // local_map_builder.with(BspLocalMapBuilder::new());
    // local_map_builder.with(SimpleLocalMapBuilder::new());
    // local_map_builder.with(BspInteriorBuilder::new());


    // CELLULAR TESTING
    // local_map_builder.start_with(BasicLocalMapBuilder::new());
    // local_map_builder.with(CellularAutomataBuilder::new());
    // local_map_builder.with(AreaStartingPosition::new(XStart::LEFT, YStart::CENTER));
    // local_map_builder.with(CullUnreachable::new());
    // local_map_builder.with(DistantExit::new());


    //DRUNKARDS_WALK TESTING 
    // local_map_builder.start_with(BasicLocalMapBuilder::new());
    // local_map_builder.with(DrunkardsWalkBuilder::fearful_symmetry());
    // local_map_builder.with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER));
    // local_map_builder.with(CullUnreachable::new());
    // local_map_builder.with(DistantExit::new());


    //DLA TESTING 
    // local_map_builder.start_with(BasicLocalMapBuilder::new());
    // local_map_builder.with(DLABuilder::insectoid());
    // local_map_builder.with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER));
    // local_map_builder.with(CullUnreachable::new());
    // local_map_builder.with(DistantExit::new());


    //MAZE TESTING 
    // local_map_builder.start_with(BasicLocalMapBuilder::new());
    // local_map_builder.with(MazeBuilder::new());
    // local_map_builder.with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER));
    // local_map_builder.with(DistantExit::new());


    //VORONOI TESTING 
    // local_map_builder.start_with(BasicLocalMapBuilder::new());
    // local_map_builder.with(VoronoiCellBuilder::new());
    // local_map_builder.with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER));
    // local_map_builder.with(CullUnreachable::new());
    // local_map_builder.with(DistantExit::new());


    //WAVEFORM TESTING
    // local_map_builder.start_with(BasicLocalMapBuilder::new());
    // local_map_builder.with(VoronoiCellBuilder::pythagoras());
    // local_map_builder.with(WaveformCollapseBuilder::new());
    // local_map_builder.with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER));
    // local_map_builder.with(CullUnreachable::new());
    // local_map_builder.with(DistantExit::new());


    // TODO: Implement prefab builder

    local_map_builder.build_map(&mut rng);
    
    commands.insert_resource(local_map_builder.build_data.map);
}
