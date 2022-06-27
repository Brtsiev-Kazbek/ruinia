use crate::prelude::*;

pub fn local_map_render(local_map: Res<LocalMap>, local_camera: Res<LocalCamera>, local_point_position: ResMut<PointG>) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    for y in local_camera.top_y..=local_camera.bottom_y {
        for x in local_camera.left_x..local_camera.right_x {
            let position = Point::new(x, y);
            let offset = Point::new(local_camera.left_x, local_camera.top_y);

            if local_map.in_bounds(position) {
                let idx = local_map_idx(x, y);

                match local_map.tiles[idx] {
                    LocalTileType::Floor => {
                        draw_batch.set(
                            position - offset,
                            ColorPair::new(GREY, BLACK),
                            to_cp437('.'),
                        );
                    }
                    LocalTileType::Wall => {
                        draw_batch.set(
                            position - offset,
                            ColorPair::new(GREY, BLACK),
                            to_cp437('#'),
                        );
                    }
                    LocalTileType::DownStairs => {
                        draw_batch.set(
                            position - offset,
                            ColorPair::new(WHITE, BLACK),
                            to_cp437('↓'),
                        );
                    }
                }
            }

            // if x == local_point_position.0.x && y == local_point_position.0.y {
            //     draw_batch.set(
            //         position - offset,
            //         ColorPair::new(WHITE, BLACK),
            //         to_cp437('@'),
            //     );
            // }
        }
    }

    draw_batch.submit(0).expect("Batch error");
}