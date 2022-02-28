use bevy::prelude::*;

#[derive(Component)]
pub struct GeneratorOptions {
    width: u32,
    height: u32,
}

pub fn generate(
    mut query: Query<&GeneratorOptions>
) {

}