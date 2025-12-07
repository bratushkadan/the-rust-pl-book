pub enum Message {
  Quit,
  // Anonymous struct
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}