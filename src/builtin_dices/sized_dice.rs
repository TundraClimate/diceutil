use crate::Dice;
use rand::prelude::*;
use std::sync::Mutex;

/// Specify the size and roll the dice.
///
/// A lottery will be drawn within the specified size.
///
/// ### Examples
///
/// ```
/// use diceutil::builtin_dices::SizedDice;
/// use diceutil::Dice;
///
/// # fn main() {
/// let dice = SizedDice::new(1, 12); // 1, 12 <- "size": min, max
/// println!("{}", dice.roll()); // Randomly selected from "size" results.
/// # }
/// ```
pub struct SizedDice {
    rand_thread: Mutex<ThreadRng>,
    min: i32,
    max: i32,
}

impl SizedDice {
    /// Create new `SizedDice`.
    pub fn new(min: i32, max: i32) -> SizedDice {
        SizedDice {
            rand_thread: Mutex::new(rand::thread_rng()),
            min,
            max,
        }
    }
}

impl Dice<i32> for SizedDice {
    /// Roll `SizedDice`. Randomly selected from "size" results.
    fn roll(&self) -> i32 {
        let lock = &mut self.rand_thread.lock().expect("mutex lock failed");
        lock.gen_range(self.min..=self.max)
    }
}
