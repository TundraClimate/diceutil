use crate::Dice;
use rand::prelude::*;
use std::sync::Mutex;

/// Has standard dice behavior.
///
/// The possible outcome of a 6-sided die is returned every time you call `roll`.
///
/// ### Examples
///
/// ```
/// use diceutil::builtin::NormalDice;
/// use diceutil::Dice;
///
/// # fn main() {
/// let dice = NormalDice::new();
/// println!("{}", dice.roll()); // Randomly selected from 1..=6 results.
/// # }
/// ```
pub struct NormalDice {
    rand_thread: Mutex<ThreadRng>,
}

impl NormalDice {
    /// Create new `NormalDice`.
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
    /// Roll `NormalDice`. Randomly selected from 1..=6 results.
    fn roll(&self) -> i32 {
        let lock = &mut self.rand_thread.lock().expect("mutex lock failed");
        lock.gen_range(1..=6)
    }
}
