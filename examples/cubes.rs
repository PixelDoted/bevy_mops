use bevy::{
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
};
use bevy_mops::{GIMesh, MergeSettings};
use bevy_panorbit_camera::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, PanOrbitCameraPlugin, WireframePlugin));
    app.insert_resource(IsRunning(true));
    app.add_systems(Startup, setup)
        .add_systems(Update, (controls, show_slice_gizmos, rotate_mesh_b).chain());
    app.run();
}

#[derive(Resource)]
pub struct IsRunning(pub bool);

#[derive(Component)]
pub struct SliceA;

#[derive(Component)]
pub struct SliceB;

#[derive(Component)]
pub struct Output {
    pub index: usize,
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
        SliceA,
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
        SliceB,
    ));

    commands.spawn((
        meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        materials.add(StandardMaterial {
            base_color: Color::rgb(0.0, 0.0, 1.0),
            // cull_mode: None,
            ..default()
        }),
        SpatialBundle {
            transform: Transform::from_xyz(0.75, 2.0, 0.0),
            ..default()
        },
        Wireframe,
        Output { index: 0 },
    ));
    commands.spawn((
        meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        materials.add(StandardMaterial {
            base_color: Color::rgb(1.0, 0.0, 0.0),
            // cull_mode: None,
            ..default()
        }),
        SpatialBundle {
            transform: Transform::from_xyz(-0.75, 2.0, 0.0),
            ..default()
        },
        Wireframe,
        Output { index: 1 },
    ));
}

fn controls(keys: Res<ButtonInput<KeyCode>>, mut is_running: ResMut<IsRunning>) {
    if keys.just_pressed(KeyCode::Space) {
        is_running.0 = !is_running.0;
    }
}

fn rotate_mesh_b(
    time: Res<Time>,
    is_running: Res<IsRunning>,
    mut query: Query<&mut Transform, With<SliceB>>,
) {
    if !is_running.0 {
        return;
    }

    for mut transform in query.iter_mut() {
        let dt = time.delta_seconds() * 0.1;
        transform.rotate_x(dt);
        transform.rotate_y(dt * 0.9);
        transform.rotate_z(dt * 1.1);
    }
}

fn show_slice_gizmos(
    is_running: Res<IsRunning>,
    query: Query<(&GlobalTransform, &Handle<Mesh>)>,
    a_query: Query<Entity, With<SliceA>>,
    b_query: Query<Entity, With<SliceB>>,
    output_query: Query<(Entity, &Output)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if !is_running.0 {
        return;
    }

    let instant = std::time::Instant::now();

    let (a_transform, a_handle) = query.get(a_query.single()).unwrap();
    let (b_transform, b_handle) = query.get(b_query.single()).unwrap();

    let a_mesh = meshes.get(a_handle).unwrap();
    let b_mesh = meshes.get(b_handle).unwrap();

    let mut slicee_a = GIMesh::from_mesh(a_mesh, a_transform.affine()).unwrap();
    let slicer_b = GIMesh::from_mesh(b_mesh, b_transform.affine()).unwrap();
    let mut slicee_b = slicer_b.clone();
    let slicer_a = slicee_a.clone();

    slicee_a.slice(&slicer_b);
    slicee_b.slice(&slicer_a);

    let mut output_a = slicee_a.seperate(&slicer_b); // seperate the triangles in `A` to inside and outside of `B`
    let mut output_b = slicee_b.seperate(&slicer_a); // sperate the triangles in `B` to inside and outside of `A`

    // Merge the difference of `B` with the intersection of `A`
    output_b.outside.merge_with(
        &output_a.inside,
        &MergeSettings {
            invert_b_normals: true,
            ..default()
        },
    );

    // Merge the intersection of `A` with the intersection of `B`
    output_a
        .inside
        .merge_with(&output_b.inside, &MergeSettings::default());

    // Cleanup duplicate vertices created by slice and seperate
    let output_a = output_a
        .inside
        .merge_vertices(bevy_mops::DEFAULT_VERTEX_MERGE_DISTANCE);
    let output_b = output_b
        .outside
        .merge_vertices(bevy_mops::DEFAULT_VERTEX_MERGE_DISTANCE);

    let handles = [
        meshes.add(output_a.to_mesh().unwrap()),
        meshes.add(output_b.to_mesh().unwrap()),
    ];
    for (entity, output) in output_query.iter() {
        if handles.len() <= output.index {
            continue;
        }

        let handle = handles[output.index].clone();
        commands.entity(entity).remove::<Handle<Mesh>>();
        commands.entity(entity).insert(handle);
    }

    println!("Took {}s", instant.elapsed().as_secs_f32());
}
