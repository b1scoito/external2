use std::{thread, time::Duration};

use color_eyre::eyre::{self, Ok, Result};

use crate::sdk::cs2::{Client, Cs2};

pub mod bhop;

struct CheatState {
    last_tick: f32,
    last_frame: i32,
}

impl CheatState {
    fn new() -> Self {
        Self::default()
    }

    fn cheat_impl<F>(&mut self, cs2: &Cs2, func: F) -> Result<()>
    where
        F: Fn(&Cs2) -> Result<()>,
    {
        let global_vars = cs2.get_global_vars()?;
        
        // Only update each tick
        let update = global_vars.tick_count != self.last_tick || global_vars.frame_count != self.last_frame;
        if !update {
            // Sleep for 1 tick
            thread::sleep(Duration::from_micros(15625));
            return Err(eyre::eyre!("no need to update"));
        }

        thread::sleep(Duration::from_secs_f32(global_vars.absolute_frame_time));

        func(cs2)?;

        self.last_tick = global_vars.tick_count;
        self.last_frame = global_vars.frame_count;

        Ok(())
    }

}

impl Default for CheatState {
    fn default() -> Self {
        Self {
            last_tick: 0.0,
            last_frame: 0,
        }
    }
}