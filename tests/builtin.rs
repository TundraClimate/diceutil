use diceutil::builtin_dices::*;
use diceutil::Dice;

#[test]
fn normal_roll_test() {
    let dice = NormalDice::new();
    let rolled = dice.roll();
    assert!(rolled.is_positive());
    assert!((1..=6).contains(&rolled));
}

#[test]
fn hundred_roll_test() {
    let dice = HundredDice::new();
    let rolled = dice.roll();
    assert!(rolled.is_positive());
    assert!((1..=100).contains(&rolled));
}

#[test]
fn sized_roll_test() {
    let dice = SizedDice::new(1, 4);
    let rolled = dice.roll();
    assert!(rolled.is_positive());
    assert!((1..=4).contains(&rolled));
}

#[test]
fn multi_roll_test() {
    let dice = MultiDice::new(vec![
        Box::new(NormalDice::new()),
        Box::new(SizedDice::new(1, 12)),
    ]);
    let rolls = dice.roll();
    assert!(rolls[0].is_positive());
    assert!((1..=6).contains(&rolls[0]));
    assert!(rolls[1].is_positive());
    assert!((1..=12).contains(&rolls[1]));
}
