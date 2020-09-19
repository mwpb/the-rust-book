mod vector;
mod string;
mod hash_maps;
mod averages;
mod pig_latin;
mod employeeAdder;

fn main() {
  // vector::run();
  // string::run();
  // hash_maps::run();

  // let mut v = Vec::new();
  // v.push(1);
  // v.push(1);
  // v.push(5);
  // v.push(6);
  // v.push(7);
  // v.push(7);
  // v.push(8);
  // v.push(8);
  // v.push(8);

  // let a = averages::mean(&v);
  // println!("{}", a);

  // let median = averages::median(&v);
  // println!("{}", median);

  // let mode = averages::mode(&v);
  // println!("{:?}", mode);

  // let first = String::from("first apple");
  // let pig_latin = pig_latin::to_pig_latin(first);
  // println!("{}", pig_latin);

  // let second = String::from("Please speak more slowly");
  // let pig_latin_second = pig_latin::to_pig_latin(second);
  // println!("{}", pig_latin_second);

  employeeAdder::run();
}
