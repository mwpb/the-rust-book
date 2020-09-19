pub fn to_pig_latin(s: String) -> String {
  let words = s.split(" ");
  let mut pig_latin_words = Vec::new();
  for word in words {
    pig_latin_words.push(convert_word(word));
  }

  let pig_latin = pig_latin_words.join(" ");
  return pig_latin;
}

fn convert_word(s: &str) -> String {
  let mut t = String::from(s);
  if starts_with_vowel(&t) {
    t.push_str("-hay");
    return t;
  } else {
    let first_char = t.remove(0);
    t.push('-');
    t.push(first_char.to_ascii_lowercase());
    t.push_str("ay");
    
    if first_char.is_uppercase() {
      let second_char = t.remove(0);
      return format!("{}{}", second_char.to_ascii_uppercase(), t);
    }

    return t;
  }
}

fn starts_with_vowel(s: &String) -> bool {
  for vowel in &["a", "e", "i", "o", "u"] {
    if s.starts_with(vowel) {
      return true;
    }
  }
  return false;
}