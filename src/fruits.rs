use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::spawner::FruitSpawnPosition;

#[derive(Copy, Clone, PartialEq)]
pub enum FruitType {
    Cherry,
    Strawberry,
    Grape,
    Pomelo,
    Orange,
}

#[derive(Component)]
pub struct Fruit {
    pub fruit_type: FruitType,
}

pub struct FruitTemplate {
    pub fruit_type: FruitType,
    pub radius: f32,
    pub color: Color,
}

impl FruitTemplate {
    pub fn from_type(fruit_type: FruitType) -> Self {
        match fruit_type {
            FruitType::Cherry => FruitTemplate {
                fruit_type,
                radius: 10.0,
                color: Color::RED,
            },
            FruitType::Strawberry => FruitTemplate {
                fruit_type,
                radius: 20.0,
                color: Color::PINK,
            },
            FruitType::Grape => FruitTemplate {
                fruit_type,
                radius: 30.0,
                color: Color::PURPLE,
            },
            FruitType::Pomelo => FruitTemplate {
                fruit_type,
                radius: 40.0,
                color: Color::ORANGE,
            },
            FruitType::Orange => FruitTemplate {
                fruit_type,
                radius: 50.0,
                color: Color::BLUE,
            },      
        }
    }
}


pub fn spawn_fruit(
    fruit_type: FruitType,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    spawn_position: &FruitSpawnPosition
) {
    let template = FruitTemplate::from_type(fruit_type);
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(template.radius).into()).into(),
            material: materials.add(ColorMaterial::from(template.color)),
            transform: Transform::from_translation(Vec3::new(spawn_position.x, 400.0, 0.0)),
            ..default()
        })
        .insert(Fruit { fruit_type })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(template.radius))
        .insert(Restitution::coefficient(0.7));
}

fn handle_fruit_combinations(
    mut commands: Commands,
    collision_events: EventReader<CollisionEvent>,
    mut fruit_query: Query<(Entity, &Fruit)>,
) {
    for collision_event in collision_events.iter() {
        // Ensure it's not from removed entities of sensors
        if collision_event.flags == CollisionEventFlags::REMOVED
            || collision_event.flags.contains(CollisionEventFlags::SENSOR) {
                continue;
            }
            let entity_a = collision_event.collider1.entity();
            let entity_b = collision_event.collider2.entity();

            if let (Some((entity_a, fruit_a)), Some((entity_b, fruit_b))) = (
                fruit_query.get(entity_a).ok(),
                fruit_query.get(entity_b).ok(),
            ) {
                // Logic to combine fruits and despawn old ones
                combine_fruits(&mut commands, entity_a, fruit_a, entity_b, fruit_b);
            }
    }
}

fn combine_fruits(
    commands: &mut Commands,
    entity_a: Entity, 
    fruit_a: &Fruit, 
    entity_b: Entity, 
    fruit_b: &Fruit,
    transform_query: &Query<&Transform>  // to get the position of the entities
) {
    if fruit_a.fruit_type != fruit_b.fruit_type {
        // Fruits are not of the same type, so don't combine.
        return;
    }

    // Calculate the average position of the two fruits.
    let transform_a = transform_query.get(entity_a).expect("Expected entity to have a Transform");
    let transform_b = transform_query.get(entity_b).expect("Expected entity to have a Transform");
    let avg_position = (transform_a.translation + transform_b.translation) * 0.5;

    let new_fruit_type = match fruit_a {
        Fruit::Cherry => Some(Fruit::Strawberry),
        Fruit::Strawberry => Some(Fruit::Grape),
        Fruit::Grape => Some(Fruit::Pomelo),
        Fruit::Pomelo => Some(Fruit::Orange),
        // Add more combinations here.
        // If we reached the last type in the sequence, we might not want to spawn a new fruit.
        Fruit::Orange => None,
    };

    if let Some(fruit_type) = new_fruit_type {
        // Despawn the old fruits.
        commands.entity(entity_a).despawn();
        commands.entity(entity_b).despawn();

        // Spawn the new fruit at the average position.
        spawn_fruit(commands, fruit_type, avg_position);
    }
}