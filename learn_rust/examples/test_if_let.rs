enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(String),
}

fn process_coin(coin: Coin) -> Option<String> {
  let Coin::Quarter(state) = coin else {
      return None;
  };
  // 如果 coin 是 Quarter，state 就被解构出来
  Some(state)
}

fn main() {
  let coin = Coin::Quarter("USA".to_string());
  println!("{:?}", process_coin(coin)); // 输出 Some("USA")

  let coin = Coin::Dime;
  println!("{:?}", process_coin(coin)); // 输出 None
}
