use crate::prelude::*;

pub fn local_map_input(
    key: Option<Res<VirtualKeyCode>>,
    mut local_camera: ResMut<LocalCamera>,
    mut local_point_position: ResMut<PointL>,
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
        let new_position = delta + local_point_position.0;
        local_camera.on_player_move(new_position);
        local_point_position.0 += delta;

        match key {
            VirtualKeyCode::Escape => commands.insert_resource(NextState(TurnState::MainMenu)),
            _ => (),
        };
    }
}
