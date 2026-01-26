use bevy::prelude::*;

mod scene;
mod ui;
mod debug;

use scene::{AutoRotation, setup as setup_scene, rotate_cube};
use ui::{setup_ui, handle_button_interaction};
use debug::{
    DebugMode, setup_debug_ui, toggle_debug_mode, draw_debug_axes,
    update_debug_text, screenshot_on_f12, auto_screenshot,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<AutoRotation>()
        .init_resource::<DebugMode>()
        .add_systems(Startup, (setup_scene, setup_ui, setup_debug_ui))
        .add_systems(Update, (rotate_cube, handle_button_interaction, screenshot_on_f12))
        .add_systems(Update, (toggle_debug_mode, draw_debug_axes, update_debug_text))
        .add_systems(Update, auto_screenshot)
        .run();
}
