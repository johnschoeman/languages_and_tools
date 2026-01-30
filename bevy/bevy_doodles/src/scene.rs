use bevy::prelude::*;
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

// Initial rotation (matches CUBE_CONFIG in ui.rs)
const MAIN_CUBE_INITIAL_ROTATION: (f32, f32, f32) = (115.0, 0.0, -45.0);

// Color constants
const MAIN_CUBE_COLOR: (f32, f32, f32) = (0.8, 0.8, 0.8); // Light gray
const LEAF_CUBE_COLOR: (f32, f32, f32) = (0.7, 0.7, 0.7); // Medium light gray

// Light constants
const LIGHT_POSITION: (f32, f32, f32) = (-4.0, 6.0, 4.0);

// Camera constants
const CAMERA_POSITION: (f32, f32, f32) = (0.0, 2.5, 8.0);

#[derive(Resource)]
pub struct AutoRotation {
    pub enabled: bool,
}

impl Default for AutoRotation {
    fn default() -> Self {
        Self { enabled: false }
    }
}

#[derive(Resource)]
pub struct ColorAnimation {
    pub timer: f32,
    pub change_interval: f32,
    pub main_cube_hue: f32,
    pub leaf_cube_hue: f32,
    pub target_main_hue: f32,
    pub target_leaf_hue: f32,
}

impl Default for ColorAnimation {
    fn default() -> Self {
        Self {
            timer: 0.0,
            change_interval: 20.0, // Change colors every 20 seconds
            main_cube_hue: 0.6,    // Start with a blue-ish hue
            leaf_cube_hue: 0.8,    // Start with a purple-ish hue
            target_main_hue: 0.6,
            target_leaf_hue: 0.8,
        }
    }
}

// Convert HSL to RGB (attempt to create pleasant colors)
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = match (h * 6.0) as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    (r + m, g + m, b + m)
}

#[derive(Component)]
pub struct RotatingCube;

#[derive(Component)]
pub struct LeafCube;

#[derive(Component)]
pub struct SceneLight;

#[derive(Component)]
pub struct GroundPlane;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Main/central cube - this is the parent that everything rotates around
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE))),
        MeshMaterial3d(materials.add(Color::srgb(MAIN_CUBE_COLOR.0, MAIN_CUBE_COLOR.1, MAIN_CUBE_COLOR.2))),
        Transform::from_xyz(0.0, CUBE_Y_POSITION, 0.0)
            .with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                MAIN_CUBE_INITIAL_ROTATION.0.to_radians(),
                MAIN_CUBE_INITIAL_ROTATION.1.to_radians(),
                MAIN_CUBE_INITIAL_ROTATION.2.to_radians(),
            )),
        RotatingCube,
    )).with_children(|parent| {
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
        ));
    });

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(LIGHT_POSITION.0, LIGHT_POSITION.1, LIGHT_POSITION.2),
        SceneLight,
    ));

    // Ground plane to receive shadows
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0)))),
        MeshMaterial3d(materials.add(Color::srgb(0.9, 0.9, 0.9))),
        Transform::from_xyz(0.0, -2.0, 0.0),
        GroundPlane,
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

pub fn sync_main_rotation_to_inputs(
    auto_rotation: Res<AutoRotation>,
    main_query: Query<&Transform, With<RotatingCube>>,
    mut input_query: Query<(&InputField, &mut TextInput)>,
) {
    // Don't sync when autorotation is enabled to avoid jittery feedback loop
    if auto_rotation.enabled {
        return;
    }

    // Get the current rotation of the main cube
    let Some(transform) = main_query.iter().next() else {
        return;
    };

    // Convert quaternion to Euler angles (in radians)
    let (x, y, z) = transform.rotation.to_euler(EulerRot::XYZ);

    // Convert to degrees and update text inputs (only if not focused)
    for (field, mut input) in &mut input_query {
        if input.is_focused {
            continue; // Don't update while user is editing
        }

        let new_value = match field {
            InputField::MainRotationX => format!("{:.0}", x.to_degrees()),
            InputField::MainRotationY => format!("{:.0}", y.to_degrees()),
            InputField::MainRotationZ => format!("{:.0}", z.to_degrees()),
            _ => continue,
        };

        input.value = new_value;
    }
}

pub fn apply_main_rotation_from_inputs(
    changed_query: Query<&InputField, Changed<TextInput>>,
    input_query: Query<(&InputField, &TextInput)>,
    mut main_query: Query<&mut Transform, With<RotatingCube>>,
) {
    // Check if any main rotation input changed
    let mut has_main_rotation_change = false;
    for field in &changed_query {
        match field {
            InputField::MainRotationX | InputField::MainRotationY | InputField::MainRotationZ => {
                has_main_rotation_change = true;
                break;
            }
            _ => {}
        }
    }

    // Only apply if a main rotation input actually changed
    if !has_main_rotation_change {
        return;
    }

    // Collect rotation values from all inputs
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

pub fn apply_light_position_from_inputs(
    input_query: Query<(&InputField, &TextInput)>,
    mut light_query: Query<&mut Transform, With<SceneLight>>,
) {
    // Collect position values from inputs
    let mut pos_x = LIGHT_POSITION.0;
    let mut pos_y = LIGHT_POSITION.1;
    let mut pos_z = LIGHT_POSITION.2;

    for (field, input) in &input_query {
        let value = input.value.parse::<f32>().unwrap_or(0.0);
        match field {
            InputField::LightPositionX => pos_x = value,
            InputField::LightPositionY => pos_y = value,
            InputField::LightPositionZ => pos_z = value,
            _ => {}
        }
    }

    // Update light position
    for mut transform in &mut light_query {
        transform.translation = Vec3::new(pos_x, pos_y, pos_z);
    }
}

pub fn animate_cube_colors(
    time: Res<Time>,
    mut color_anim: ResMut<ColorAnimation>,
    mut clear_color: ResMut<ClearColor>,
    main_cube_query: Query<&MeshMaterial3d<StandardMaterial>, With<RotatingCube>>,
    leaf_cube_query: Query<&MeshMaterial3d<StandardMaterial>, With<LeafCube>>,
    ground_query: Query<&MeshMaterial3d<StandardMaterial>, With<GroundPlane>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Update timer
    color_anim.timer += time.delta_secs();

    // Pick new target colors when timer expires
    if color_anim.timer >= color_anim.change_interval {
        color_anim.timer = 0.0;
        // Use time-based pseudo-random for variety
        let seed = time.elapsed_secs();
        color_anim.target_main_hue = (seed * 0.1).fract();
        color_anim.target_leaf_hue = (seed * 0.13 + 0.5).fract();
    }

    // Smoothly interpolate current hues toward targets (very slow for subtle changes)
    let lerp_speed = time.delta_secs() * 0.05;
    color_anim.main_cube_hue = lerp_hue(color_anim.main_cube_hue, color_anim.target_main_hue, lerp_speed);
    color_anim.leaf_cube_hue = lerp_hue(color_anim.leaf_cube_hue, color_anim.target_leaf_hue, lerp_speed);

    // Convert HSL to RGB with pleasant saturation and lightness
    let main_rgb = hsl_to_rgb(color_anim.main_cube_hue, 0.5, 0.6); // Soft, pleasant colors
    let leaf_rgb = hsl_to_rgb(color_anim.leaf_cube_hue, 0.5, 0.5); // Slightly darker

    // Calculate complementary colors for background and ground
    // Use the average of the two cube hues, shifted for complement
    let avg_hue = (color_anim.main_cube_hue + color_anim.leaf_cube_hue) / 2.0;
    let complement_hue = (avg_hue + 0.5).fract(); // Opposite on color wheel

    // Background: dark, desaturated complement
    let bg_rgb = hsl_to_rgb(complement_hue, 0.15, 0.12);

    // Ground: light, slightly tinted with the complement
    let ground_rgb = hsl_to_rgb(complement_hue, 0.08, 0.85);

    // Update main cube color
    for material_handle in &main_cube_query {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.base_color = Color::srgb(main_rgb.0, main_rgb.1, main_rgb.2);
        }
    }

    // Update leaf cube color
    for material_handle in &leaf_cube_query {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.base_color = Color::srgb(leaf_rgb.0, leaf_rgb.1, leaf_rgb.2);
        }
    }

    // Update ground color
    for material_handle in &ground_query {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.base_color = Color::srgb(ground_rgb.0, ground_rgb.1, ground_rgb.2);
        }
    }

    // Update background color
    clear_color.0 = Color::srgb(bg_rgb.0, bg_rgb.1, bg_rgb.2);
}

// Lerp between hues, handling wrap-around (0.0 and 1.0 are the same hue)
fn lerp_hue(current: f32, target: f32, t: f32) -> f32 {
    let diff = target - current;
    // Handle wrap-around: if difference is more than 0.5, go the short way
    let adjusted_diff = if diff > 0.5 {
        diff - 1.0
    } else if diff < -0.5 {
        diff + 1.0
    } else {
        diff
    };
    (current + adjusted_diff * t).rem_euclid(1.0)
}
