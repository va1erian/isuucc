
#[derive(Serialize, Deserialize, Debug)]
pub enum InputButtons {
  Up,
  Down,
  Left,
  Right,
  Fire,
  Item1,
  Item2
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Input {
  up : bool,
  down : bool,
  left : bool,
  right : bool,
  fire : bool,
  item1 : bool,
  item2: bool
}

impl Input {
  fn new() -> Input {
    Input {
      up: false,
      down: false,
      left: false,
      right: false,
      fire: false,
      item1: false,
      item2: false
    }
  }
}