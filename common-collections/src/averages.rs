use std::collections::HashSet;
use std::collections::HashMap;

pub fn mean(v: &Vec<i32>) -> f64 {
  let n = v.len();
  let mut sum = 0;
  for i in v {
    sum += i;
  }

  return (sum as f64) / (n as f64);
}

pub fn median(v: &Vec<i32>) -> f64 {
  let n = v.len();
  let mut w = Vec::clone(v);
  w.sort();
  if n % 2 != 0 {
    let median_option = w.get(n / 2);
    match median_option {
      Some(median) => return *median as f64,
      None => return 0.,
    }
  } else {
    match (w.get(n / 2 - 1), w.get(n / 2)) {
      (Some(l), Some(r)) => return ((l + r) as f64) / 2.,
      _ => return 0.,
    }
  }
}

pub fn mode(v: &Vec<i32>) -> HashSet<i32> {
  let mut frequencies: HashMap<i32, i32> = HashMap::new();
  for i in v {
    let count = frequencies.entry(*i).or_insert(0);
    *count += 1;
  } 
  
  let mut maximum = 0;
  for (_, value) in &frequencies {
    if *value > maximum {
      maximum = *value;
    }
  }

  if maximum <= 1 {
    return HashSet::new();
  }
  
  let mut modes = HashSet::new();
  for (key, value) in &frequencies {
    if *value == maximum {
      modes.insert(*key);
    }
  }

  return modes;
}