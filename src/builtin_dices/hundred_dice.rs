use crate::Dice;
use rand::prelude::*;
use std::sync::Mutex;

/// Has 100-sided dice behavior.
///
/// The possible outcome of a 100-sided dice is returned every time you call `roll`.
///
/// ### Examples
///
/// ```
/// use diceutil::builtin_dices::HundredDice;
/// use diceutil::Dice;
///
/// # fn main() {
/// let dice = HundredDice::new();
/// println!("{}", dice.roll()); // Randomly selected from 1..=100 results.
/// # }
/// ```
pub struct HundredDice {
    rand_thread: Mutex<ThreadRng>,
}

impl HundredDice {
    /// Create new `HundredDice`.
    pub fn new() -> HundredDice {
        HundredDice {
            rand_thread: Mutex::new(rand::thread_rng()),
        }
    }
}

impl Default for HundredDice {
    fn default() -> Self {
        HundredDice::new()
    }
}

impl Dice<i32> for HundredDice {
    /// Roll `HundredDice`. Randomly selected from 1..=100 results.
    fn roll(&self) -> i32 {
        let lock = &mut self.rand_thread.lock().expect("mutex lock failed");
        lock.gen_range(1..=100)
    }
}
