use bevy::prelude::Component;

#[derive(Component)]
pub struct Player {
    name: String,
    speed: u32,
}