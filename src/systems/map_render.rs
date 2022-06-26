use crate::prelude::*;

pub fn global_map_render(global_map: Res<GlobalMap>, global_camera: Res<GlobalCamera>, global_point_position: ResMut<PointG>) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    for y in global_camera.top_y..=global_camera.bottom_y {
        for x in global_camera.left_x..global_camera.right_x {
            let position = Point::new(x, y);
            let offset = Point::new(global_camera.left_x, global_camera.top_y);

            if global_map.in_bounds(position) {
                let idx = global_map_idx(x, y);

                match global_map.tiles[idx] {
                    GlobalTileType::Forest => {
                        draw_batch.set(
                            position - offset,
                            ColorPair::new(GREEN, BLACK),
                            to_cp437('♣'),
                        );
                    }
                    GlobalTileType::Mountains => {
                        draw_batch.set(
                            position - offset,
                            ColorPair::new(GREY, BLACK),
                            to_cp437('▲'),
                        );
                    }
                    GlobalTileType::Plain => {
                        draw_batch.set(
                            position - offset,
                            ColorPair::new(GREEN, BLACK),
                            to_cp437('.'),
                        );
                    }
                    GlobalTileType::Water => {
                        draw_batch.set(
                            position - offset,
                            ColorPair::new(BLUE, BLACK),
                            to_cp437('≈'),
                        );
                    }
                    GlobalTileType::MonsterFortress => {
                        draw_batch.set(
                            position - offset,
                            ColorPair::new(PURPLE, BLACK),
                            to_cp437('Ω'),
                        );
                    }
                    GlobalTileType::Town => {
                        draw_batch.set(
                            position - offset,
                            ColorPair::new(BROWN1, BLACK),
                            to_cp437('⌂'),
                        );
                    }
                    GlobalTileType::Temple => {
                        draw_batch.set(
                            position - offset,
                            ColorPair::new(YELLOW, BLACK),
                            to_cp437('±'),
                        );
                    }
                }
            }

            if x == global_point_position.0.x && y == global_point_position.0.y {
                draw_batch.set(
                    position - offset,
                    ColorPair::new(ORANGE, BLACK),
                    to_cp437('X'),
                );
            }
        }
    }

    draw_batch.submit(0).expect("Batch error");
}
