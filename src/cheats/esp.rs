use color_eyre::eyre::{self, Result};

use crate::{cheats::CheatState, sdk::cs2::System, Cs2};


// TODO: Make global init function with game sync global state, using GlobalVars
pub fn initialize(cs2: Cs2) -> Result<()> {
    log::info!("initializing bhop cheat");

    let mut cheat_state = CheatState::new();

    loop {
        let result = cheat_state.cheat_impl(&cs2, |cs2| -> Result<()> {
            if !cs2.window_is_cs2()? {
                return Err(eyre::eyre!("cs2 window not found"));
            }

            // Loop through all cached entities
            // Get their location
            // Draw a simple box with overlay
            
            Ok(())
        });

        if let Err(_e) = result {
            continue;
        }
    }
}
