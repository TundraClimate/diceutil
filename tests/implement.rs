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
