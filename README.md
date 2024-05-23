# diceutil

ダイス関連のいろいろを詰め込んだライブラリ(自分用)

# features

**trait** `Dice`

```rs
struct MyDice;

impl Dice<i32> for MyDice {
  fn roll(&self) -> i32 {
    32
  }
}
```

**builtin** `NormalDice`

```rs
use diceutil::builtin::NormalDice;
use diceutil::Dice;

let dice = NormalDice::new();
println!("{}", dice.roll()); // 1..=6の範囲で数字がランダムに返される
```

**builtin** `HundredDice`

```rs
use diceutil::builtin::HundredDice;
use diceutil::Dice;

let dice = HundredDice::new();
println!("{}", dice.roll()); // 1..=100の範囲で数字がランダムに返される
```

# todo

- ドキュメント書いたほうがいいよね〜〜
- 返却される値が複数のダイスも作りたーい！
