use std::ffi::{c_char, c_void};

use color_eyre::eyre::{self, Result};
use log::{debug, info};
use sysinfo::System;

#[cfg(target_os = "windows")]
use crate::memory::{Memory, WindowsMemory};
#[cfg(target_os = "linux")]
use crate::memory::{LinuxMemory, Memory};

pub(crate) mod cs2;

// TODO: Is this the best way to make this cross-platform?
#[cfg(target_os = "linux")]
pub struct LinuxSdk {}

#[cfg(target_os = "windows")]
pub struct WindowsSdk {
    local_player_pawn_address: usize,
    client_base_address: usize,
    client_size: usize,
    memory: WindowsMemory,
    global_vars: GlobalVarsBase,
}

pub trait Sdk {
    fn get_global_vars(&self) -> &GlobalVarsBase;
    fn get_memory(&self) -> &WindowsMemory;
    fn get_client_size(&self) -> usize;
    fn get_client_base_address(&self) -> usize;
    fn get_local_player_pawn_address(&self) -> usize;
    fn new() -> Result<Self>
    where
        Self: Sized;
}

#[cfg(target_os = "linux")]
impl Sdk for LinuxSdk {
    fn new() -> Self {
        Self {}
    }

    fn init(&self) -> Result<()> {
        info!("initializing linux sdk");

        // System
        let mut system = System::new();
        system.refresh_all();

        let mut cs2_pid: usize = 0;

        // Get cs2 process ID
        let cs2_pids = system.processes_by_exact_name("cs2");
        for cs2 in cs2_pids {
            cs2_pid = cs2.parent().unwrap().into();
        }

        if cs2_pid == 0 {
            return Err(eyre::eyre!("failed to find cs2 process"));
        }

        debug!("found game process with pid: {}", cs2_pid);

        let mut libclient_base_address: usize = 0;

        // Populate base addresses
        // TODO: make a hashmap for each module
        let process_maps = proc_maps::get_process_maps(cs2_pid as i32)?;
        for map in process_maps {
            if map.filename().is_none() {
                continue;
            }

            match map.filename() {
                Some(filename) => {
                    if filename.to_string_lossy().contains("libclient.so") && map.is_exec() {
                        debug!(
                            "found libclient.so at: {:?} address: 0x{:x}",
                            filename,
                            map.start()
                        );

                        libclient_base_address = map.start();
                    }
                }
                None => continue,
            }
        }

        // TODO: Find dwLocalPlayerPawn pattern AND offset, then add them together to get the final player address,
        // with that, copy the player struct into a local struct and return the flags so we can check if the player
        // is onground, with that, we get the dwForceJump pattern and respective address.

        debug!("client base address: 0x{:x}", libclient_base_address);

        let memory: LinuxMemory = Memory::new(Pid::from_raw(cs2_pid as i32));
        memory.write::<u32>(libclient_base_address, 0x90909090)?;

        Ok(())
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct GlobalVarsBase {
    real_time: f32,                  // 0x0000
    frame_count: i32,                // 0x0004
    frame_time: f32,                 // 0x0008
    absolute_frame_time: f32,        // 0x000C
    max_clients: i32,                // 0x0010
    pad_0: [u8; 0x14],               // 0x0014
    frame_time_2: f32,               // 0x0028
    current_time: f32,               // 0x002C
    current_time_2: f32,             // 0x0030
    pad_1: [u8; 0xC],                // 0x0034
    tick_count: f32,                 // 0x0040
    pad_2: [u8; 0x4],                // 0x0044
    network_channel: *const c_void,  // 0x0048
    pad_3: [u8; 0x130],              // 0x0050
    current_map: *const c_char,      // 0x0180
    current_map_name: *const c_char, // 0x0188
}

#[cfg(target_os = "windows")]
impl Sdk for WindowsSdk {
    fn new() -> Result<WindowsSdk> {
        info!("initializing windows sdk");

        // System
        let mut system = System::new();
        system.refresh_all();

        let mut cs2_pid: usize = 0;

        // Get cs2 process ID
        let cs2_pids = system.processes_by_exact_name("cs2.exe");
        for cs2 in cs2_pids {
            cs2_pid = cs2.pid().into();
        }

        if cs2_pid == 0 {
            return Err(eyre::eyre!("failed to find cs2 process"));
        }

        debug!("found game process with pid: {}", cs2_pid);

        let memory: WindowsMemory = Memory::new(cs2_pid.into())?;
        let (client_base_address, client_size) = memory.get_module("client.dll")?;
        debug!("client base address: 0x{:x} size: {}", client_base_address, client_size);

        let (engine_base_address, engine_size) = memory.get_module("engine2.dll")?;
        debug!("engine base address: 0x{:x} size: {}", engine_base_address, engine_size);

        let local_player_pawn_address: usize = memory.read::<usize>(client_base_address + cs2::windows::offsets::client_dll::dwLocalPlayerPawn)?;
        debug!("local player pawn address: 0x{:x}", local_player_pawn_address);

        debug!("global vars addres: {}", client_base_address + cs2::windows::offsets::client_dll::dwGlobalVars);

        let global_vars = memory.read::<GlobalVarsBase>(client_base_address + cs2::windows::offsets::client_dll::dwGlobalVars)?;

        Ok(Self {
            local_player_pawn_address,
            client_base_address,
            client_size,
            memory,
            global_vars,
        })
    }

    fn get_local_player_pawn_address(&self) -> usize {
        self.local_player_pawn_address
    }

    fn get_client_base_address(&self) -> usize {
        self.client_base_address
    }

    fn get_client_size(&self) -> usize {
        self.client_size
    }

    fn get_memory(&self) -> &WindowsMemory {
        &self.memory
    }

    fn get_global_vars(&self) -> &GlobalVarsBase {
        &self.global_vars
    }
}
