use color_eyre::eyre::{self, Result};

use crate::memory::Memory;

use super::{cs2, Sdk};

pub struct Entity {
    sdk: Box<dyn Sdk>,
}

impl Entity {
    #[inline]
    pub fn new(sdk: Box<dyn Sdk>) -> Result<Self> {
        Ok(Self { sdk })
    }

    #[inline]
    pub fn get_local_player(&self) -> Result<usize> {
        let offset = if cfg!(target_os = "windows") {
            cs2::windows::offsets::client_dll::dwLocalPlayerPawn
        } else if cfg!(target_os = "linux") {
            // cs2::linux::offsets::client_dll::dwLocalPlayerPawn
            0x0
        } else {
            return Err(eyre::eyre!("unsupported platform"));
        };

        let module = if cfg!(target_os = "windows") {
            "client.dll"
        } else if cfg!(target_os = "linux") {
            "libclient.so"
        } else {
            return Err(eyre::eyre!("unsupported platform"));
        };

        let local_player_addr = self
            .sdk
            .get_memory()
            .read::<usize>(self.sdk.get_module(module).unwrap().base_address + offset)?;

        Ok(local_player_addr)
    }

    pub fn get_local_player_flags(&self) -> Result<i32> {
        Ok(self.sdk.get_memory().read::<i32>(
            self.get_local_player()? + cs2::windows::interfaces::client::C_BaseEntity::m_fFlags,
        )?)
    }
}
