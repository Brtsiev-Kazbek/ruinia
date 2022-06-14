mod global_map_input;
mod main_menu;
mod map_render;

use crate::prelude::*;

pub fn build_systems_set(app: &mut App) {
    // Системы слушают изменения игрового стета и вызываются с помощью iyes_loopless

    app.add_system_set(
        ConditionSet::new()
            .run_in_state(TurnState::MainMenu)
            .with_system(main_menu::main_menu)
            .with_system(main_menu::main_menu_input)
            .into(),
    );

    app.add_system_set(
        ConditionSet::new()
            .run_in_state(TurnState::GlobalMap)
            .with_system(map_render::global_map_render)
            .with_system(global_map_input::global_map_input)
            .into(),
    );
}
