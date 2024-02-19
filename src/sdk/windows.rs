use std::{sync::Arc, thread, time::Duration};

use color_eyre::eyre::{self, Result};
use sysinfo::{Pid, System};

use crate::{memory::{windows::Windows, Memory, Module}, sdk::Games};

use super::{inputsystem::InputSystem, Sdk};

#[derive(Debug, Clone)]
pub struct WindowsSdk {
    modules: Vec<Module>,
    memory: Windows,
    game: Games,
    input_system: Arc<InputSystem>,
}

impl Sdk for WindowsSdk {
    fn new() -> Result<Self> {
        log::info!("initializing windows sdk");

        let input_system = InputSystem::new()?;
        let mut system = System::new();
        let mut found_game: Option<Games> = None;
        let mut game_pid: Pid = Pid::from(0);
    
        while found_game.is_none() {
            system.refresh_all();
    
            for (process_name, game) in Games::process_names() {
                let processes = system.processes_by_exact_name(process_name);
                for process in processes {
                    found_game = Some(game);
                    game_pid = process.pid();
                    break;
                }
    
                if found_game.is_some() {
                    break;
                }
            }
    
            if found_game.is_none() {
                log::info!("waiting for supported game process... list of supported games: {:?}", Games::process_names());
                thread::sleep(Duration::from_secs(5)); // Wait for 5 seconds before retrying
            }
        }
    
        log::info!("found {:?} process with PID: {}", found_game.unwrap(), game_pid.as_u32());
        
        // Assuming `Memory` is properly implemented elsewhere
        let memory: Windows = Memory::new(game_pid)?;
        let mut modules = Vec::new();

        match found_game {
            Some(game) => {
                match game {
                    Games::Cs2 => {
                        log::debug!("waiting for navsystem.dll to be loaded...");
                        loop {
                            match memory.get_module("navsystem.dll") {
                                Ok(module) => {
                                    log::debug!("navsystem.dll loaded, base address: {:#X}", module.base_address);
                                    break;
                                },
                                Err(_) => {
                                    log::debug!("waiting for navsystem.dll to be loaded...");
                                    thread::sleep(Duration::from_secs(5));
                                }
                            }
                        }

                        modules.push(memory.get_module("client.dll")?);
                        modules.push(memory.get_module("engine2.dll")?);
                        modules.push(memory.get_module("inputsystem.dll")?);
                    
                        log::debug!("Modules loaded: {:?}", modules);
                    },
                }
            },
            None => {
                return Err(eyre::eyre!("No supported game process found"));
            },
        }
    
    
        
        Ok(Self { memory, modules, game: found_game.unwrap(), input_system: Arc::new(input_system)})
    }

    #[inline]
    fn get_memory(&self) -> &Windows {
        &self.memory
    }

    #[inline]
    fn get_module(&self, name: &str) -> Option<&Module> {
        self.modules.iter().find(|module| module.name == name)
    }

    #[inline]
    fn get_game(&self) -> Games {
        self.game
    }

    #[inline]
    fn get_input_system(&self) -> Arc<InputSystem> {
        self.input_system.clone()
    }
}
