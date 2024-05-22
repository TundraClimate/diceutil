use crate::Dice;
use rand::prelude::*;
use std::sync::Mutex;

pub struct NormalDice {
    rand_thread: Mutex<ThreadRng>,
}

impl NormalDice {
    pub fn new() -> NormalDice {
        NormalDice {
            rand_thread: Mutex::new(rand::thread_rng()),
        }
    }
}

impl Default for NormalDice {
    fn default() -> Self {
        NormalDice::new()
    }
}

impl Dice<i32> for NormalDice {
    fn roll(&self) -> i32 {
        let lock = &mut self.rand_thread.lock().expect("mutex lock failed");
        lock.gen_range(1..=6)
    }
}
