use crate::constants::DELTA_TIME;
use bevy::{
    core::FixedTimestep,
    prelude::{Plugin, SystemSet},
};

use self::{player::player_movement_system, player_startup::PlayerStartupDone, gui_system::gui::gui};
use crate::player_system::gui_system::gui_startup::*;

pub mod player;
pub mod player_startup;
pub mod gui_system;

pub struct PlayerSystemPlugin;

impl Plugin for PlayerSystemPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(PlayerStartupDone { done: false });
        app.add_system(player_startup::player_start)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(DELTA_TIME as f64))
                    .with_system(player_movement_system),
            )
            .add_startup_system(gui_startup)
            .add_system(gui);
    }
}
