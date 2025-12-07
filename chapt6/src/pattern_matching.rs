
fn _filler__() {
  let some_u8_value = 0u8;

  match some_u8_value {
    1 => println!("one"),
    2 => println!("two"),
    4 => println!("four"),
    7 => println!("seven"),
    _ => (),
  }
}

pub fn execute_match_only_when_num_is_3() {
  // Possible syntax with the "match" keyword
  let some_u8_value = Some(0u8);
  match some_u8_value {
    Some(3) => println!("three!"),
    _ => (),
  }

  // No boilerplate syntax with the "if let" syntax
  if let Some(3) = some_u8_value {
    println!("three!")
  }
}