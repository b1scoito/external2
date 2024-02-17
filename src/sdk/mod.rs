use color_eyre::eyre::{self, Result};
use log::{debug, info};
use sysinfo::{Pid, System};

use crate::memory::Module;
#[cfg(target_os = "linux")]
use crate::memory::{Linux, Memory};

#[cfg(target_os = "windows")]
use crate::memory::{Memory, Windows};

pub mod cs2;
pub mod entity;

// TODO: Is this the best way to make this cross-platform?
#[cfg(target_os = "linux")]
pub struct LinuxSdk {
    modules: Vec<Module>,
    memory: Linux,
}

#[cfg(target_os = "windows")]
pub struct WindowsSdk {
    modules: Vec<Module>,
    memory: Windows,
}

pub trait Sdk {
    fn get_module(&self, name: &str) -> Option<&Module>;
    #[cfg(target_os = "linux")]
    fn get_memory(&self) -> &Linux;
    #[cfg(target_os = "windows")]
    fn get_memory(&self) -> &Windows;
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

        if cs2_pid == Pid::from(0) {
            return Err(eyre::eyre!("failed to find cs2 process"));
        }

        // Get cs2 process ID
        let memory: Linux = Memory::new(nix::unistd::Pid::from_raw(cs2_pid.as_u32() as i32))?;

        let modules = vec![
            memory.get_module("libclient.so")?,
            memory.get_module("libengine2.so")?,
        ];

        debug!("Modules found: {:?}", modules);

        Ok(Self { modules, memory })
    }

    fn get_memory(&self) -> &Linux {
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

        let memory: Windows = Memory::new(cs2_pid.into())?;
        let modules = vec![
            memory.get_module("client.dll")?,
            memory.get_module("engine2.dll")?,
        ];

        debug!("Modules found: {:?}", modules);

        Ok(Self {
            memory,
            modules,
        })
    }

    fn get_memory(&self) -> &Windows {
        &self.memory
    }

    fn get_module(&self, name: &str) -> Option<&Module> {
        match name {
            "client" => {
                #[cfg(target_os = "windows")]
                return self.modules.iter().find(|module| module.name == "client.dll");

                #[cfg(target_os = "linux")]
                return self.modules.iter().find(|module| module.name == "libclient.so");
            },
            "engine2" => {
                #[cfg(target_os = "windows")]
                return self.modules.iter().find(|module| module.name == "engine2.dll");

                #[cfg(target_os = "linux")]
                return self.modules.iter().find(|module| module.name == "libengine2.so");
            },
            _ => None,
        }
    }
}
