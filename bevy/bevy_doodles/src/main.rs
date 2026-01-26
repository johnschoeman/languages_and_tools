use bevy::{
    app::AppExit,
    color::palettes::css,
    prelude::*,
    render::view::screenshot::{save_to_disk, Screenshot},
};

// Rotation constants
const KEYBOARD_ROTATION_SPEED: f32 = 2.0;
const BUTTON_ROTATION_AMOUNT: f32 = 0.1;
const AUTO_ROTATION_SPEED_Y: f32 = 1.0;
const AUTO_ROTATION_SPEED_X: f32 = 0.5;

// Cube constants
const CUBE_SIZE: f32 = 1.0;
const CUBE_Y_POSITION: f32 = 0.5;
const SECOND_CUBE_X_OFFSET: f32 = 0.8;
const SECOND_CUBE_ROTATION_DEGREES: f32 = 45.0;

// Color constants
const FIRST_CUBE_COLOR: (f32, f32, f32) = (0.8, 0.7, 0.6);
const SECOND_CUBE_COLOR: (f32, f32, f32) = (0.6, 0.7, 0.8);

// Light constants
const LIGHT_POSITION: (f32, f32, f32) = (4.0, 8.0, 4.0);

// Camera constants
const CAMERA_POSITION: (f32, f32, f32) = (-2.5, 4.5, 9.0);

// UI constants
const UI_PADDING: f32 = 20.0;
const BUTTON_SPACING: f32 = 10.0;
const BUTTON_WIDTH: f32 = 150.0;
const BUTTON_HEIGHT: f32 = 50.0;
const BUTTON_FONT_SIZE: f32 = 18.0;
const BUTTON_BG_COLOR: (f32, f32, f32) = (0.15, 0.15, 0.15);
const BUTTON_TEXT_COLOR: (f32, f32, f32) = (0.9, 0.9, 0.9);

// Debug constants
const AXIS_LENGTH: f32 = 2.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<AutoRotation>()
        .init_resource::<DebugMode>()
        .add_systems(Startup, (setup, setup_ui, setup_debug_ui))
        .add_systems(Update, (rotate_cube, handle_button_interaction, screenshot_on_f12))
        .add_systems(Update, (toggle_debug_mode, draw_debug_axes, update_debug_text))
        .add_systems(Update, auto_screenshot)
        .run();
}

#[derive(Resource)]
struct AutoRotation {
    enabled: bool,
}

#[derive(Resource)]
struct DebugMode {
    enabled: bool,
}

#[derive(Component)]
struct RotatingCube;

#[derive(Component)]
enum RotationButton {
    ToggleAuto,
    Reset,
    PlusX,
    MinusX,
    PlusY,
    MinusY,
    PlusZ,
    MinusZ,
}

impl Default for AutoRotation {
    fn default() -> Self {
        Self { enabled: false }
    }
}

impl Default for DebugMode {
    fn default() -> Self {
        // Enable debug mode if AUTO_DEBUG env var is set
        let enabled = std::env::var("AUTO_DEBUG").is_ok();
        Self { enabled }
    }
}

#[derive(Component)]
struct DebugText;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Main/central cube - this is the parent that everything rotates around
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE))),
        MeshMaterial3d(materials.add(Color::srgb(FIRST_CUBE_COLOR.0, FIRST_CUBE_COLOR.1, FIRST_CUBE_COLOR.2))),
        Transform::from_xyz(0.0, CUBE_Y_POSITION, 0.0),
        RotatingCube,
    )).with_children(|parent| {
        // Second cube - attached as child, rotated 45 degrees on X and Y, offset for 1/5 overlap
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE))),
            MeshMaterial3d(materials.add(Color::srgb(SECOND_CUBE_COLOR.0, SECOND_CUBE_COLOR.1, SECOND_CUBE_COLOR.2))),
            Transform::from_xyz(SECOND_CUBE_X_OFFSET, 0.0, 0.0)
                .with_rotation(Quat::from_euler(
                    EulerRot::XYZ,
                    SECOND_CUBE_ROTATION_DEGREES.to_radians(),
                    SECOND_CUBE_ROTATION_DEGREES.to_radians(),
                    0.0
                )),
        ));
    });

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(LIGHT_POSITION.0, LIGHT_POSITION.1, LIGHT_POSITION.2),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(CAMERA_POSITION.0, CAMERA_POSITION.1, CAMERA_POSITION.2)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn setup_ui(mut commands: Commands) {
    // Root UI container
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(Val::Px(UI_PADDING)),
            ..default()
        })
        .with_children(|parent| {
            // Left side button panel
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(BUTTON_SPACING),
                    ..default()
                })
                .with_children(|panel| {
                    spawn_button(panel, "⏯ Auto (Space)", RotationButton::ToggleAuto);
                    spawn_button(panel, "↺ Reset (R)", RotationButton::Reset);
                });

            // Right side button panel for individual axis rotation
            parent
                .spawn(Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(UI_PADDING),
                    top: Val::Px(UI_PADDING),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(BUTTON_SPACING),
                    ..default()
                })
                .with_children(|panel| {
                    spawn_button(panel, "+X (J)", RotationButton::PlusX);
                    spawn_button(panel, "-X (U)", RotationButton::MinusX);
                    spawn_button(panel, "+Y (K)", RotationButton::PlusY);
                    spawn_button(panel, "-Y (I)", RotationButton::MinusY);
                    spawn_button(panel, "+Z (L)", RotationButton::PlusZ);
                    spawn_button(panel, "-Z (O)", RotationButton::MinusZ);
                });
        });
}

fn spawn_button(parent: &mut ChildSpawnerCommands, text: &str, button_type: RotationButton) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(BUTTON_WIDTH),
                height: Val::Px(BUTTON_HEIGHT),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(BUTTON_BG_COLOR.0, BUTTON_BG_COLOR.1, BUTTON_BG_COLOR.2)),
            button_type,
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(text),
                TextFont {
                    font_size: BUTTON_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(BUTTON_TEXT_COLOR.0, BUTTON_TEXT_COLOR.1, BUTTON_TEXT_COLOR.2)),
            ));
        });
}

fn handle_button_interaction(
    interaction_query: Query<(&Interaction, &RotationButton), Changed<Interaction>>,
    mut cube_query: Query<&mut Transform, With<RotatingCube>>,
    mut auto_rotation: ResMut<AutoRotation>,
) {
    for (interaction, button_type) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button_type {
                RotationButton::ToggleAuto => {
                    auto_rotation.enabled = !auto_rotation.enabled;
                }
                RotationButton::Reset => {
                    for mut transform in &mut cube_query {
                        transform.rotation = Quat::IDENTITY;
                    }
                }
                _ => {
                    for mut transform in &mut cube_query {
                        match button_type {
                            RotationButton::PlusX => transform.rotate_local_x(BUTTON_ROTATION_AMOUNT),
                            RotationButton::MinusX => transform.rotate_local_x(-BUTTON_ROTATION_AMOUNT),
                            RotationButton::PlusY => transform.rotate_local_y(BUTTON_ROTATION_AMOUNT),
                            RotationButton::MinusY => transform.rotate_local_y(-BUTTON_ROTATION_AMOUNT),
                            RotationButton::PlusZ => transform.rotate_local_z(BUTTON_ROTATION_AMOUNT),
                            RotationButton::MinusZ => transform.rotate_local_z(-BUTTON_ROTATION_AMOUNT),
                            RotationButton::ToggleAuto | RotationButton::Reset => {}
                        }
                    }
                }
            }
        }
    }
}

fn rotate_cube(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut auto_rotation: ResMut<AutoRotation>,
    mut query: Query<&mut Transform, With<RotatingCube>>,
) {
    // Toggle auto-rotation with space bar
    if keyboard.just_pressed(KeyCode::Space) {
        auto_rotation.enabled = !auto_rotation.enabled;
    }

    // Reset rotation with R key
    if keyboard.just_pressed(KeyCode::KeyR) {
        for mut transform in &mut query {
            transform.rotation = Quat::IDENTITY;
        }
        return;
    }

    let keyboard_delta = time.delta_secs() * KEYBOARD_ROTATION_SPEED;

    for mut transform in &mut query {
        // Automatic rotation
        if auto_rotation.enabled {
            transform.rotate_y(time.delta_secs() * AUTO_ROTATION_SPEED_Y);
            transform.rotate_x(time.delta_secs() * AUTO_ROTATION_SPEED_X);
        }

        // Individual axis controls
        if keyboard.pressed(KeyCode::KeyJ) {
            transform.rotate_local_x(keyboard_delta);
        }
        if keyboard.pressed(KeyCode::KeyU) {
            transform.rotate_local_x(-keyboard_delta);
        }
        if keyboard.pressed(KeyCode::KeyK) {
            transform.rotate_local_y(keyboard_delta);
        }
        if keyboard.pressed(KeyCode::KeyI) {
            transform.rotate_local_y(-keyboard_delta);
        }
        if keyboard.pressed(KeyCode::KeyL) {
            transform.rotate_local_z(keyboard_delta);
        }
        if keyboard.pressed(KeyCode::KeyO) {
            transform.rotate_local_z(-keyboard_delta);
        }
    }
}

fn setup_debug_ui(mut commands: Commands) {
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

fn toggle_debug_mode(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut debug_mode: ResMut<DebugMode>,
) {
    if keyboard.just_pressed(KeyCode::KeyD) {
        debug_mode.enabled = !debug_mode.enabled;
        info!("Debug mode: {}", if debug_mode.enabled { "ON" } else { "OFF" });
    }
}

fn draw_debug_axes(
    mut gizmos: Gizmos,
    debug_mode: Res<DebugMode>,
) {
    if !debug_mode.enabled {
        return;
    }

    // Draw coordinate axes
    // X-axis (Red)
    gizmos.line(Vec3::ZERO, Vec3::X * AXIS_LENGTH, css::RED);
    // Y-axis (Green)
    gizmos.line(Vec3::ZERO, Vec3::Y * AXIS_LENGTH, css::GREEN);
    // Z-axis (Blue)
    gizmos.line(Vec3::ZERO, Vec3::Z * AXIS_LENGTH, css::BLUE);
}

fn update_debug_text(
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

fn screenshot_on_f12(
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

fn auto_screenshot(
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
