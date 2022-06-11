use bevy::prelude::{App, Plugin};

use self::generator::{generate_terrain, GeneratorOptions, TerrainGenDone};

pub mod compound_collider_builder;
pub mod generator;
pub mod mutate_mesh;
pub mod relevant_attributes;
pub mod terrain_block;

pub struct GeneratorPlugin;

impl Plugin for GeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GeneratorOptions {
            radius: 100,
            height: 1,
        })
        .insert_resource(TerrainGenDone { done: false })
        .add_system(generate_terrain);
    }
}
