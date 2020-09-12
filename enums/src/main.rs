enum IpAddrKind {
  V4(u8, u8, u8, u8),
  V6(String),
}

enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}


fn main() {
  let four = IpAddrKind::V4(127, 0, 0, 1);
  let six = IpAddrKind::V6(String::from("::1"));

  route(four);
  route(six);

  let x = 5;
  let y = Some(5);

  let sum = match y {
    Some(y) => x + y,
    None => 0
  };


  println!("Hello, world!");
}

fn route(ip_kind: IpAddrKind) {}
