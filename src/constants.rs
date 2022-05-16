use std::{collections::HashMap, f32::consts::PI};

use lazy_static::lazy_static;
use rand::{thread_rng, Rng};

use crate::{player_system::gui_system::gui::GuiOr};

use bevy::math::Vec3;

pub const DELTA_TIME: f32 = 1.0 / 60.0;
pub const SQRT_OF_2: f32 = 1.41421356237f32 / 2.0;
pub static mut GLOBAL_PIPE_ID: u32 = 0;
pub const HALF_PI: f32 = PI / 2.0;
pub const HALF_SIZE: f32 = 10.0;

macro_rules! GUIBranch {
    ($name:literal - $x0:literal $y0:literal, $x1:literal $y1:literal, $x2:literal $y2:literal, $x3:literal $y3:literal) => {
        (
            String::from($name),
            [
                if $x0 == 0 {
                    if $y0 == "None" {
                        GuiOr::None
                    } else {
                        GuiOr::Id(String::from($y0))
                    }
                } else {
                    if $y0 == "None" {
                        GuiOr::None
                    } else {
                        GuiOr::Item(String::from($y0))
                    }
                },
                if $x1 == 0 {
                    if $y1 == "None" {
                        GuiOr::None
                    } else {
                        GuiOr::Id(String::from($y1))
                    }
                } else {
                    if $y1 == "None" {
                        GuiOr::None
                    } else {
                        GuiOr::Item(String::from($y1))
                    }
                },
                if $x2 == 0 {
                    if $y2 == "None" {
                        GuiOr::None
                    } else {
                        GuiOr::Id(String::from($y2))
                    }
                } else {
                    if $y2 == "None" {
                        GuiOr::None
                    } else {
                        GuiOr::Item(String::from($y2))
                    }
                },
                if $x3 == 0 {
                    if $y3 == "None" {
                        GuiOr::None
                    } else {
                        GuiOr::Id(String::from($y3))
                    }
                } else {
                    if $y3 == "None" {
                        GuiOr::None
                    } else {
                        GuiOr::Item(String::from($y3))
                    }
                },
            ],
        )
    };
}

lazy_static! {
    pub static ref SEED: u32 = thread_rng().gen::<u32>();

    pub static ref GUI_LOOKUP: HashMap<String, [GuiOr<String>; 4]> = HashMap::from([
            // NAMING: Everything BEFORE underscores (_) will be IGNORED in display text. Use spaces.
            GUIBranch!("base" - 0 "Iridium", 0 "Structures", 0 "Military", 0 "Technology"),

            GUIBranch!("Iridium" - 0 "Pipes", 0 "Extractors", 0 "Tanks", 0 "Iridium_Special"),
            GUIBranch!("Structures" - 1 "Seat", 1 "Bridge", 0 "Structures_Defense", 0 "Basic"),
            GUIBranch!("Military" - 1 "Priority Beacon", 1 "Shield Generator", 0 "Military_Production", 0 "Weapons"),
            GUIBranch!("Technology" - 1 "Spawn Point", 1 "Upgrade Station", 0 "Structure Tech", 0 "Technology_Misc"),

            // Iridium
            GUIBranch!("Pipes" - 1 "Pipe", 1 "T-Junction", 1 "Four-Way", 1 "Elbow"),
            GUIBranch!("Extractors" - 1 "Well Pump", 1 "Condenser", 1 "Submersible", 1 "Crystal Resonator"),
            GUIBranch!("Tanks" - 1 "Small Tank", 1 "Standard Tank", 1 "Large Tank", 1 "Reservoir"),
            GUIBranch!("Iridium_Special" - 1 "Cap", 1 "Refill Station", 0 "Iridium_Special_Tech", 0 "Valves"),

            GUIBranch!("Iridium_Special_Tech" - 1 "Distributor Cap", 1 "Gauge", 0 "None", 0 "None"),
            GUIBranch!("Valves" - 1 "Valve", 1 "Drop Tank", 0 "None", 0 "None"),

            // Structures
            GUIBranch!("Defense_Structures" - 1 "Wall", 1 "Keep", 1 "Gate", 0 "None"),
             /* make these later im lazy */
            GUIBranch!("Basic" - 1 "Ladder Block", 0 "None", 0 "None", 0 "None"),

            // Military
            GUIBranch!("Military_Production" - 1 "Arsenal", 1 "Garage", 1 "S.S.I.M.", 0 "None"),
            GUIBranch!("Weapons" - 0 "Offense", 0 "Defense", 0 "None", 0 "None"),

            GUIBranch!("Offense" - 1 "Artillery", 1 "Bore", 0 "None", 0 "None"),
            GUIBranch!("Defense" - 1 "Turret", 1 "Arc Turret", 1 "Point Defense", 0 "None"),

            // Technology
            GUIBranch!("Structure Tech" - 1 "Fabricator", 1 "Automechanic", 0 "None", 0 "None"),
            GUIBranch!("Technology_Misc" - 1 "Boost Pad", 1 "Distributor", 1 "Transceiver", 1 "Spatial Anchor"),
        ]);

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
    
    pub static ref PIPE_CYLINDER_OFFSET: Vec3 = Vec3::new(0.0, 0.25, 0.0675);
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
