use bevy::{
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
};
use bevy_mops::{merge_meshes, seperate, slice, GIMesh, MergeSettings};
use bevy_panorbit_camera::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, PanOrbitCameraPlugin, WireframePlugin));
    app.add_systems(Startup, setup)
        .add_systems(Update, (show_slice_gizmos, rotate_slicer).chain());
    app.run();
}

#[derive(Component)]
pub struct Slicee;

#[derive(Component)]
pub struct Slicer;

#[derive(Component)]
pub struct Output {
    inside: bool,
}

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

    commands.spawn((
        // meshes.add(Sphere::new(0.75)), //Cuboid::new(1.0, 1.0, 1.0)),
        meshes.add(Cuboid::new(0.5, 0.5, 0.5)),
        materials.add(StandardMaterial {
            base_color: Color::rgba(1.0, 0.0, 0.0, 1.0),
            // alpha_mode: AlphaMode::Blend,
            cull_mode: None,
            ..default()
        }),
        SpatialBundle {
            transform: Transform::from_xyz(0.5, 0.0, 0.0),
            ..default()
        },
        Slicee,
    ));
    commands.spawn((
        meshes.add(Cuboid::new(1.0, 1.0, 1.0)), //Plane3d::new(Vec3::Y)),
        materials.add(StandardMaterial {
            base_color: Color::rgba(1.0, 1.0, 1.0, 0.5),
            alpha_mode: AlphaMode::Blend,
            cull_mode: None,
            ..default()
        }),
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
            transform: Transform::from_xyz(-0.75, 2.0, 0.0),
            ..default()
        },
        Wireframe,
        Output { inside: false },
    ));
    commands.spawn((
        meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        materials.add(StandardMaterial {
            base_color: Color::rgb(1.0, 0.0, 0.0),
            // cull_mode: None,
            ..default()
        }),
        SpatialBundle {
            transform: Transform::from_xyz(0.75, 2.0, 0.0),
            ..default()
        },
        Wireframe,
        Output { inside: true },
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
    output_query: Query<(Entity, &Output)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let instant = std::time::Instant::now();

    let (slicee_transform, slicee_handle) = query.get(slicee_query.single()).unwrap();
    let (slicer_transform, slicer_handle) = query.get(slicer_query.single()).unwrap();

    let slicee_mesh = meshes.get(slicee_handle).unwrap();
    let slicer_mesh = meshes.get(slicer_handle).unwrap();

    let mut slicee_a = GIMesh::from_mesh(slicee_mesh, slicee_transform.affine()).unwrap();
    let slicer_a = GIMesh::from_mesh(slicer_mesh, slicer_transform.affine()).unwrap();

    let mut slicee_b = slicer_a.clone();
    let slicer_b = slicee_a.clone();

    slice(&mut slicee_a, &slicer_a);
    slice(&mut slicee_b, &slicer_b);

    let mut output_a = seperate(&slicee_a, &slicer_a);
    let mut output_b = seperate(&slicee_b, &slicer_b);

    merge_meshes(
        &mut output_a.outside,
        &output_a.inside,
        &MergeSettings::default(),
    );
    merge_meshes(
        &mut output_b.outside,
        &output_a.inside,
        &MergeSettings {
            invert_b_normals: true,
            ..default()
        },
    );

    let inside_handle = meshes.add(output_b.outside.to_mesh().unwrap());
    let outside_handle = meshes.add(output_a.outside.to_mesh().unwrap());
    for (entity, output) in output_query.iter() {
        commands.entity(entity).remove::<Handle<Mesh>>();
        let handle = if output.inside {
            inside_handle.clone()
        } else {
            outside_handle.clone()
        };
        commands.entity(entity).insert(handle);
    }

    println!("Took {}s", instant.elapsed().as_secs_f32());
}
