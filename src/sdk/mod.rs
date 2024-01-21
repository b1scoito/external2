use color_eyre::eyre::Result;
use log::{debug, info};
use sysinfo::System;

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

    // Get client.so
    let process_maps = proc_maps::get_process_maps(cs2_pid as i32)?;
    for process_map in process_maps {
        if process_map.
    }

    Ok(())
}
