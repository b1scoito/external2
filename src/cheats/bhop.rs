use color_eyre::eyre::Result;

use crate::sdk::cs2::{structures::EntityFlag, Cs2, LocalPlayer};

pub fn init(cs2: Cs2) -> Result<()> {
    log::info!("initializing bhop cheat");

    loop {
        let flags = cs2.flags()?;

        if flags & EntityFlag::FL_ONGROUND == 1 {
            log::debug!("onground");
            // sdk.set_jump()?;
        } else {
            log::debug!("in air");
            // sdk.unset_jump()?;
        }
    }
}
