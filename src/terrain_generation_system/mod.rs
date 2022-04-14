use bevy::prelude::{Plugin, App};

use self::generator::{GeneratorOptions, generate_terrain, TerrainGenDone};

pub mod generator;
pub mod relevant_attributes;
pub mod mutate_mesh;

pub struct GeneratorPlugin;

impl Plugin for GeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GeneratorOptions {
            width: 200,
            length: 200,
            height: 1,
        })
        .insert_resource(TerrainGenDone { done: false })
        .add_system(generate_terrain);
    }
}