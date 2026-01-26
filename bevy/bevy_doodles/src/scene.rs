use bevy::prelude::*;

// Rotation constants
const KEYBOARD_ROTATION_SPEED: f32 = 2.0;
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

#[derive(Resource)]
pub struct AutoRotation {
    pub enabled: bool,
}

impl Default for AutoRotation {
    fn default() -> Self {
        Self { enabled: false }
    }
}

#[derive(Component)]
pub struct RotatingCube;

pub fn setup(
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

pub fn rotate_cube(
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
