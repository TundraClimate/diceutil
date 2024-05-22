#[cfg(test)]
mod impl_dice_test {
    use diceutil::Dice;
    struct OneDice;

    impl Dice<usize> for OneDice {
        fn roll(&self) -> usize {
            1
        }
    }

    impl OneDice {
        fn new() -> OneDice {
            OneDice
        }
    }

    #[test]
    fn roll_test() {
        let dice = OneDice::new();
        assert_eq!(dice.roll(), 1);
    }

    #[test]
    fn multi_roll_test() {
        let dice = OneDice::new();
        let res1 = dice.roll();
        let res2 = dice.roll();
        assert_eq!(res1, res2);
    }
}

#[cfg(test)]
mod builtin_dices {
    use diceutil::builtin_dices::*;
    use diceutil::Dice;

    #[test]
    fn normal_roll_test() {
        let dice = NormalDice::new();
        let rolled = dice.roll();
        assert!(rolled.is_positive());
        assert!((1..=6).contains(&rolled));
    }
}
