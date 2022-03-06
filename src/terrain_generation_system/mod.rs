use bevy::prelude::{Plugin, App};

use self::generator::{GeneratorOptions, generate_terrain};

pub mod generator;

pub struct GeneratorPlugin;

impl Plugin for GeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GeneratorOptions {
            width: 100,
            length: 100,
            height: 1,
        })
        .add_startup_system(generate_terrain);
    }
}