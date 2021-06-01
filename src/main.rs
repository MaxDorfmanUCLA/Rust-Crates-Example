// To import restaurant we can use:
extern crate restaurant;
// OR:
use restaurant::eat_at_restaurant;

fn main() {
  restaurant::eat_at_restaurant();
  eat_at_restaurant();
}

