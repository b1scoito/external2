use std::ffi::{c_char, c_void};

use color_eyre::eyre::Result;
use log::info;
use sysinfo::{Pid, System};

#[cfg(target_os = "linux")]
use crate::memory::{LinuxMemory, Memory};

#[cfg(target_os = "windows")]
use crate::memory::{Memory, WindowsMemory};

pub(crate) mod cs2;

// TODO: Is this the best way to make this cross-platform?
#[cfg(target_os = "linux")]
pub struct LinuxSdk {
    local_player_pawn_address: usize,
    client_base_address: usize,
    client_size: usize,
    memory: LinuxMemory,
    global_vars: GlobalVarsBase,
}

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
    #[cfg(target_os = "linux")]
    fn get_memory(&self) -> &LinuxMemory;
    #[cfg(target_os = "windows")]
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
    fn new() -> Result<Self> {
        info!("initializing linux sdk");

        // System
        let mut system = System::new();
        system.refresh_all();

        let mut cs2_pid: Pid = Pid::from(0);

        // system.processes_by_name("cs2").iter().for_each(|cs2| {
        //     cs2_pid = cs2.pid();
        // });

        // Get cs2 process ID
        let memory: LinuxMemory = Memory::new(nix::unistd::Pid::from_raw(cs2_pid.as_u32() as i32))?;
        let (client_base_address, client_size) = memory.get_module("libclient.so")?;
        let (engine2_base_address, engine2_size) = memory.get_module("libengine2.so")?;

        // let local_player_pawn_address: usize = memory.read::<usize>(
        //     client_base_address + cs2::linux::offsets::client_dll::dwLocalPlayerPawn,
        // )?;
        let local_player_pawn_address: usize = 0;

        Ok(Self {
            local_player_pawn_address,
            client_base_address,
            client_size,
            memory,
            global_vars: GlobalVarsBase {
                real_time: todo!(),
                frame_count: todo!(),
                frame_time: todo!(),
                absolute_frame_time: todo!(),
                max_clients: todo!(),
                pad_0: todo!(),
                frame_time_2: todo!(),
                current_time: todo!(),
                current_time_2: todo!(),
                pad_1: todo!(),
                tick_count: todo!(),
                pad_2: todo!(),
                network_channel: todo!(),
                pad_3: todo!(),
                current_map: todo!(),
                current_map_name: todo!(),
            },
        })
    }

    fn get_memory(&self) -> &LinuxMemory {
        todo!()
    }

    fn get_client_size(&self) -> usize {
        todo!()
    }

    fn get_client_base_address(&self) -> usize {
        todo!()
    }

    fn get_local_player_pawn_address(&self) -> usize {
        todo!()
    }

    fn get_global_vars(&self) -> &GlobalVarsBase {
        todo!()
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
        debug!(
            "client base address: 0x{:x} size: {}",
            client_base_address, client_size
        );

        let (engine_base_address, engine_size) = memory.get_module("engine2.dll")?;
        debug!(
            "engine base address: 0x{:x} size: {}",
            engine_base_address, engine_size
        );

        let local_player_pawn_address: usize = memory.read::<usize>(
            client_base_address + cs2::windows::offsets::client_dll::dwLocalPlayerPawn,
        )?;
        debug!(
            "local player pawn address: 0x{:x}",
            local_player_pawn_address
        );

        debug!(
            "global vars addres: {}",
            client_base_address + cs2::windows::offsets::client_dll::dwGlobalVars
        );

        let global_vars = memory.read::<GlobalVarsBase>(
            client_base_address + cs2::windows::offsets::client_dll::dwGlobalVars,
        )?;

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
