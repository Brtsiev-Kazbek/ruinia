mod camera;
mod map;
mod resource_wrappers;
mod systems;
mod turn_state;

mod prelude {
    pub use bevy::prelude::*;
    pub use bracket_lib::prelude::*;

    pub const SCREEN_WIDTH: i32 = 100;
    pub const SCREEN_HEIGHT: i32 = 60;

    // Размер экрана
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;

    // Размер глобальной карты
    pub const GLOBAL_MAP_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const GLOBAL_MAP_HEIGHT: i32 = SCREEN_HEIGHT / 2;

    // Размер локальной карты
    pub const LOCAL_MAP_WIDTH: i32 = 250;
    pub const LOCAL_MAP_HEIGHT: i32 = 250;

    pub const TOWN_COUNT: i32 = 2;
    pub const FORTRESS_COUNT: i32 = 1;
    pub const TEMPLE_COUNT: i32 = 4;

    pub use crate::camera::*;
    pub use crate::map::*;
    pub use crate::resource_wrappers::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
    pub use iyes_loopless::prelude::*;
}

use crate::prelude::*;

struct State {
    ecs: App,
}

impl State {
    pub fn new() -> Self {
        let mut ecs = App::new();
        ecs.add_loopless_state(TurnState::MainMenu);

        ecs.insert_resource(PointG(Point::new(DISPLAY_WIDTH / 2, DISPLAY_HEIGHT / 2)));

        // TODO: Implement getting player position depends on local map type
        ecs.insert_resource(PointL(Point::new(DISPLAY_WIDTH / 2, DISPLAY_HEIGHT / 2)));


        ecs.insert_resource(GlobalCamera::new(Point::new(
            DISPLAY_WIDTH / 2,
            DISPLAY_HEIGHT / 2,
        )));

        ecs.insert_resource(LocalCamera::new(Point::new(
            DISPLAY_WIDTH / 2,
            DISPLAY_HEIGHT / 2,
        )));

        build_systems_set(&mut ecs);
        Self { ecs }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();

        // Добавляем ресурс, обозначающий нажатую кнопку
        if let Some(key) = ctx.key {
            self.ecs.insert_resource(key);
        } else {
            // Напрямую из ecs удалить ресурс пока невозможно, необходимо обратиться к world
            // Удалить ресурс необходимо для того, чтобы в следующем тике нажатая кнопка не пробрасывалась дальше
            self.ecs.world.remove_resource::<VirtualKeyCode>();
        }

        self.ecs.update();
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let mut context = BTermBuilder::new()
        .with_title("Ruinia")
        .with_fps_cap(120.0)
        .with_dimensions(DISPLAY_WIDTH * 2, DISPLAY_HEIGHT * 2)
        .with_tile_dimensions(8, 8)
        .with_resource_path("resources/")
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "terminal8x8.png") // Map layer
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "terminal8x8.png") // Entity render
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png") // GUI
        .build()?;

    // context.with_post_scanlines(true);
    main_loop(context, State::new())
}
