use bevy::prelude::{Component, Transform, Query};

#[derive(Component)]
pub struct Player {
    pub name: String,
    pub speed: u32,
}


pub fn player_movement_system(
    mut query: Query<(&Player, &mut Transform)>
) {
    
}