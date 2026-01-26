use bevy::prelude::*;
use bevy::pbr::wireframe::{Wireframe, WireframeColor};
use crate::text_input::{TextInput, InputField};

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
const MAIN_CUBE_COLOR: (f32, f32, f32) = (0.7, 0.7, 0.7); // Light gray
const LEAF_CUBE_COLOR: (f32, f32, f32) = (0.4, 0.4, 0.4); // Darker gray
const WIREFRAME_COLOR: (f32, f32, f32) = (0.1, 0.1, 0.1); // Dark edge color

// Wireframe constants
const WIREFRAME_SCALE: f32 = 1.002; // Slightly larger to be visible over solid mesh

// Light constants
const LIGHT_POSITION: (f32, f32, f32) = (-4.0, 6.0, 4.0);

// Camera constants
const CAMERA_POSITION: (f32, f32, f32) = (0.0, 0.0, 8.0);

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

#[derive(Component)]
pub struct LeafCube;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Main/central cube - this is the parent that everything rotates around
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE))),
        MeshMaterial3d(materials.add(Color::srgb(MAIN_CUBE_COLOR.0, MAIN_CUBE_COLOR.1, MAIN_CUBE_COLOR.2))),
        Transform::from_xyz(0.0, CUBE_Y_POSITION, 0.0),
        RotatingCube,
    )).with_children(|parent| {
        // Wireframe edges for main cube
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE))),
            Transform::from_scale(Vec3::splat(WIREFRAME_SCALE)),
            Wireframe,
            WireframeColor {
                color: Color::srgb(WIREFRAME_COLOR.0, WIREFRAME_COLOR.1, WIREFRAME_COLOR.2),
            },
        ));

        // Leaf cube - attached to main cube
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE))),
            MeshMaterial3d(materials.add(Color::srgb(LEAF_CUBE_COLOR.0, LEAF_CUBE_COLOR.1, LEAF_CUBE_COLOR.2))),
            Transform::from_xyz(SECOND_CUBE_X_OFFSET, 0.0, 0.0)
                .with_rotation(Quat::from_euler(
                    EulerRot::XYZ,
                    SECOND_CUBE_ROTATION_DEGREES.to_radians(),
                    SECOND_CUBE_ROTATION_DEGREES.to_radians(),
                    0.0
                )),
            LeafCube,
        )).with_children(|leaf_parent| {
            // Wireframe edges for leaf cube
            leaf_parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE))),
                Transform::from_scale(Vec3::splat(WIREFRAME_SCALE)),
                Wireframe,
                WireframeColor {
                    color: Color::srgb(WIREFRAME_COLOR.0, WIREFRAME_COLOR.1, WIREFRAME_COLOR.2),
                },
            ));
        });
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
            .looking_at(Vec3::new(0.0, CUBE_Y_POSITION, 0.0), Vec3::Y),
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

pub fn apply_leaf_rotation_from_inputs(
    input_query: Query<(&InputField, &TextInput)>,
    parent_query: Query<&Children, With<RotatingCube>>,
    mut leaf_query: Query<&mut Transform, With<LeafCube>>,
) {
    // Collect rotation and translation values from inputs
    let mut rot_x = 0.0;
    let mut rot_y = 0.0;
    let mut rot_z = 0.0;
    let mut trans_x = 0.0;
    let mut trans_y = 0.0;
    let mut trans_z = 0.0;

    for (field, input) in &input_query {
        let value = input.value.parse::<f32>().unwrap_or(0.0);
        match field {
            InputField::LeafRotationX => rot_x = value,
            InputField::LeafRotationY => rot_y = value,
            InputField::LeafRotationZ => rot_z = value,
            InputField::LeafTranslationX => trans_x = value,
            InputField::LeafTranslationY => trans_y = value,
            InputField::LeafTranslationZ => trans_z = value,
            _ => {}
        }
    }

    // Find the leaf cube and update its rotation and translation
    for children in &parent_query {
        for child in children.iter() {
            if let Ok(mut transform) = leaf_query.get_mut(child) {
                transform.rotation = Quat::from_euler(
                    EulerRot::XYZ,
                    rot_x.to_radians(),
                    rot_y.to_radians(),
                    rot_z.to_radians(),
                );
                transform.translation = Vec3::new(trans_x, trans_y, trans_z);
            }
        }
    }
}

pub fn apply_main_rotation_from_inputs(
    input_query: Query<(&InputField, &TextInput)>,
    mut main_query: Query<&mut Transform, With<RotatingCube>>,
) {
    // Collect rotation values from inputs
    let mut rot_x = 0.0;
    let mut rot_y = 0.0;
    let mut rot_z = 0.0;

    for (field, input) in &input_query {
        let value = input.value.parse::<f32>().unwrap_or(0.0);
        match field {
            InputField::MainRotationX => rot_x = value,
            InputField::MainRotationY => rot_y = value,
            InputField::MainRotationZ => rot_z = value,
            _ => {}
        }
    }

    // Update main cube rotation
    for mut transform in &mut main_query {
        transform.rotation = Quat::from_euler(
            EulerRot::XYZ,
            rot_x.to_radians(),
            rot_y.to_radians(),
            rot_z.to_radians(),
        );
    }
}
