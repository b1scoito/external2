#![allow(dead_code)]

mod windows;
pub mod structures;

use color_eyre::eyre::{self, Result};
use crate::memory::Memory;
use super::{cs2, Sdk};

pub struct Cs2 {
    sdk: Box<dyn Sdk>,
}

pub trait LocalPlayer {
    fn flags(&self) -> Result<u32>;
    fn set_jump(&self) -> Result<()>;
    fn unset_jump(&self) -> Result<()>;
    fn get_jump(&self) -> Result<u32>;
}

impl Cs2 {
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

        let mut local_player_addr: usize = self
            .sdk
            .get_memory()
            .read::<usize>(self.sdk.get_module(module).unwrap().base_address + offset)?;

        // Wait for local_player to be found, will be changed with the addition of map changes and other events
        if local_player_addr == 0 {
            loop {
                local_player_addr = self
                    .sdk
                    .get_memory()
                    .read::<usize>(self.sdk.get_module(module).unwrap().base_address + offset)?;

                if local_player_addr != 0 {
                    break;
                }
            }
        }            

        Ok(local_player_addr)
    }
}

impl LocalPlayer for Cs2 {
    fn flags(&self) -> Result<u32> {
        Ok(self.sdk.get_memory().read::<u32>(
            self.get_local_player()? + cs2::windows::interfaces::client::C_BaseEntity::m_fFlags,
        )?)
    }

    fn set_jump(&self) -> Result<()> {
        self.sdk.get_memory().write::<u32>(
            self.sdk.get_module("client.dll").unwrap().base_address + cs2::windows::offsets::client_dll::dwForceJump,
            65537,
        )?;
        
        Ok(())
    }

    fn unset_jump(&self) -> Result<()> {
        self.sdk.get_memory().write::<u32>(
            self.sdk.get_module("client.dll").unwrap().base_address + cs2::windows::offsets::client_dll::dwForceJump,
            255,
        )?;
        
        Ok(())
    }

    fn get_jump(&self) -> Result<u32> {
        Ok(self.sdk.get_memory().read::<u32>(
            self.sdk.get_module("client.dll").unwrap().base_address + cs2::windows::offsets::client_dll::dwForceJump,
        )?)
    }
}
