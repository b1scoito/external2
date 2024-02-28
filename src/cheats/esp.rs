use std::sync::Arc;

use color_eyre::eyre::{self, Result};

use crate::{cheats::CheatState, sdk::cs2::{Entity, System}, Cs2};


// TODO: Make global init function with game sync global state, using GlobalVars
pub fn initialize(cs2: Arc<Cs2>) -> Result<()> {
    log::info!("initializing bhop cheat");

    let mut cheat_state = CheatState::new();

    loop {
        let result = cheat_state.cheat_impl(&cs2, |cs2| -> Result<()> {
            if !cs2.window_is_cs2()? {
                return Err(eyre::eyre!("cs2 window not found"));
            }

            for (entity_address, entity) in cs2.get_cached_entity_list()? {
                let position = entity.get_position()?;
                let health = entity.health()?;
                let life_state = entity.life_state()?;
                let move_type = entity.move_type()?;
                let flags = entity.flags()?;
                let name = entity.name()?;

                log::info!("entity_address: {:#X}, position: {:?}, health: {}, life_state: {}, move_type: {}, flags: {}, name {}", entity_address, position, health, life_state, move_type, flags, name);
            }
            
            Ok(())
        });

        if let Err(_e) = result {
            continue;
        }
    }
}
