use bevy::prelude::*;
use rand::prelude::*;
use crate::fruits;
use crate::spawner::FruitSpawnPosition;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_input)
            .add_systems(Update,update_spawn_position);
    }
}

fn setup_input(mut input: ResMut<Input<KeyCode>>) {
    input.press(KeyCode::Space);
}


fn update_spawn_position(
    keyboard_input: Res<Input<KeyCode>>,
    mut spawn_position: ResMut<FruitSpawnPosition>
) {
    const MOVE_AMOUNT: f32 = 10.0;
    if keyboard_input.just_pressed(KeyCode::Left) {
        spawn_position.x -= MOVE_AMOUNT;
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        spawn_position.x += MOVE_AMOUNT
    }
}

