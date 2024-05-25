use crate::Dice;

/// Roll multiple dice.
///
/// Roll multiple dice at once and get multiple results.
///
/// ### Examples
///
/// ```
/// use diceutil::Dice;
/// use diceutil::builtin_dices::*;
///
/// # fn main() {
/// let multi_dice = MultiDice::new(vec![
///     Box::new(NormalDice::default()),
///     Box::new(HundredDice::default()),
/// ]);
///
/// let rolled = multi_dice.roll();
/// # }
/// ```
pub struct MultiDice<T> {
    dices: Vec<Box<dyn Dice<T>>>,
}

impl<T> MultiDice<T> {
    /// Create new `MultiDice` from [`Box<Dice<T>>`]
    ///
    /// ### Arguments
    ///
    /// * dices - Dice vector wrapped in Box
    ///
    /// [`Box<Dice<T>>`]: Box
    pub fn new(dices: Vec<Box<dyn Dice<T>>>) -> MultiDice<T> {
        MultiDice { dices }
    }
}

impl<T> Dice<Vec<T>> for MultiDice<T> {
    /// Roll the dice in [`Vec`] in order.
    fn roll(&self) -> Vec<T> {
        let rolled: Vec<_> = self.dices.iter().map(|d| d.roll()).collect();
        rolled
    }
}
