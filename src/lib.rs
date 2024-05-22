#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

//! This library is utility of dice.

/// Implement the `roll` function to roll the dice
///
/// `O` is Output type. result of roll by dice.
///
/// ### Examples
///
/// ```
/// use diceutil::Dice;
///
/// struct MyDice;
///
/// impl Dice<i32> for MyDice {
///   fn roll(&self) -> i32 {
///     32
///   }
/// }
///```
pub trait Dice<O> {
    /// Roll dice.`O` returns the result.
    fn roll(&self) -> O;
}

pub mod builtin_dices;
