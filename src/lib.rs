pub trait Dice<O> {
    fn roll(&self) -> O;
}

pub mod builtin_dices;
