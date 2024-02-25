use std::sync::{Arc, Mutex};

use color_eyre::eyre::{self, Result};
use rdev::{listen, Event, EventType, Key};

#[derive(Debug)]
pub struct InputSystem {
    keys: Arc<Mutex<Vec<Key>>>,
}

impl InputSystem {
    pub fn new() -> Result<Self> {
        Ok(Self { keys: Arc::new(Mutex::new(Vec::new())) })
    }

    pub fn init_listen_callback(&self) -> Result<()> {
        log::debug!("initializing input system");

        let keys = self.keys.clone();

        let callback = move |event: Event| -> () {
            match event.event_type {
                EventType::KeyPress(key) => {
                    let mut keys = keys.lock().unwrap();
                    if !keys.contains(&key) {
                        keys.push(key);
                    }
                }
                EventType::KeyRelease(key) => {
                    let mut keys = keys.lock().unwrap();
                    keys.retain(|&x| x != key);
                }
                _ => {}
            }
        };

        if let Err(e) = listen(callback) {
            return Err(eyre::eyre!("failed to listen for input events: {:?}", e));
        }

        Ok(())
    }

    pub fn is_key_down(&self, key: Key) -> Result<bool> {
        let keys = self.keys.lock().unwrap();        
        Ok(keys.contains(&key))
    }

    pub fn send_input(&self, key: Key) -> Result<()> {
        rdev::simulate(&EventType::KeyPress(key))?;
        rdev::simulate(&EventType::KeyRelease(key))?;
        
        Ok(())
    }
}