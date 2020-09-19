pub fn run() {
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");

  // format doesn't take ownership of any of its parameters
  let s = format!("{}-{}-{}", s1, s2, s3);
  println!("{}", s);

  for c in "नमस्ते".chars() {
    println!("{}", c);
  }

  for b in "नमस्ते".bytes() {
    println!("{}", b);
  }
}