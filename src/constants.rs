use std::{f32::consts::PI, collections::HashMap};

use rand::{thread_rng, Rng};
use lazy_static::lazy_static;

use crate::player_system::gui_system::gui::GuiOr;

pub const DELTA_TIME: f32 = 1.0 / 60.0;
pub const SQRT_OF_2: f32 = 1.41421356237f32 / 2.0;
pub static mut GLOBAL_PIPE_ID: u32 = 0;
pub const HALF_PI: f32 = PI / 2.0;
pub const HALF_SIZE: f32 = 10.0;

lazy_static! {
    pub static ref SEED: u32 = thread_rng().gen::<u32>();

    pub static ref GUI_LOOKUP: HashMap<String, [GuiOr<String>; 4]> = HashMap::from([
        // NAMING: Everything BEFORE underscores (_) will be IGNORED in display text. Use spaces.
            ("base".ts(), [GuiOr::Id("Iridium".ts()), GuiOr::Id("Structures".ts()), GuiOr::Id("Military".ts()), GuiOr::Id("Technology".ts())]),
        
            ("Iridium".ts(), [GuiOr::Id("Pipes".ts()), GuiOr::Id("Extractors".ts()), GuiOr::Id("Tanks".ts()), GuiOr::Id("Iridium_Special".ts())]),
            ("Structures".ts(), [GuiOr::Item("Seat".ts()), GuiOr::Item("Bridge".ts()), GuiOr::Id("Structures_Defense".ts()), GuiOr::Id("Basic".ts())]),
            ("Military".ts(), [GuiOr::Item("Priority Beacon".ts()), GuiOr::Item("Shield Generator".ts()), GuiOr::Id("Military_Production".ts()), GuiOr::Id("Weapons".ts())]),
            ("Technology".ts(), [GuiOr::Item("Spawn Point".ts()), GuiOr::Item("Upgrade Station".ts()), GuiOr::Id("Structure Tech".ts()), GuiOr::Id("Technology_Misc".ts())]),
        
        // Iridium
            ("Pipes".ts(), [GuiOr::Item("Pipe".ts()), GuiOr::Item("T-Junction".ts()), GuiOr::Item("Four-Way".ts()), GuiOr::Item("Elbow".ts())]),
            ("Extractors".ts(), [GuiOr::Item("Well Pump".ts()), GuiOr::Item("Condenser".ts()), GuiOr::Item("Submersible".ts()), GuiOr::Item("Crystal Resonator".ts())]),
            ("Tanks".ts(), [GuiOr::Item("Small Tank".ts()), GuiOr::Item("Standard Tank".ts()), GuiOr::Item("Large Tank".ts()), GuiOr::Item("Reservoir".ts())]),
            ("Iridium_Special".ts(), [GuiOr::Item("Cap".ts()), GuiOr::Item("Refill Station".ts()), GuiOr::Id("Iridium_Special_Tech".ts()), GuiOr::Id("Valves".ts())]),
        
            ("Iridium_Special_Tech".ts(), [GuiOr::Item("Distributor Cap".ts()), GuiOr::Item("Gauge".ts()), GuiOr::None, GuiOr::None]),
            ("Valves".ts(), [GuiOr::Item("Valve".ts()), GuiOr::Item("Drop Tank".ts()), GuiOr::None, GuiOr::None]),
        
        // Structures
            ("Defense_Structures".ts(), [GuiOr::Item("Wall".ts()), GuiOr::Item("Keep".ts()), GuiOr::Item("Gate".ts()), GuiOr::None]),
            ("Basic".ts(), [GuiOr::Item("Ladder Block".ts()), /* make these later im lazy */ GuiOr::None, GuiOr::None, GuiOr::None]),
        
        // Military
            ("Military_Production".ts(), [GuiOr::Item("Arsenal".ts()), GuiOr::Item("Garage".ts()), GuiOr::Item("S.S.I.M.".ts()), GuiOr::None]),
            ("Weapons".ts(), [GuiOr::Id("Offense".ts()), GuiOr::Id("Defense".ts()), GuiOr::None, GuiOr::None]),
        
            ("Offense".ts(), [GuiOr::Item("Artillery".ts()), GuiOr::Item("Bore".ts()), GuiOr::None, GuiOr::None]),
            ("Defense".ts(), [GuiOr::Item("Turret".ts()), GuiOr::Item("Arc Turret".ts()), GuiOr::Item("Point Defense".ts()), GuiOr::None]),
        
        // Technology
            ("Structure Tech".ts(), [GuiOr::Item("Fabricator".ts()), GuiOr::Item("Automechanic".ts()), GuiOr::None, GuiOr::None]),
            ("Technology_Misc".ts(), [GuiOr::Item("Boost Pad".ts()), GuiOr::Item("Distributor".ts()), GuiOr::Item("Transceiver".ts()), GuiOr::Item("Spatial Anchor".ts())]),
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
