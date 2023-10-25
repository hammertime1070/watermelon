use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
mod fruits;
use fruits::{FruitType, spawn_fruit};
mod inputs;
mod spawner;


fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_physics)
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(inputs::InputPlugin)
        .add_plugins(spawner::SpawnerPlugin)
        .run();
}

struct Boundary {
    width: f32,
    height: f32,
}

struct Block;
struct Position {
    x: f32,
    y: f32,
}

struct Velocity {
    x: f32,
    y: f32,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands,
                 mut meshes: ResMut<Assets<Mesh>>,
                 mut materials: ResMut<Assets<ColorMaterial>>,
                ) {
    // setup floor
    let box_width: f32 = 500.0;
    let box_height: f32 = 300.0;
    let boundary_thickness: f32 = 50.0;
    // setup bottom boundary
    commands
        .spawn(Collider::cuboid(box_width / 2.0, boundary_thickness / 2.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -box_height / 2.0, 0.0)))
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(box_width, boundary_thickness)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -box_height / 2.0, 0.0),
            ..default()
        });

    // setup left boundary
    commands
        .spawn(Collider::cuboid(boundary_thickness / 2.0, (box_height - boundary_thickness) / 2.0))
        .insert(TransformBundle::from(Transform::from_xyz(-box_width / 2.0 + boundary_thickness / 2.0, 0.0, 0.0)))
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(boundary_thickness, box_height - boundary_thickness)),
                ..default()
            },
            transform: Transform::from_xyz(-box_width / 2.0 + boundary_thickness / 2.0, 0.0, 0.0),
            ..default()
        });

    // setup right boundary
    commands
        .spawn(Collider::cuboid(boundary_thickness / 2.0, (box_height - boundary_thickness) / 2.0))
        .insert(TransformBundle::from(Transform::from_xyz(box_width / 2.0 - boundary_thickness / 2.0, 0.0, 0.0)))
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(boundary_thickness, box_height - boundary_thickness)),
                ..default()
            },
            transform: Transform::from_xyz(box_width / 2.0 - boundary_thickness / 2.0, 0.0, 0.0),
            ..default()
        });
}

