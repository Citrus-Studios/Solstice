use bevy::{prelude::{Plugin, SystemSet, CoreStage, ParallelSystemDescriptorCoercion}, core::FixedTimestep};
use bevy_mod_raycast::RaycastSystem;

use crate::{constants::DELTA_TIME, building_system::{visualizer::visualizer, raycasting::{update_raycast_with_cursor, raycast}}};

use self::player::{player_movement_system, player_camera_system};

pub mod player;
pub mod player_startup;

pub struct PlayerSystemPlugin;

impl Plugin for PlayerSystemPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(DELTA_TIME as f64))
                .with_system(player_movement_system)
        );
    }
}