mod building;
mod debug;
mod general;
mod player;
pub mod utilities;
mod world_grid;

use crate::building::BuildingPlugin;
use crate::debug::SmallDebugPlugin;
use crate::general::GeneralPlugin;
use crate::player::PlayerPlugin;
use crate::world_grid::WorldGridPlugin;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_mod_billboard::prelude::*;
use bevy_turborand::prelude::*;
use bevy_vector_shapes::prelude::*;
use dotenv::dotenv;

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

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(BillboardPlugin)
        .add_plugins(EditorPlugin::default())
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
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rng: ResMut<GlobalRng>,
) {
    // plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(1000.0, 1000.0)),
            material: materials.add(StandardMaterial::from(Color::rgb(0.3, 0.5, 0.3))),
            ..default()
        },
        Collider::cuboid(1000.0, 0.01, 1000.0),
        RigidBody::Static,
        Name::new("Floor"),
    ));

    for _i in 0..30 {
        let size = 0.25;
        let max_position = 20.0;
        let position = Vec3::new(
            rng.f32_normalized() * max_position,
            size * 0.5 + 10.0,
            rng.f32_normalized() * max_position,
        );

        // cube
        let _ = commands
            .spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(Cuboid {
                        half_size: Vec3::splat(size),
                    })),
                    material: materials.add(Color::rgb(0.8, rng.f32(), 0.6)),
                    transform: Transform::from_translation(position),
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::cuboid(size * 0.5, size * 0.5, size * 0.5),
                Friction::new(0.9)
                    .with_dynamic_coefficient(0.9)
                    .with_combine_rule(CoefficientCombine::Max),
                Name::new("cube"),
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
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MainCamera {},
    ));
}
