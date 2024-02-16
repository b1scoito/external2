use std::ffi::{c_char, c_void};

use color_eyre::eyre::Result;
use log::{debug, info};
use sysinfo::{Pid, System};
use tracing_subscriber::field::debug;

#[cfg(target_os = "linux")]
use crate::memory::{LinuxMemory, Memory};

#[cfg(target_os = "windows")]
use crate::memory::{Memory, WindowsMemory};

pub(crate) mod cs2;

struct Module {
    name: String,
    base_address: usize,
    size: usize,
}

// TODO: Is this the best way to make this cross-platform?
#[cfg(target_os = "linux")]
pub struct LinuxSdk {
    modules: Vec<Module>,
    memory: LinuxMemory,
}

#[cfg(target_os = "windows")]
pub struct WindowsSdk {
    modules: Vec<Module>,
    memory: WindowsMemory,
}

pub trait Sdk {
    fn get_module(&self, name: &str) -> Option<&Module>;
    #[cfg(target_os = "linux")]
    fn get_memory(&self) -> &LinuxMemory;
    #[cfg(target_os = "windows")]
    fn get_memory(&self) -> &WindowsMemory;
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

        let cs2_pids = system.processes_by_exact_name("cs2");
        for cs2 in cs2_pids {
            cs2_pid = match cs2.parent() {
                Some(pid) => pid,
                None => Pid::from(0),
            }
        }

        // Get cs2 process ID
        let memory: LinuxMemory = Memory::new(nix::unistd::Pid::from_raw(cs2_pid.as_u32() as i32))?;
        let (client_base_address, client_size) = memory.get_module("libclient.so")?;
        let (engine2_base_address, engine2_size) = memory.get_module("libengine2.so")?;

        debug!(
            "client base address: 0x{:x} size: {}",
            client_base_address, client_size
        );

        debug!(
            "engine2 base address: 0x{:x} size: {}",
            engine2_base_address, engine2_size
        );

        let modules = vec![
            Module {
                name: "libclient.so".to_string(),
                base_address: client_base_address,
                size: client_size,
            },
            Module {
                name: "libengine2.so".to_string(),
                base_address: engine2_base_address,
                size: engine2_size,
            },
        ];

        Ok(Self { modules, memory })
    }

    fn get_memory(&self) -> &LinuxMemory {
        &self.memory
    }

    fn get_module(&self, name: &str) -> Option<&Module> {
        self.modules.iter().find(|module| module.name == name)
    }
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
