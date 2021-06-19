mod front_of_house;

pub use crate::front_of_house::hosting;
pub use crate::front_of_house::serving;

mod back_of_house;

// Using `use` keyword with absolute path to bring struct Breakfast into scope
pub use back_of_house::Breakfast;


pub fn eat_at_restaurant() {
  front_of_house::hosting::add_to_waitlist();
  front_of_house::hosting::seat_at_table();
  front_of_house::serving::take_order();

  impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
  }

  front_of_house::serving::serve_order();
  front_of_house::serving::take_payment();

  // Using struct Breakfast and its method summer() without needing to use path because we brought it into scope earlier^
  let mut meal = Breakfast::summer("Rye");
  
  let mut meal2 = Breakfast::summer("Rye");
  // Change our mind about what bread we'd like
  println!("Breakfast: {:?}", meal);
  meal.toast = String::from("Wheat");
  println!("For meal 1 I'd like {} toast please", meal.toast);

  meal2.toast = String::from("White Bread");
  println!("For meal 2 I'd like {} toast please", meal2.toast);

}


// mod front_of_house;

// pub use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }

