mod global_map_generation;
mod global_map_input;
mod main_menu;
mod global_map_render;
mod local_map_render;
mod local_map_generation;
mod local_map_input;

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
            .with_system(
                global_map_generation::global_map_generation
                    .run_unless_resource_exists::<GlobalMap>(),
            )
            .with_system(global_map_render::global_map_render.run_if_resource_exists::<GlobalMap>())
            .with_system(global_map_input::global_map_input)
            .into(),
    );

    app.add_system_set(
        ConditionSet::new()
            .run_in_state(TurnState::AwaitingInput)
            .with_system(
                local_map_generation::local_map_generation
                    .run_unless_resource_exists::<LocalMap>(),
            )
            .with_system(local_map_render::local_map_render.run_if_resource_exists::<LocalMap>())
            .with_system(local_map_input::local_map_input)
            .into(),
    );
}
