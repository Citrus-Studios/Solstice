use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct GeneratorOptions {
    pub width: u32,
    pub height: u32,
}

pub fn generate_terrain(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    generator_options: Res<GeneratorOptions>
) {

    let hollowground = asset_server.load("models/ground1/ground1.obj");

    let mut rng = rand::thread_rng();
    let randomcolor = (
        rng.gen_range(0.0..255.0)/255.0, 
        rng.gen_range(0.0..255.0)/255.0, 
        rng.gen_range(0.0..255.0)/255.0
    );

    commands.spawn_bundle(PbrBundle {
        mesh: hollowground,
        material: materials.add(Color::rgb(randomcolor.0, randomcolor.1, randomcolor.2).into()),
        transform: Transform::from_xyz(0.0, -2.0, 0.0),
        ..Default::default()
    });
}