use crate::memory::Memory;

#[cfg(target_os = "windows")]
use super::{cs2, Sdk, WindowsSdk};

#[cfg(target_os = "linux")]
use super::{cs2, Sdk, LinuxSdk};

use color_eyre::eyre::Result;
use log::debug;


pub struct Entity {
    #[cfg(target_os = "windows")]
    sdk: WindowsSdk,
    #[cfg(target_os = "linux")]
    sdk: LinuxSdk,
}

impl Entity {
    #[inline]
    #[cfg(target_os = "windows")]
    pub fn new(sdk: WindowsSdk) -> Result<Self> {
        Ok(Self {sdk})
    }

    #[inline]
    #[cfg(target_os = "linux")]
    pub fn new(sdk: LinuxSdk) -> Result<Self> {
        Ok(Self {sdk})
    }

    #[inline]
    pub fn get_local_player_pawn_address(&self) -> Result<usize> {
        Ok(self.sdk.get_memory().read::<usize>(self.sdk.get_module("client").unwrap().base_address + cs2::windows::offsets::client_dll::dwLocalPlayerPawn)?)
    }

    pub fn get_local_player_flags(&self) -> Result<i32> {
        Ok(self.sdk.get_memory().read::<i32>(self.get_local_player_pawn_address()? + cs2::windows::interfaces::client::C_BaseEntity::m_fFlags)?)
    }
}