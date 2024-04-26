use bevy::prelude::*;
use bevy_mops::{slice, GIMesh};
use bevy_panorbit_camera::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, PanOrbitCameraPlugin));
    app.add_systems(Startup, setup)
        .add_systems(Update, (show_slice_gizmos, rotate_slicer).chain());
    app.run();
}

#[derive(Component)]
pub struct Slicee;

#[derive(Component)]
pub struct Slicer;

#[derive(Component)]
pub struct Output;

fn setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0),
            ..default()
        },
        PanOrbitCamera::default(),
    ));

    let mat = materials.add(StandardMaterial {
        base_color: Color::rgba(1.0, 1.0, 1.0, 0.5),
        alpha_mode: AlphaMode::Blend,
        cull_mode: None,
        ..default()
    });
    commands.spawn((
        meshes.add(Sphere::new(0.75)), //Cuboid::new(1.0, 1.0, 1.0)),
        mat.clone(),
        SpatialBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Slicee,
    ));
    commands.spawn((
        meshes.add(Cuboid::new(1.0, 1.0, 1.0)), //Plane3d::new(Vec3::Y)),
        mat.clone(),
        SpatialBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Slicer,
    ));

    commands.spawn((
        meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        materials.add(StandardMaterial {
            base_color: Color::rgb(0.0, 0.0, 1.0),
            // cull_mode: None,
            ..default()
        }),
        SpatialBundle {
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..default()
        },
        Output,
    ));
}

fn rotate_slicer(time: Res<Time>, mut query: Query<&mut Transform, With<Slicer>>) {
    for mut transform in query.iter_mut() {
        let dt = time.delta_seconds() * 0.1;
        transform.rotate_x(dt);
        transform.rotate_y(dt * 0.9);
        transform.rotate_z(dt * 1.1);
    }
}

fn show_slice_gizmos(
    query: Query<(&GlobalTransform, &Handle<Mesh>)>,
    slicee_query: Query<Entity, With<Slicee>>,
    slicer_query: Query<Entity, With<Slicer>>,
    output_query: Query<Entity, With<Output>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let instant = std::time::Instant::now();

    let (slicee_transform, slicee_handle) = query.get(slicee_query.single()).unwrap();
    let (slicer_transform, slicer_handle) = query.get(slicer_query.single()).unwrap();

    let slicee_mesh = meshes.get(slicee_handle).unwrap();
    let slicer_mesh = meshes.get(slicer_handle).unwrap();

    let mut slicee = GIMesh::from_mesh(slicer_mesh, slicer_transform.affine()).unwrap();
    let slicer = GIMesh::from_mesh(slicee_mesh, slicee_transform.affine()).unwrap();
    slice(&mut slicee, &slicer);

    let output_handle = meshes.add(slicee.to_mesh().unwrap());
    for entity in output_query.iter() {
        commands.entity(entity).remove::<Handle<Mesh>>();
        commands.entity(entity).insert(output_handle.clone());
    }

    println!("Took {}s", instant.elapsed().as_secs_f32());
}
