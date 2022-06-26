use crate::prelude::*;

pub fn main_menu() {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    draw_batch.print_color_centered(2, "Ruinia", ColorPair::new(PURPLE, BLACK));
    draw_batch.print_centered(8, "[P]lay");
    draw_batch.print_centered(10, "[G]enerate global map");
    draw_batch.print_centered(12, "[E]xit");

    draw_batch.submit(100).expect("Batch error");
}

pub fn main_menu_input(mut commands: Commands, key: Option<Res<VirtualKeyCode>>) {
    if let Some(key) = key.as_deref() {
        match key {
            VirtualKeyCode::P => {
                commands.insert_resource(NextState(TurnState::AwaitingInput));
                println!("State GlobalMap");
            }
            VirtualKeyCode::G => {
                commands.insert_resource(NextState(TurnState::GlobalMap));
                println!("State GlobalMap");
            }
            _ => {
                println!("State MainMenu");
            }
        }
    }
    commands.remove_resource::<VirtualKeyCode>();
}
