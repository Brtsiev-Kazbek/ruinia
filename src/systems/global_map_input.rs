use crate::prelude::*;

pub fn global_map_input(
    key: Option<Res<VirtualKeyCode>>,
    mut global_camera: ResMut<GlobalCamera>,
    mut global_point_position: ResMut<PointG>,
    mut commands: Commands,
) {
    if let Some(key) = key.as_deref() {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };
        let new_position = delta + global_point_position.0;
        global_camera.on_player_move(&new_position);
        global_point_position.0 += delta;

        match key {
            VirtualKeyCode::Escape => commands.insert_resource(NextState(TurnState::MainMenu)),
            _ => (),
        };
    }
}
