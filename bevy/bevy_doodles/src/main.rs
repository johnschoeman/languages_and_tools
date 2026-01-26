use bevy::prelude::*;

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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<AutoRotation>()
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(Update, (rotate_cube, handle_button_interaction))
        .run();
}

#[derive(Resource)]
struct AutoRotation {
    enabled: bool,
}

#[derive(Component)]
struct RotatingCube;

#[derive(Component)]
enum RotationButton {
    Left,
    Right,
    Up,
    Down,
    ToggleAuto,
}

impl Default for AutoRotation {
    fn default() -> Self {
        Self { enabled: false }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Parent entity for rigid rotation
    commands.spawn((
        Transform::from_xyz(0.0, CUBE_Y_POSITION, 0.0),
        Visibility::default(),
        RotatingCube,
    )).with_children(|parent| {
        // First cube
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE))),
            MeshMaterial3d(materials.add(Color::srgb(FIRST_CUBE_COLOR.0, FIRST_CUBE_COLOR.1, FIRST_CUBE_COLOR.2))),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));

        // Second cube - rotated 45 degrees on X and Y, offset for 1/5 overlap
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
                    spawn_button(panel, "⬆ Up (W)", RotationButton::Up);
                    spawn_button(panel, "⬇ Down (S)", RotationButton::Down);
                    spawn_button(panel, "⬅ Left (A)", RotationButton::Left);
                    spawn_button(panel, "➡ Right (D)", RotationButton::Right);
                    spawn_button(panel, "⏯ Auto (Space)", RotationButton::ToggleAuto);
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
                _ => {
                    for mut transform in &mut cube_query {
                        match button_type {
                            RotationButton::Left => transform.rotate_y(BUTTON_ROTATION_AMOUNT),
                            RotationButton::Right => transform.rotate_y(-BUTTON_ROTATION_AMOUNT),
                            RotationButton::Up => transform.rotate_x(BUTTON_ROTATION_AMOUNT),
                            RotationButton::Down => transform.rotate_x(-BUTTON_ROTATION_AMOUNT),
                            RotationButton::ToggleAuto => {}
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

    let keyboard_delta = time.delta_secs() * KEYBOARD_ROTATION_SPEED;

    for mut transform in &mut query {
        // Automatic rotation
        if auto_rotation.enabled {
            transform.rotate_y(time.delta_secs() * AUTO_ROTATION_SPEED_Y);
            transform.rotate_x(time.delta_secs() * AUTO_ROTATION_SPEED_X);
        }

        // Keyboard controls (work alongside automatic rotation)
        if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
            transform.rotate_y(keyboard_delta);
        }
        if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
            transform.rotate_y(-keyboard_delta);
        }
        if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
            transform.rotate_x(keyboard_delta);
        }
        if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
            transform.rotate_x(-keyboard_delta);
        }
    }
}
