#![allow(dead_code)]

mod windows;
pub mod structures;

use active_win_pos_rs::get_active_window;
use std::{collections::HashMap, sync::{Arc, Mutex}};

use color_eyre::eyre::{self, Ok, Result};
use rdev::Key;
use crate::{memory::Memory, sdk::cs2::structures::GlobalVarsBase};
use lazy_static::lazy_static;

use super::{cs2, Sdk};

lazy_static! { // Requires the `lazy_static` crate for global laziness
    static ref ENTITY_CACHE: Mutex<HashMap<usize, EntityImpl>> = Mutex::new(HashMap::new());
}

pub struct Cs2 {
    sdk: Arc<dyn Sdk>,
}


#[derive(Clone)]
pub struct EntityImpl {
    pub entity_address: usize,
    pub sdk: Arc<dyn Sdk>,
}

pub trait Entity {
    fn name(&self) -> Result<String>;
    fn get_position(&self) -> Result<structures::Vector3::<f32>>;
    fn health(&self) -> Result<u32>;
    fn life_state(&self) -> Result<u32>;
    fn move_type(&self) -> Result<u32>;
    fn flags(&self) -> Result<u32>;
}

pub trait Client {
    fn get_local_player(&self) -> Result<EntityImpl>;
    fn get_global_vars(&self) -> Result<GlobalVarsBase>;
    fn get_current_map_name(&self) -> Result<String>;
    fn set_jump(&self) -> Result<()>;
    fn unset_jump(&self) -> Result<()>;
    fn get_jump(&self) -> Result<u32>;
}

pub trait Input {
    fn is_key_down(&self, key: Key) -> Result<bool>;
}

pub trait System {
    fn window_is_cs2(&self) -> Result<bool>;
}


impl Cs2 {
    #[inline]
    pub fn new(sdk: Arc<dyn Sdk>) -> Result<Self> {
        Ok(Self { sdk })
    }

    pub fn update_entity_cache(&self) -> Result<()> {
        let mut cache = ENTITY_CACHE.lock().unwrap();
        cache.clear(); // Clear the existing cache

        let entity_list_address = self.sdk.get_memory().read::<usize>(
            self.sdk.get_module("client.dll").unwrap().base_address + cs2::windows::offsets::client_dll::dwEntityList,
        )?;

        // Get the entity list address
        for i in 0..self.get_global_vars()?.max_clients {
            // first entry
            let list_entry = self.sdk.get_memory().read::<usize>(entity_list_address + (0x8 * ((i as usize & 0x7FFF) >> 9)) + 0x10)?;
            if list_entry == 0 {
                continue;
            } else {
                log::debug!("list_entry: {:#X}", list_entry);
            }

            // Get the entity controller address
            let controller = self.sdk.get_memory().read::<usize>(list_entry + (0x78 * (i as usize & 0x1FF)))?;
            if controller == 0 {
                continue; 
            } else {
                log::debug!("controller: {:#X}", controller);
            }

            // Get the entity pawn address
            let pawn_handle = self.sdk.get_memory().read::<usize>(controller + 0x7E4)?;
            if pawn_handle == 0 {
                continue;
            } else {
                log::debug!("pawn_handle: {:#X}", pawn_handle);
            }

            let second_list_entry = self.sdk.get_memory().read::<usize>(entity_list_address + 0x8 * (((pawn_handle & 0x7FFF) >> 9) + 0x10))?;
            if second_list_entry == 0 { 
                continue;
            } else{
                log::debug!("second_list_entry: {:#X}", second_list_entry);
            }

            let pawn = self.sdk.get_memory().read::<usize>(second_list_entry + (0x78 * (pawn_handle & 0x1FF)))?;
            if pawn == 0 {
                continue;
            } else{
                log::debug!("pawn: {:#X}", pawn);
            }

            cache.insert(i as usize, EntityImpl {
                entity_address: pawn, // Just an example, actual initialization may vary
                sdk: self.sdk.clone(),
            }); // Use entity index or a unique identifier as the key
        }

        Ok(())
    }

    pub fn get_cached_entity_list(&self) -> Result<HashMap<usize, EntityImpl>> {
        let cache = ENTITY_CACHE.lock().unwrap();
        Ok(cache.clone())
    }
}


impl Entity for EntityImpl {
    #[inline]
    fn flags(&self) -> Result<u32> {
        Ok(self.sdk.get_memory().read::<u32>(self.entity_address + cs2::windows::interfaces::client::C_BaseEntity::m_fFlags)?)
    }

    #[inline]
    fn move_type(&self) -> Result<u32> {
        Ok(self.sdk.get_memory().read::<u32>(self.entity_address + cs2::windows::interfaces::client::C_BaseEntity::m_MoveType)?)
    }

    #[inline]
    fn life_state(&self) -> Result<u32> {
        Ok(self.sdk.get_memory().read::<u32>(self.entity_address + cs2::windows::interfaces::client::C_BaseEntity::m_lifeState)?)
    }

    #[inline]
    fn health(&self) -> Result<u32> {
        Ok(self.sdk.get_memory().read::<u32>(self.entity_address + cs2::windows::interfaces::client::C_BaseEntity::m_iHealth)?)
    }

    #[inline]
    fn get_position(&self) -> Result<structures::Vector3::<f32>> {
        Ok(self.sdk.get_memory().read::<structures::Vector3::<f32>>(self.entity_address + cs2::windows::interfaces::client::C_BasePlayerPawn::m_vOldOrigin
            )?)
    }

    #[inline]
    fn name(&self) -> Result<String> {
        Ok(self.sdk.get_memory().read_string(self.entity_address + cs2::windows::interfaces::client::CBasePlayerController::m_iszPlayerName)?)
    }
}

impl Client for Cs2 {
    fn get_local_player(&self) -> Result<EntityImpl> {
        let mut local_player_addr: usize = self
            .sdk
            .get_memory()
            .read::<usize>(self.sdk.get_module("client.dll").unwrap().base_address + cs2::windows::offsets::client_dll::dwLocalPlayerPawn)?;

        // Wait for local_player to be found, will be changed with the addition of map changes and other events
        if local_player_addr == 0 {
            loop {
                local_player_addr = self
                    .sdk
                    .get_memory()
                    .read::<usize>(self.sdk.get_module("client.dll").unwrap().base_address + cs2::windows::offsets::client_dll::dwLocalPlayerPawn)?;

                if local_player_addr != 0 {
                    break;
                }
            }
        }            

        Ok(EntityImpl {
            entity_address: local_player_addr,
            sdk: self.sdk.clone(),
        })
    }
    
    fn get_global_vars(&self) -> Result<GlobalVarsBase> {
        let global_vars_address = self.sdk.get_memory().read::<*const cs2::structures::GlobalVarsBase>(
            self.sdk.get_module("client.dll").unwrap().base_address + cs2::windows::offsets::client_dll::dwGlobalVars,
        )?;

        let global_vars: GlobalVarsBase = self.sdk.get_memory().read(global_vars_address as usize)?;

        Ok(global_vars)
    }

    #[inline]
    fn get_current_map_name(&self) -> Result<String> {
        Ok(self.sdk.get_memory().read_string(self.get_global_vars()?.current_map as usize)?)
    }

    #[inline]
    fn set_jump(&self) -> Result<()> {
        // Ok(self.sdk.get_memory().write::<u32>(
        //             self.sdk.get_module("client.dll").unwrap().base_address + cs2::windows::offsets::client_dll::dwForceJump,
        //             65537,
        //         )?)

        Ok(self.sdk.get_input_system().send_input(Key::KeyJ)?)
    }

    #[inline]
    fn unset_jump(&self) -> Result<()> {
        Ok(self.sdk.get_memory().write::<u32>(
            self.sdk.get_module("client.dll").unwrap().base_address + cs2::windows::offsets::client_dll::dwForceJump,
            255,
        )?)
    }

    #[inline]
    fn get_jump(&self) -> Result<u32> {
        Ok(self.sdk.get_memory().read::<u32>(
            self.sdk.get_module("client.dll").unwrap().base_address + cs2::windows::offsets::client_dll::dwForceJump,
        )?)
    }
}

impl Input for Cs2 {
    fn is_key_down(&self, key: Key) -> Result<bool> {
        // let input_system = self.sdk.get_module("inputsystem.dll").unwrap().base_address + cs2::windows::offsets::inputsystem_dll::dwInputSystem;

        // let is_key_down = |key_code: i32| -> bool {
        //     let key_map_element = self.sdk.get_memory().read::<i32>((input_system + 0x4 * (key_code as usize / 32) + 0x12A0).into()).unwrap_or(0);
        
        //     unsafe { _bittest(&key_map_element, key_code & 0x1F) != 0 }
        // };

        // Ok(is_key_down(key as i32))

        Ok(self.sdk.get_input_system().is_key_down(key)?)
    }
}

impl System for Cs2 {
    fn window_is_cs2(&self) -> Result<bool> {
        let active_window = get_active_window();
        match active_window {
            std::result::Result::Ok(window) => {
                if window.title == "Counter-Strike 2" {
                    return Ok(true);
                } 
            },
            Err(e) => {
                return Err(eyre::eyre!("Error getting active window: {:?}", e));
            },
        }

        Ok(false)
    }
}