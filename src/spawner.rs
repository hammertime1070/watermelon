use bevy::prelude::*;
use rand::prelude::*;
use crate::fruits;

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_and_spawn_fruit)
            .insert_resource(FruitSpawnPosition{x: 0.0});
    }
}

#[derive(Resource)]
pub struct FruitSpawnPosition {
    pub x: f32,
}

fn check_and_spawn_fruit(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    spawn_position: Res<FruitSpawnPosition>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let fruit = random_fruit();
        fruits::spawn_fruit(fruit, &mut commands, &mut meshes, &mut materials, &spawn_position);
    }
}

fn random_fruit() -> fruits::FruitType {
    let fruits = [
        fruits::FruitType::Cherry,
        fruits::FruitType::Strawberry,
        fruits::FruitType::Grape,
        fruits::FruitType::Pomelo,
        fruits::FruitType::Orange,
    ];
    let mut rng = rand::thread_rng();
    *fruits.choose(&mut rng).unwrap()
}