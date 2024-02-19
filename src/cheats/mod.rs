use std::{thread, time::{Duration, Instant}};

use color_eyre::eyre::{self, Ok, Result};

use crate::sdk::cs2::{Client, Cs2};

pub mod bhop;

struct CheatState {
    last_tick: f32,
    last_frame: i32,
    elapsed: f32,
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
            thread::sleep(Duration::from_secs_f32(0.015625));
            return Err(eyre::eyre!("not updating"));
        }

        let frametime = global_vars.absolute_frame_time;
        let delay = self.elapsed - if frametime < 0.015625 {
            frametime * 0.5
        } else {
            frametime
        };
        let sleep_duration = f32::min(delay, frametime * 1000.0);

        thread::sleep(Duration::from_millis(sleep_duration as u64));

        let start = Instant::now();

        func(cs2)?;

        // Update last frame and last tick
        self.last_tick = global_vars.tick_count;
        self.last_frame = global_vars.frame_count;

        let elapsed = start.elapsed();
        self.elapsed = elapsed.as_secs_f32() * 1000.0; // Convert to milliseconds

        Ok(())
    }

}

impl Default for CheatState {
    fn default() -> Self {
        Self {
            last_tick: 0.0,
            last_frame: 0,
            elapsed: 0.0,
        }
    }
}