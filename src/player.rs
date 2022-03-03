use bevy::{prelude::*, input::mouse::{MouseMotion, MouseWheel}};
use bevy_rapier3d::{prelude::{RigidBodyVelocityComponent}};

use crate::constants::{DELTA_TIME, SQRT_OF_2, HALF_PI};

#[derive(Component)]
pub struct Player {
    pub name: String,
    pub speed: f32,
}

#[derive(Component, Debug)]
pub struct CameraComp {
    pub yaw: f32,
    pub roll: f32,
    pub zoom: f32,
}


pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,

    gamepads: Res<Gamepads>,
    gamepad_input: Res<Input<GamepadButton>>,
    gamepad_axes: Res<Axis<GamepadAxis>>,

    c_query: Query<&mut CameraComp>,
    mut r_query: Query<&mut RigidBodyVelocityComponent, (Without<CameraComp>, With<Player>)>,
    p_query: Query<&Player, Without<CameraComp>>,
) {
    let mut player_rigidbody = r_query.single_mut();
    let player = p_query.single();
    let camera = c_query.single();

    let mut x_mov = 0f32;
    let mut z_mov = 0f32;

    let yaw = camera.yaw.to_radians();

    let cos_yaw = yaw.cos();
    let sin_yaw = yaw.sin();

    let cos_yaw_half = (yaw + HALF_PI).cos();
    let sin_yaw_half = (yaw + HALF_PI).sin();

    // Get gamepad inputs
    for gamepad in gamepads.iter().cloned() {
        let button_pressed = |button| {
            gamepad_input.pressed(GamepadButton(gamepad, button))
        };
        let axes_moved = |axis| {
            gamepad_axes.get(GamepadAxis(gamepad, axis)).unwrap()
        };

        if button_pressed(GamepadButtonType::DPadUp) || axes_moved(GamepadAxisType::LeftStickY) > 0.05 {
            x_mov -= cos_yaw;
            z_mov -= sin_yaw;
        }
        if button_pressed(GamepadButtonType::DPadDown) || axes_moved(GamepadAxisType::LeftStickY) < -0.05 {
            x_mov += cos_yaw;
            z_mov += sin_yaw;
        }
        if button_pressed(GamepadButtonType::DPadLeft) || axes_moved(GamepadAxisType::LeftStickX) > 0.05 {
            x_mov += cos_yaw_half;
            z_mov += sin_yaw_half;
        }
        if button_pressed(GamepadButtonType::DPadRight) || axes_moved(GamepadAxisType::LeftStickX) < -0.05 {
            x_mov -= cos_yaw_half;
            z_mov -= sin_yaw_half;
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
        x_mov += cos_yaw_half;
        z_mov += sin_yaw_half;
    }
    if keyboard_input.pressed(KeyCode::D) {
        x_mov -= cos_yaw_half;
        z_mov -= sin_yaw_half;
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

    player_rigidbody.linvel = (Vec3::new(x_mov, 0.0, z_mov) * player.speed * DELTA_TIME).into();
}

pub fn player_camera_system(
    mut mouse_motion_event: EventReader<MouseMotion>,
    mouse_input: Res<Input<MouseButton>>,
    mut mouse_scroll_event: EventReader<MouseWheel>,

    gamepads: Res<Gamepads>,
    gamepad_input: Res<Input<GamepadButton>>,
    gamepad_axes: Res<Axis<GamepadAxis>>,

    mut c_query: Query<(&mut CameraComp, &mut Transform)>,
) {
    let (mut camera, mut c_transform) = c_query.single_mut();

    let mut c_translation = *(&mut c_transform.translation.clone());

    let last_camera_zoom = camera.zoom;

    for event in mouse_scroll_event.iter() {
        camera.zoom -= event.y / 5.0;
        camera.zoom = camera.zoom.max(0.1).min(10.0);
    }

    let rmb_pressed = mouse_input.pressed(MouseButton::Right);

    let mut gamepad_axes_moved = false;

    for gamepad in gamepads.iter().cloned() {
        let button_pressed = |button| {
            gamepad_input.pressed(GamepadButton(gamepad, button))
        };
        let axes_moved = |axis| {
            gamepad_axes.get(GamepadAxis(gamepad, axis)).unwrap()
        };

        let rsy = axes_moved(GamepadAxisType::RightStickY);
        let rsx = axes_moved(GamepadAxisType::RightStickX);
        if rsy > 0.05 {
            camera.roll += 2.0 * rsy;
            gamepad_axes_moved = true;
        }
        if rsy < -0.05 {
            camera.roll += 2.0 * rsy;
            gamepad_axes_moved = true;
        }
        if rsx > 0.05 {
            camera.yaw += 2.0 * rsx;
            gamepad_axes_moved = true;
        }
        if rsx < -0.05 {
            camera.yaw += 2.0 * rsx;
            gamepad_axes_moved = true;
        }
        
        if button_pressed(GamepadButtonType::RightThumb) {
            if button_pressed(GamepadButtonType::RightTrigger) {
                camera.zoom += 1.0;
            } else {
                camera.zoom += 0.1;
            }
        }
        if button_pressed(GamepadButtonType::LeftThumb) {
            if button_pressed(GamepadButtonType::LeftTrigger) {
                camera.zoom -= 1.0;
            } else {
                camera.zoom -= 0.1;
            }
        }
    }

    if rmb_pressed || last_camera_zoom != camera.zoom || gamepad_axes_moved {
        if rmb_pressed {
            for event in mouse_motion_event.iter() {
                camera.yaw  += event.delta.x / 5.0;
                camera.roll += event.delta.y / 5.0;
            }
        }

        camera.roll = camera.roll.min(89.99999).max(-89.99999);
        camera.yaw = camera.yaw % 360.0;
        
        let yaw = camera.yaw.to_radians();
        let roll = camera.roll.to_radians();

        let roll_cos = roll.cos();

        c_translation.x = roll_cos * yaw.cos() * camera.zoom;
        c_translation.y = roll.sin() * camera.zoom;
        c_translation.z = roll_cos * yaw.sin() * camera.zoom;

        c_transform.translation = c_translation;
        c_transform.look_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y);
    }
}

// Height (y) of the circle is sin(roll)
// Radius     of the circle is cos(roll)
// Point on the circle is x = cos(roll) * cos(yaw)
//                        y = sin(roll)
//                        z = cos(roll) * sin(yaw)
