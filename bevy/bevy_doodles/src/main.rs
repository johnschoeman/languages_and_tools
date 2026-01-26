use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_cube)
        .run();
}

#[derive(Component)]
struct RotatingCube;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Parent entity for rigid rotation
    commands.spawn((
        Transform::from_xyz(0.0, 0.5, 0.0),
        Visibility::default(),
        RotatingCube,
    )).with_children(|parent| {
        // First cube
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));

        // Second cube - rotated 45 degrees on X and Y, offset for 1/5 overlap
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.6, 0.7, 0.8))),
            Transform::from_xyz(0.8, 0.0, 0.0)
                .with_rotation(Quat::from_euler(EulerRot::XYZ, 45f32.to_radians(), 45f32.to_radians(), 0.0)),
        ));
    });

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn rotate_cube(time: Res<Time>, mut query: Query<&mut Transform, With<RotatingCube>>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs());
        transform.rotate_x(time.delta_secs() * 0.5);
    }
}
