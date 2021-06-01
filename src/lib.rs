mod front_of_house {
  pub mod hosting {
      pub fn add_to_waitlist() {}

      fn seat_at_table() {}
  }

  mod serving {
      fn take_order() {}

      fn serve_order() {}

      fn take_payment() {}
  }
}


mod back_of_house {
  #[derive(Debug)]
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
}

pub fn eat_at_restaurant() {

  // Order a breakfast in the summer with Rye toast
  let mut meal = back_of_house::Breakfast::summer("Rye");

  // Using `use` keyword with absolute path to bring struct Breakfast into scope 
  use crate::back_of_house::Breakfast;

  // Using struct Breakfast and its method summer() without needing to use path because we brought it into scope earlier^
  let mut meal2 = Breakfast::summer("Rye");

  // Change our mind about what bread we'd like
  println!("Breakfast: {:?}", meal);
  meal.toast = String::from("Wheat");
  println!("For meal 1 I'd like {} toast please", meal.toast);

  meal2.toast = String::from("White Bread");
  println!("For meal 2 I'd like {} toast please", meal2.toast);

  // The next line won't compile if we uncomment it; we're not allowed
  // to see or modify the seasonal fruit that comes with the meal
  // meal.seasonal_fruit = String::from("blueberries");
}
