pub fn run() {
  let mut v = Vec::new();

  v.push(5);
  v.push(6);
  v.push(7);

  let third = v[2];
  let third_again = v.get(2);

  for i in &mut v {
    *i += 50;
  }

  for j in &v {
    println!("{}", j);
  }
}