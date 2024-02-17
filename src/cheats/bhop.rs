use color_eyre::eyre::Result;
use log::{debug, info};

#[cfg(target_os = "windows")]
use winapi::um::winuser::{GetAsyncKeyState, VK_SPACE};

use crate::sdk::entity::Entity;

pub fn init(entity: Entity) -> Result<()> {
    info!("initializing bhop cheat");

    loop {
        #[cfg(target_os = "windows")]
        if (unsafe { GetAsyncKeyState(VK_SPACE) } == 0) {
            continue;
        }

        let player_flags = entity.get_local_player_flags()?;
        if player_flags & 1 << 0 != 0 {
            debug!("jumping");
            // sdk.get_memory().write::<i32>(
            //     sdk.get_client_base_address() + cs2::windows::offsets::client_dll::dwForceJump,
            //     65537,
            // )?;
        } else {
            debug!("not jumping")
            // if sdk.get_memory().read::<i32>(
            //     sdk.get_client_base_address() + cs2::windows::offsets::client_dll::dwForceJump,
            // )? == 65537
            // {
            //     debug!("not jumping");
            //     sdk.get_memory().write::<i32>(
            //         sdk.get_client_base_address() + cs2::windows::offsets::client_dll::dwForceJump,
            //         256,
            //     )?;
            // }
        }
    }
}
