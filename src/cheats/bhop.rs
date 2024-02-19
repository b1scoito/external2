use color_eyre::eyre::{self, Result};

use crate::{cheats::CheatState, sdk::cs2::{structures::{EntityFlag, MoveType}, Client, Cs2, Input, LocalPlayer}};

// TODO: Make global init function with game sync global state, using GlobalVars
pub fn initialize(cs2: Cs2) -> Result<()> {
    log::info!("initializing bhop cheat");

    let mut cheat_state = CheatState::new();

    loop {
        let result = cheat_state.cheat_impl(&cs2, |cs2| -> Result<()> {
            // TODO: Fix bhop timing???
            // Space key
            // TODO: Make enum for keys
            if !cs2.is_key_down(66)? {
                return Err(eyre::eyre!("bhop key not pressed"));
            }

            let local_player = cs2.get_local_player()?;
            let move_type = local_player.move_type()?;


            if move_type == MoveType::MOVETYPE_LADDER as u32 || move_type == MoveType::MOVETYPE_NOCLIP as u32 || move_type == MoveType::MOVETYPE_OBSERVER as u32{
                return Err(eyre::eyre!("move type not supported"));
            }
            
            if local_player.flags()? & EntityFlag::FL_ONGROUND == 1 {
                cs2.set_jump()?;
            } else {
                if cs2.get_jump()? == 65537 {
                    cs2.unset_jump()?;
                }
            }
            
            Ok(())
        });

        if let Err(_e) = result {
            continue;
        }
    }
}
