use bevy::prelude::*;
use bevy::pbr::wireframe::{WireframePlugin, WireframeConfig};
use bevy::render::{
    render_resource::WgpuFeatures,
    settings::{RenderCreation, WgpuSettings},
    RenderPlugin,
};

mod scene;
mod ui;
mod debug;
mod text_input;

use scene::{AutoRotation, setup as setup_scene, rotate_cube, apply_leaf_rotation_from_inputs, apply_main_rotation_from_inputs};
use ui::{setup_ui, handle_button_interaction, UiVisibility, toggle_ui_visibility, update_ui_visibility};
use debug::{
    DebugMode, setup_debug_ui, toggle_debug_mode, draw_debug_axes,
    update_debug_text, screenshot_on_f12, auto_screenshot,
};
use text_input::{
    InputFocusState, handle_text_input_focus, handle_keyboard_input,
    update_cursor_blink, update_text_input_display,
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(WireframePlugin::default())
        .insert_resource(WireframeConfig {
            global: false,
            default_color: Color::BLACK,
        })
        .init_resource::<AutoRotation>()
        .init_resource::<DebugMode>()
        .init_resource::<InputFocusState>()
        .init_resource::<UiVisibility>()
        .add_systems(Startup, (setup_scene, setup_ui, setup_debug_ui))
        .add_systems(Update, (rotate_cube, handle_button_interaction, screenshot_on_f12))
        .add_systems(Update, (toggle_debug_mode, draw_debug_axes, update_debug_text))
        .add_systems(Update, (toggle_ui_visibility, update_ui_visibility))
        .add_systems(Update, (
            handle_text_input_focus,
            handle_keyboard_input,
            update_cursor_blink,
            update_text_input_display,
        ))
        .add_systems(Update, apply_leaf_rotation_from_inputs)
        .add_systems(Update, apply_main_rotation_from_inputs)
        .add_systems(Update, auto_screenshot)
        .run();
}
