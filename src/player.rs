use bevy::prelude::Component;

#[derive(Component)]
pub struct Player {
    pub name: String,
    pub speed: u32,
}