use color_eyre::eyre::{self, Result};
use log::{debug, info};
use sysinfo::{Pid, System};

use crate::memory::{linux::LinuxMemory, Memory, Module};

use super::Sdk;

pub struct LinuxSdk {
    modules: Vec<Module>,
    memory: LinuxMemory,
}

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
        let memory = LinuxMemory::new(cs2_pid)?;

        let modules = vec![
            memory.get_module("libclient.so")?,
            memory.get_module("libengine2.so")?,
        ];

        debug!("Modules found: {:?}", modules);

        Ok(Self { modules, memory })
    }

    fn get_memory(&self) -> &LinuxMemory {
        &self.memory
    }

    fn get_module(&self, name: &str) -> Option<&Module> {
        self.modules.iter().find(|module| module.name == name)
    }
}
