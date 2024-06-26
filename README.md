# diceutil

ダイス関連のいろいろを詰め込んだライブラリ(自分用)  
[docs](https://tundraclimate.github.io/diceutil/diceutil/)

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

<details>
<summary>Builtin-dices</summary>

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

**builtin** `SizedDice`

```rs
use diceutil::builtin::SizedDice;
use diceutil::Dice;

let dice = SizedDice::new(1, 4); // min, max
println!("{}", dice.roll()); // min..=maxの範囲で数字がランダムに返される
```

**builtin** `MultiDice`

```rs
use diceutil::builtin::*;
use diceutil::Dice;

let dice = MultiDice::new(vec![
    Box::new(NormalDice::default()),
    Box::new(HundredDice::default()),
]);
let rolls = dice.roll(); // ダイスを2個振る
println!("{}, {}", &rolls[0], &rolls[1]);
```

</details>

# todo

- ドキュメント書いたほうがいいよね〜〜
- 返却される値が複数のダイスも作りたーい！
