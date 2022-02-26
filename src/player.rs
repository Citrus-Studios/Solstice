use bevy::{prelude::*, input::mouse::MouseMotion};

use crate::constants::{DELTA_TIME, SQRT_OF_2};

#[derive(Component)]
pub struct Player {
    pub name: String,
    pub speed: f32,
}

#[derive(Component)]
pub struct CameraComp {
    pub yaw: f32,
    pub roll: f32,
}


pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,

    gamepads: Res<Gamepads>,
    gamepad_input: Res<Input<GamepadButton>>,
    gamepad_axes: Res<Axis<GamepadAxis>>,

    mut query: Query<(&Player, &mut Transform)>
) {
    // Get the player and their transform
    let (player, mut p_transform) = query.single_mut();

    let mut x_mov = 0f32;
    let mut z_mov = 0f32;

    // Get gamepad inputs
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
    // Get keyboard inputs
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

    // Clamp x and z to -1.0 and 1.0
    if x_mov > 1.0 {
        x_mov = 1.0
    } else if x_mov < -1.0 {
        x_mov = -1.0
    } 

    if z_mov > 1.0 {
        z_mov = 1.0
    } else if z_mov < -1.0 {
        z_mov = -1.0
    }

    if x_mov.abs() + z_mov.abs() == 2.0 {
        x_mov = SQRT_OF_2 * x_mov;
        z_mov = SQRT_OF_2 * z_mov;
    }

    let p_translation = &mut p_transform.translation;
    p_translation.x += x_mov * player.speed * DELTA_TIME;
    p_translation.z += z_mov * player.speed * DELTA_TIME;
}

pub fn player_camera_system(
    mut mouse_motion_event: EventReader<MouseMotion>,

    mut query: Query<(&mut CameraComp, &mut Transform)>,
) {
    let (mut camera, mut c_transform) = query.single_mut();

    let c_rotation = &mut c_transform.rotation; 

    for event in mouse_motion_event.iter() {
        camera.yaw  += event.delta.x;
        camera.roll += event.delta.y;
    }
}