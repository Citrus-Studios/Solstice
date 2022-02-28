use bevy::prelude::*;

#[derive(Component)]
pub struct GeneratorOptions {
    pub width: u32,
    pub height: u32,
}

pub fn generate(
    mut query: Query<&GeneratorOptions>
) {

}