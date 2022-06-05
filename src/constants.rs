use std::{collections::HashMap, f32::consts::PI};

use lazy_static::lazy_static;
use phf::phf_map;
use rand::{thread_rng, Rng};

use crate::gui_branch_builder::GuiBranchBuilder;
use crate::player_system::gui_system::gui::GuiOr;

use bevy::math::Vec3;
use bevy_rapier3d::prelude::CollisionGroups;

pub const DELTA_TIME: f32 = 1.0 / 60.0;
pub const SQRT_OF_2: f32 = 1.41421356237f32 / 2.0;
pub static mut GLOBAL_PIPE_ID: u32 = 0;
pub const HALF_PI: f32 = PI / 2.0;
pub const HALF_SIZE: f32 = 10.0;

/// the portafab fab speed in i/s
pub const FABRICATOR_SPEED: u32 = 100;

pub const MAX_BUILD_DISTANCE: f32 = 30.0;
pub const SNAP_DISTANCE: f32 = 0.5;

// NAMING: Everything BEFORE underscores (_) will be IGNORED in display text. Use spaces.
pub const GUI_LOOKUP: phf::Map<&'static str, [GuiOr; 4]> = phf_map! {
    "base" => GuiBranchBuilder::new()
        .branch("Iridium")
        .branch("Structures")
        .branch("Military")
        .branch("Technology")
        .build(),

    "Iridium" => GuiBranchBuilder::new()
        .branch("Pipes")
        .branch("Extractors")
        .branch("Tanks")
        .branch("Iridium_Special")
        .build(),

    "Structures" => GuiBranchBuilder::new()
        .item("Seat")
        .item("Bridge")
        .branch("Structures_Defense")
        .branch("Basic")
        .build(),

    "Military" => GuiBranchBuilder::new()
        .item("Priority Beacon")
        .item("Shield Generator")
        .branch("Military_Production")
        .branch("Weapons")
        .build(),

    "Technology" => GuiBranchBuilder::new()
        .item("Spawn Point")
        .item("Upgrade Station")
        .branch("Structure Tech")
        .branch("Technology_Misc")
        .build(),

    // Iridium
    "Pipes" => GuiBranchBuilder::new()
        .item("Pipe")
        .item("T-Junction")
        .item("Four-Way")
        .item("Elbow")
        .build(),

    "Extractors" => GuiBranchBuilder::new()
        .item("Well Pump")
        .item("Condenser")
        .item("Submersible")
        .item("Crystal Resonator")
        .build(),

    "Tanks" => GuiBranchBuilder::new()
        .item("Small Tank")
        .item("Standard Tank")
        .item("Large Tank")
        .item("Reservoir")
        .build(),

    "Iridium_Special" => GuiBranchBuilder::new()
        .item("Cap")
        .item("Refill Station")
        .branch("Iridium_Special_Tech")
        .branch("Valves")
        .build(),

    "Iridium_Special_Tech" => GuiBranchBuilder::new()
        .item("Distributor Cap")
        .item("Gauge")
        .build(),

    "Valves" => GuiBranchBuilder::new()
        .item("Valve")
        .item("Drop Tank")
        .build(),

    // Structures
    "Structures_Defense" => GuiBranchBuilder::new()
        .item("Wall")
        .item("Keep")
        .item("Gate")
        .build(),

    "Basic" => GuiBranchBuilder::new()
        .item("Ladder Block") // There are more, obviously
        .build(),

    // Military
    "Military_Production" => GuiBranchBuilder::new()
        .item("Arsenal")
        .item("Garage")
        .item("S.S.I.M.")
        .build(),

    "Weapons" => GuiBranchBuilder::new()
        .branch("Offense")
        .branch("Defense")
        .build(),

    "Offense" => GuiBranchBuilder::new()
        .item("Artillery")
        .item("Bore")
        .build(),

    "Defense" => GuiBranchBuilder::new()
        .item("Turret")
        .item("Arc Turret")
        .item("Point Defense")
        .build(),

    // Technology
    "Structure Tech" => GuiBranchBuilder::new()
        .item("Fabricator")
        .item("Automechanic")
        .build(),

    "Technology_Misc" => GuiBranchBuilder::new()
        .item("Boost Pad")
        .item("Distributor")
        .item("Transceiver")
        .item("Spatial Anchor")
        .build(),
};

lazy_static! {
    pub static ref SEED: u32 = thread_rng().gen::<u32>();

    pub static ref GUI_BACK_LOOKUP: HashMap<String, String> = HashMap::from([
        ("base".ts(), "base".ts()),

        ("Iridium".ts(), "base".ts()),
        ("Structures".ts(), "base".ts()),
        ("Military".ts(), "base".ts()),
        ("Technology".ts(), "base".ts()),

        ("Pipes".ts(), "Iridium".ts()),
        ("Extractors".ts(), "Iridium".ts()),
        ("Tanks".ts(), "Iridium".ts()),
        ("Iridium_Special".ts(), "Iridium".ts()),

        ("Iridium_Special_Tech".ts(), "Iridium_Special".ts()),
        ("Valves".ts(), "Iridium_Special".ts()),

        ("Structures_Defense".ts(), "Structures".ts()),
        ("Basic".ts(), "Structures".ts()),

        ("Military_Production".ts(), "Military".ts()),
        ("Weapons".ts(), "Military".ts()),

        ("Offense".ts(), "Weapons".ts()),
        ("Defense".ts(), "Weapons".ts()),

        ("Structure Tech".ts(), "Technology".ts()),
        ("Technology_Misc".ts(), "Technology".ts()),
    ]);

    /// Pipe cylinder start/end position relative to the position of a pipe base
    pub static ref PIPE_CYLINDER_OFFSET: Vec3 = Vec3::new(0.0, 0.0, 0.235);

    /// Pipe base offset (when placed on ground) relative to the intersection position
    pub static ref PIPE_BASE_OFFSET: Vec3 = Vec3::new(0.0, 0.25, -0.1675);
    pub static ref NO_COLLISION: CollisionGroups = CollisionGroups::new(0, 0);
    pub static ref BLUEPRINT_COLLISION: CollisionGroups = CollisionGroups { memberships: 0b100000, filters: 0b1111111 };
}
// Shorten the .to_string() method by several characters, just for looks
pub trait ShortToString {
    fn ts(self) -> String;
}
impl ShortToString for &str {
    fn ts(self) -> String {
        self.to_string()
    }
}

pub trait MoreVec3Constants {
    fn NEG_Z() -> Vec3;
}

impl MoreVec3Constants for Vec3 {
    fn NEG_Z() -> Vec3 {
        Vec3::new(0.0, 0.0, -1.0)
    }
}
