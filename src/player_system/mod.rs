use bevy::{prelude::{Plugin, SystemSet}, core::FixedTimestep};
use crate::{constants::DELTA_TIME};

use self::player::{player_movement_system};

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