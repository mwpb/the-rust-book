mod front_of_house;

fn serve_order() {}

mod back_of_house {
  pub enum Appetizer {
    Soup,
    Salad,
  }
  pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
  }

  impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(toast),
        seasonal_fruit: String::from("peaches"),
      }
    }
  }

  fn fix_incorrect_order() {
    cook_order();
    super::serve_order();
  }

  fn cook_order() {}
}

use crate::front_of_house::hosting as hoho;
use rand::Rng;

pub fn eat_at_restaurant() {
  let mut meal = back_of_house::Breakfast::summer("Rye");
  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please.", meal.toast);

  // meal.seasonal_fruit = String::from("blueberries");

  let order1 = back_of_house::Appetizer::Soup;
  let order2 = back_of_house::Appetizer::Salad;

  // Absolute path
  hoho::add_to_waitlist();

  // Relative path
  hoho::add_to_waitlist();
}
