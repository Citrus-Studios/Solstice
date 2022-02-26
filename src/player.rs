use bevy::prelude::*;

use crate::constants::DELTA_TIME;

#[derive(Component)]
pub struct Player {
    pub name: String,
    pub speed: f32,
}


pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,

    gamepads: Res<Gamepads>,
    gamepad_input: Res<Input<GamepadButton>>,
    gamepad_axes: Res<Axis<GamepadAxis>>,

    mut query: Query<(&Player, &mut Transform)>
) {
    let (player, mut p_transform) = query.single_mut();

    let mut x_mov = 0.0;
    let mut z_mov = 0.0;

    for gamepad in gamepads.iter().cloned() {
        let button_pressed = |button| {
            gamepad_input.pressed(GamepadButton(gamepad, button))
        };
        let axes_moved = |axis| {
            gamepad_axes.get(GamepadAxis(gamepad, axis)).unwrap()
        };

        if button_pressed(GamepadButtonType::DPadUp) || axes_moved(GamepadAxisType::LeftStickY) > 0.05 {
            x_mov += 1.0;
        }
        if button_pressed(GamepadButtonType::DPadDown) || axes_moved(GamepadAxisType::LeftStickY) < -0.05 {
            x_mov -= 1.0;
        }
        if button_pressed(GamepadButtonType::DPadLeft) || axes_moved(GamepadAxisType::LeftStickX) > 0.05 {
            z_mov -= 1.0;
        }
        if button_pressed(GamepadButtonType::DPadRight) || axes_moved(GamepadAxisType::LeftStickX) < -0.05 {
            z_mov += 1.0;
        }
    }
    if keyboard_input.pressed(KeyCode::W) {
        x_mov += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        x_mov -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        z_mov -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        z_mov += 1.0;
    }

    let p_translation = &mut p_transform.translation;
    p_translation.x += x_mov * player.speed * DELTA_TIME;
    p_translation.z += z_mov * player.speed * DELTA_TIME;
}