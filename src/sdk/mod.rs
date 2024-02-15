use color_eyre::eyre::{self, Result};
use log::{debug, info};
use sysinfo::System;

#[cfg(target_os = "linux")]
use nix::unistd::Pid;
use winapi::um::winuser::{GetAsyncKeyState, VK_SPACE};

use crate::memory::{Memory, WindowsMemory};
#[cfg(target_os = "linux")]
use crate::memory::{LinuxMemory, Memory};

mod cs2;

// TODO: Is this the best way to make this cross-platform?
pub struct LinuxSdk {}
pub struct WindowsSdk {}

pub trait Sdk {
    fn new() -> Self;
    fn init(&self) -> Result<()>;
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

struct EntityFlag;

impl EntityFlag {
    const FL_ONGROUND: u32 = 1 << 0;
    const FL_DUCKING: u32 = 1 << 1;
    const FL_WATERJUMP: u32 = 1 << 2;
    // Unused 1 << 3
    const FL_UNKNOWN0: u32 = 1 << 4;
    const FL_FROZEN: u32 = 1 << 5;
    const FL_ATCONTROLS: u32 = 1 << 6;
    const FL_CLIENT: u32 = 1 << 7;
    const FL_FAKECLIENT: u32 = 1 << 8;
    // Unused 1 << 9
    const FL_FLY: u32 = 1 << 10;
    const FL_UNKNOWN1: u32 = 1 << 11;
    // Unused 1 << 12
    // Unused 1 << 13
    const FL_GODMODE: u32 = 1 << 14;
    const FL_NOTARGET: u32 = 1 << 15;
    const FL_AIMTARGET: u32 = 1 << 16;
    // Unused 1 << 17
    const FL_STATICPROP: u32 = 1 << 18;
    // Unused 1 << 19
    const FL_GRENADE: u32 = 1 << 20;
    const FL_DONTTOUCH: u32 = 1 << 22;
    const FL_BASEVELOCITY: u32 = 1 << 23;
    const FL_WORLDBRUSH: u32 = 1 << 24;
    const FL_OBJECT: u32 = 1 << 25;
    const FL_ONFIRE: u32 = 1 << 27;
    const FL_DISSOLVING: u32 = 1 << 28;
    const FL_TRANSRAGDOLL: u32 = 1 << 29;
    const FL_UNBLOCKABLE_BY_PLAYER: u32 = 1 << 30;
}

#[cfg(target_os = "windows")]
impl Sdk for WindowsSdk {
    fn new() -> Self {
        Self {}
    }

    fn init(&self) -> Result<()> {
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
        let (base_address, size) = memory.get_module("client.dll")?;

        debug!("client base address: 0x{:x} size: {}", base_address, size);

        let local_player_pawn_address: usize = memory.read::<usize>(base_address + cs2::windows::offsets::client_dll::dwLocalPlayerPawn)?;
        debug!("local player pawn address: 0x{:x}", local_player_pawn_address);

        
        loop {
            if unsafe { GetAsyncKeyState(VK_SPACE) } == 0 {
                continue;
            }
            
            let player_flags = memory.read::<u32>(local_player_pawn_address + 0x3D4)?;

            if player_flags & EntityFlag::FL_ONGROUND != 0 {
                debug!("player is on ground");
                memory.write::<u32>(base_address + cs2::windows::offsets::client_dll::dwForceJump, 65537)?;
            } else {
                debug!("player is not on ground");
                if memory.read::<u32>(base_address + cs2::windows::offsets::client_dll::dwForceJump)? == 65537 {
                    memory.write::<u32>(base_address + cs2::windows::offsets::client_dll::dwForceJump, 256)?;
                }
            }
        }

        Ok(())
    }
}
