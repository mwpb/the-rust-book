use std::collections::HashMap;

pub fn run() {
  let field_name = String::from("Blue");

  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);
  scores.insert(field_name, 60);

  // a HashMap takes ownership of its keys and values
  // println!("{}", field_name);

  let team_name = String::from("Blue");
  let score = scores.get(&team_name);

  let text = "hello world world";
  let mut map = HashMap::new();

  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }

  println!("{:?}", map);
}