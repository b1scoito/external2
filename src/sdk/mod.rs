use color_eyre::eyre::Result;
use log::{debug, info};
use nix::unistd::Pid;
use sysinfo::System;

use crate::memory::{self, LinuxMemory};

pub fn init() -> Result<()> {
    info!("Initializing SDK!");

    // System
    let mut system = System::new();
    system.refresh_all();

    let mut cs2_pid: usize = 0;

    // Get cs2 process ID
    let cs2_pids = system.processes_by_exact_name("cs2");
    for cs2 in cs2_pids {
        cs2_pid = cs2.parent().unwrap().into();
    }

    info!("Found cs2 process with PID: {}", cs2_pid);

    let memory = LinuxMemory::new(Pid::from_raw(cs2_pid as i32));

    // Get client.so
    let mut client_base: usize = 0;
    let process_maps = proc_maps::get_process_maps(cs2_pid as i32)?;
    for map in process_maps {
        if map.filename().is_none() {
            continue;
        }

        if map
            .filename()
            .unwrap()
            .to_str()
            .unwrap()
            .contains("libclient.so")
        {
            debug!("Found client.so at: {:?}", map);
            client_base = map.start();
            let buffer = memory.read::<usize>(client_base)?;
            debug!("Read memory: {:#?}", buffer);
        }
    }

    Ok(())
}
