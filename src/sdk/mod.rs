use std::collections::HashMap;

use color_eyre::eyre::Result;
use log::{debug, info};
use sysinfo::System;

use crate::memory::{LinuxMemory, Memory};

pub struct LinuxSdk {}
pub struct WindowsSdk {}

pub trait Sdk {
    fn new() -> Self;
    fn init(&self) -> Result<()>;
}

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

        debug!("found game process with pid: {}", cs2_pid);

        let mut libclient_base_address: usize = 0;

        // Populate base addresses
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

        Ok(())
    }
}

impl Sdk for WindowsSdk {
    fn new() -> Self {
        Self {}
    }

    fn init(&self) -> Result<()> {
        info!("initializing windows sdk");

        Ok(())
    }
}
