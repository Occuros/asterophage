mod building;
mod debug;
mod general;
mod player;
mod save_and_load;
pub mod utilities;
mod world_grid;

use crate::building::BuildingPlugin;
use crate::debug::SmallDebugPlugin;
use crate::general::GeneralPlugin;
use crate::player::PlayerPlugin;
use crate::save_and_load::SaveLoadAsterophagePlugin;
use crate::world_grid::WorldGridPlugin;
use avian3d::prelude::*;
use bevy::log::{tracing_subscriber, LogPlugin};
use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;
use bevy_turborand::prelude::*;
use bevy_vector_shapes::prelude::*;
use dotenv::dotenv;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::Layer;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    MainMenu,
    #[default]
    Game,
    GameOver,
}

#[derive(Component, Debug, Clone)]
pub struct MainCamera {}

fn main() {
    dotenv().ok();

    let log_plugin = LogPlugin {
        filter: "info".into(),
        level: bevy::log::Level::INFO,
        custom_layer: |app: &mut App| {
            let subscriber = tracing_subscriber::fmt::layer()
                .with_span_events(FmtSpan::FULL)
                .with_file(true) // Include file paths
                .with_line_number(true) // Include line numbers
                .boxed();
            Some(subscriber)
        },
    };
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(BillboardPlugin)
        // .add_plugins(EditorPlugin::default())
        .add_plugins(ShapePlugin::default())
        .add_plugins(RngPlugin::new().with_rng_seed(135))
        // .add_plugins(bevy_framepace::FramepacePlugin)
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_plugins(GeneralPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(WorldGridPlugin)
        .add_plugins(BuildingPlugin)
        .add_plugins(SmallDebugPlugin)
        .add_plugins(SaveLoadAsterophagePlugin)
        // .insert_resource(Time::<Fixed>::from_hz(60.0))
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rng: ResMut<GlobalRng>,
) {
    let cube_mesh = meshes.add(Cuboid::default());

    // plane
    commands.spawn((
        Mesh3d(cube_mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::srgb(0.3, 0.5, 0.3)))),
        Transform {
            translation: Vec3::new(0.0, -0.005, 0.0),
            scale: Vec3::new(100.0, 0.01, 100.0),
            ..default()
        },
        Collider::cuboid(1.0, 1.0, 1.0),
        RigidBody::Static,
        Name::new("Floor"),
    ));

    let cube_mesh = meshes.add(Cuboid::default());
    //
    // commands.spawn((
    //     Mesh3d(cube_mesh.clone()),
    //     MeshMaterial3d(materials.add(Color::srgb(0.7, 0.7, 0.8))),
    //     Transform::from_xyz(0.0, -2.0, 0.0).with_scale(Vec3::new(100.0, 1.0, 100.0)),
    //     RigidBody::Static,
    //     Collider::cuboid(1.0, 1.0, 1.0),
    // ));

    for _i in 0..30 {
        let size = 0.25;
        let max_position = 20.0;
        let position = Vec3::new(
            rng.f32_normalized() * max_position,
            size * 0.5 + 10.0,
            rng.f32_normalized() * max_position,
        );

        let cube_mesh = meshes.add(Mesh::from(Cuboid {
            half_size: Vec3::splat(size),
        }));

        let collider_size = size * 0.5;
        // cube
        let _ = commands
            .spawn((
                Mesh3d(cube_mesh.clone()),
                MeshMaterial3d(materials.add(Color::srgb(0.8, rng.f32(), 0.6))),
                Transform::from_translation(position),
                RigidBody::Dynamic,
                Collider::cuboid(collider_size, collider_size, collider_size),
                Name::new("cube"),
                Mass(10.0),
            ))
            .id();

        // commands.entity(cubes).push_children(&[cube]);
    }

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1_000_000.0 * 0.5,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 20.0, 4.0),
        ..default()
    });

    let _eye = Vec3::new(-0.2, 2.5, 5.0);
    let _target = Vec3::default();

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        MainCamera {},
    ));
}
