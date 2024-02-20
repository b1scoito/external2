use std::{thread, time::Duration};

use color_eyre::eyre::{self, Result};

use crate::{cheats::CheatState, sdk::cs2::{structures::{EntityFlag, LifeState, MoveType}, Client, Cs2, Entity, Input, System}};


// TODO: Make global init function with game sync global state, using GlobalVars
pub fn initialize(cs2: Cs2) -> Result<()> {
    log::info!("initializing bhop cheat");

    let mut cheat_state = CheatState::new();

    loop {
        let result = cheat_state.cheat_impl(&cs2, |cs2| -> Result<()> {
            if !cs2.window_is_cs2()? {
                return Err(eyre::eyre!("cs2 window not found"));
            }

            if !cs2.is_key_down(rdev::Key::Space)? {
                return Err(eyre::eyre!("bhop key not pressed"));
            }

            let local_player = cs2.get_local_player()?;
            let move_type = local_player.move_type()?;
            // let life_state = local_player.life_state()?;

            // if life_state & LifeState::LIFE_ALIVE as u32 == 1 {
            //     return Err(eyre::eyre!("local player is dead"));
            // }

            if move_type == MoveType::MOVETYPE_LADDER as u32 || 
                move_type == MoveType::MOVETYPE_NOCLIP as u32 ||
                move_type == MoveType::MOVETYPE_OBSERVER as u32{
                return Err(eyre::eyre!("move type not supported"));
            }
            
            if local_player.flags()? & EntityFlag::FL_ONGROUND == 1 {
                // 1-tick for sub-tick?
                // This probably needs to be compensated with the f
                thread::sleep(Duration::from_micros(15625));
                cs2.set_jump()?;
            } else {
                if cs2.get_jump()? == 65537 {
                    cs2.unset_jump()?;
                }
                // TODO: Find out the best sub-tick value for this
                // TODO: Fix bhop timing???
                // Which delay for next tick???
                thread::sleep(Duration::from_micros(1000));

            }

            
            Ok(())
        });

        if let Err(_e) = result {
            continue;
        }
    }
}
