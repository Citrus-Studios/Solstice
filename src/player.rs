use std::f32::consts::PI;

use bevy::{prelude::*, input::mouse::MouseMotion};

use crate::constants::{DELTA_TIME, SQRT_OF_2};

#[derive(Component)]
pub struct Player {
    pub name: String,
    pub speed: f32,
}

#[derive(Component, Debug)]
pub struct CameraComp {
    pub yaw: f32,
    pub roll: f32,
}


pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,

    gamepads: Res<Gamepads>,
    gamepad_input: Res<Input<GamepadButton>>,
    gamepad_axes: Res<Axis<GamepadAxis>>,

    mut c_query: Query<(&mut CameraComp, &mut Transform)>,
    mut p_query: Query<(&Player, &mut Transform), Without<CameraComp>>
) {
    // Get the player and their transform
    let (player, mut p_transform) = p_query.single_mut();
    let (camera, _) = c_query.single_mut();

    let mut x_mov = 0f32;
    let mut z_mov = 0f32;

    let yaw = camera.yaw.to_radians();

    let cos_yaw = yaw.cos();
    let sin_yaw = yaw.sin();

    let half_pi = (PI / 2.0);

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
        x_mov -= cos_yaw;
        z_mov -= sin_yaw;
    }
    if keyboard_input.pressed(KeyCode::S) {
        x_mov += cos_yaw;
        z_mov += sin_yaw;
    }
    if keyboard_input.pressed(KeyCode::A) {
        x_mov -= (yaw - half_pi).cos();
        z_mov -= (yaw - half_pi).sin();
    }
    if keyboard_input.pressed(KeyCode::D) {
        x_mov -= (yaw + half_pi).cos();
        z_mov -= (yaw + half_pi).sin();
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
    mouse_input: Res<Input<MouseButton>>,

    mut c_query: Query<(&mut CameraComp, &mut Transform)>,
    mut p_query: Query<(&Player, &mut Transform), Without<CameraComp>>
) {
    let (mut camera, mut c_transform) = c_query.single_mut();
    let (_, mut p_transform) = p_query.single_mut();

    let c_rotation = c_transform.rotation; 

    let mut c_translation = *(&mut c_transform.translation.clone());

    if mouse_input.pressed(MouseButton::Right) {
    for event in mouse_motion_event.iter() {
        camera.yaw  += event.delta.x / 5.0;
        camera.roll += event.delta.y / 5.0;
        camera.roll = camera.roll.min(89.99999).max(-89.99999);

        //info!(camera.roll);

        let yaw = camera.yaw.to_radians();
        let roll = camera.roll.to_radians();

        let p_translation = *(&p_transform.translation.clone());

        let roll_cos = roll.cos();

        c_translation.x = roll_cos * yaw.cos() * 5.0;
        c_translation.y = roll.sin() * 5.0;
        c_translation.z = roll_cos * yaw.sin() * 5.0;

        c_transform.translation = c_translation;
        c_transform.look_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y);
    }}
}

// Height (y) of the circle is sin(roll)
// Radius     of the circle is cos(roll)
// Point on the circle is x = cos(roll) * cos(yaw)
//                        y = sin(roll)
//                        z = cos(roll) * sin(yaw)
