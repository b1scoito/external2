use std::{thread, time::Duration};

use color_eyre::eyre::Result;
use log::{debug, info};
use winapi::um::winuser::{GetAsyncKeyState, VK_SPACE};

use crate::{memory::Memory, sdk::{cs2, Sdk}};

pub fn init<T: Sdk>(sdk: T) -> Result<()> {
    info!("initializing bhop cheat");

    let global_vars = sdk.get_global_vars();

    debug!("global_vars: {:?}", global_vars);

    loop {
        if (unsafe { GetAsyncKeyState(VK_SPACE) } == 0) {
            continue;
        }

        thread::sleep(Duration::from_millis(35));

        
        let player_flags = sdk.get_memory().read::<i32>(sdk.get_local_player_pawn_address() + 0x3D4)?;
        if player_flags & 1 << 0 != 0 {
            debug!("jumping");
            sdk.get_memory().write::<i32>(sdk.get_client_base_address() + cs2::windows::offsets::client_dll::dwForceJump, 65537)?;
        } else {
            if sdk.get_memory().read::<i32>(sdk.get_client_base_address() + cs2::windows::offsets::client_dll::dwForceJump)? == 65537 {
                debug!("not jumping");
                sdk.get_memory().write::<i32>(sdk.get_client_base_address() + cs2::windows::offsets::client_dll::dwForceJump, 256)?;
            }
        }

    }
}