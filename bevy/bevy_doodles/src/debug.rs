use bevy::{
    app::AppExit,
    prelude::*,
    render::view::screenshot::{save_to_disk, Screenshot},
};
use crate::scene::RotatingCube;

// Debug constants
const AXIS_LENGTH: f32 = 2.0;

// UI constants for debug panel
const UI_PADDING: f32 = 20.0;

#[derive(Resource)]
pub struct DebugMode {
    pub enabled: bool,
}

impl Default for DebugMode {
    fn default() -> Self {
        // Enable debug mode if AUTO_DEBUG env var is set
        let enabled = std::env::var("AUTO_DEBUG").is_ok();
        Self { enabled }
    }
}

#[derive(Component)]
pub struct DebugText;

pub fn setup_debug_ui(mut commands: Commands) {
    // Debug info panel (bottom-right corner)
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            right: Val::Px(UI_PADDING),
            bottom: Val::Px(UI_PADDING),
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("Debug: Press D"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                DebugText,
            ));
        });
}

pub fn toggle_debug_mode(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut debug_mode: ResMut<DebugMode>,
) {
    if keyboard.just_pressed(KeyCode::KeyD) {
        debug_mode.enabled = !debug_mode.enabled;
        info!("Debug mode: {}", if debug_mode.enabled { "ON" } else { "OFF" });
    }
}

pub fn draw_debug_axes(
    mut gizmos: Gizmos,
    debug_mode: Res<DebugMode>,
) {
    if !debug_mode.enabled {
        return;
    }

    // Draw coordinate axes using gizmos.axes() which shows labeled axes
    gizmos.axes(Transform::IDENTITY, AXIS_LENGTH);
}

pub fn update_debug_text(
    debug_mode: Res<DebugMode>,
    cube_query: Query<&Transform, With<RotatingCube>>,
    mut text_query: Query<(&mut Text, &mut Visibility), With<DebugText>>,
) {
    for (mut text, mut visibility) in &mut text_query {
        if debug_mode.enabled {
            *visibility = Visibility::Visible;

            if let Ok(transform) = cube_query.single() {
                let (axis, angle): (Vec3, f32) = transform.rotation.to_axis_angle();
                let euler = transform.rotation.to_euler(EulerRot::XYZ);

                **text = format!(
                    "Debug Mode (D)\n\
                    \n\
                    Axes: Red=X, Green=Y, Blue=Z\n\
                    \n\
                    Rotation (Euler XYZ):\n\
                    X: {:.1}°\n\
                    Y: {:.1}°\n\
                    Z: {:.1}°\n\
                    \n\
                    Axis-Angle:\n\
                    Axis: ({:.2}, {:.2}, {:.2})\n\
                    Angle: {:.1}°",
                    euler.0.to_degrees(),
                    euler.1.to_degrees(),
                    euler.2.to_degrees(),
                    axis.x, axis.y, axis.z,
                    angle.to_degrees()
                );
            }
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

pub fn screenshot_on_f12(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut counter: Local<u32>,
) {
    if keyboard.just_pressed(KeyCode::F12) {
        // Create tmp directory if it doesn't exist
        if let Err(e) = std::fs::create_dir_all("./tmp") {
            error!("Failed to create tmp directory: {}", e);
            return;
        }

        let path = format!("./tmp/bevy_screenshot_{}.png", *counter);
        *counter += 1;
        info!("Taking screenshot: {}", path);
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
    }
}

pub fn auto_screenshot(
    mut commands: Commands,
    time: Res<Time>,
    mut screenshot_taken: Local<bool>,
    mut exit: MessageWriter<AppExit>,
) {
    // Check if AUTO_SCREENSHOT env var is set
    if std::env::var("AUTO_SCREENSHOT").is_err() {
        return;
    }

    // Wait 1 second for scene to render, then take screenshot
    if !*screenshot_taken && time.elapsed_secs() > 1.0 {
        *screenshot_taken = true;

        if let Err(e) = std::fs::create_dir_all("./tmp") {
            error!("Failed to create tmp directory: {}", e);
            exit.write(AppExit::Error(std::num::NonZero::new(1).unwrap()));
            return;
        }

        let path = "./tmp/bevy_screenshot_auto.png";
        info!("Auto-screenshot mode: Taking screenshot to {}", path);

        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path.to_string()));
    }

    // Exit after screenshot has been saved (give it 1.5 seconds total)
    if *screenshot_taken && time.elapsed_secs() > 1.5 {
        info!("Exiting after auto-screenshot");
        exit.write(AppExit::Success);
    }
}
