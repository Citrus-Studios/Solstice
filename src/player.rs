use bevy::{prelude::{Component, Transform, Query, Res, KeyCode}, input::Input};

use crate::constants::DELTA_TIME;

#[derive(Component)]
pub struct Player {
    pub name: String,
    pub speed: f32,
}


pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>
) {
    let (player, mut p_transform) = query.single_mut();

    let mut x_mov = 0.0;
    let mut z_mov = 0.0;


    if keyboard_input.pressed(KeyCode::W) {
        x_mov += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        x_mov -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        z_mov += 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        z_mov -= 1.0;
    }

    let p_translation = &mut p_transform.translation;
    p_translation.x += x_mov * player.speed * DELTA_TIME;
    p_translation.z += z_mov * player.speed * DELTA_TIME;
}