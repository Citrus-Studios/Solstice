use bevy::prelude::{Plugin, App};

use self::generator::{GeneratorOptions, generate_terrain, TerrainGenDone};

pub mod generator;
pub mod relevant_attributes;
pub mod mutate_mesh;
pub mod compound_collider_builder;

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