use bevy::{prelude::{Component, Transform, Query, Res, KeyCode}, input::Input};

#[derive(Component)]
pub struct Player {
    pub name: String,
    pub speed: u32,
}


pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>
) {
    let (player, p_transform) = query.single_mut();

    let mut x_mov = 0;
    let mut z_mov = 0;


    if keyboard_input.pressed(KeyCode::W) {
        x_mov += 1;
    }
    if keyboard_input.pressed(KeyCode::S) {
        x_mov -= 1;
    }
    if keyboard_input.pressed(KeyCode::A) {
        z_mov += 1;
    }
    if keyboard_input.pressed(KeyCode::D) {
        z_mov -= 1;
    }

    
}