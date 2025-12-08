mod front_of_house;

pub mod back_of_house {
    pub mod nested_inline_module {
        
    }
  pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
  }
  impl Breakfast {
      pub fn summer(toast: &str) -> Breakfast {
          Breakfast {
              toast: String::from(toast),
              seasonal_fruit: String::from("peach"),
          }
      }
  }
}

// absolute path
use crate::front_of_house::hosting;
// alias
use crate::front_of_house::hosting as h;
// relative path
use self::front_of_house::hosting as hosting_relative;
// Renaming a type
use std::fmt::Result;
use std::io::Result as IoResult;
// re-exporting - external caller may now use hosting::add_to_waitlist()
pub use crate::front_of_house::hosting as h2;


pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    h::add_to_waitlist();
    crate::front_of_house::hosting::waiting::wait();

    let meal = crate::back_of_house::Breakfast::summer("dark bread");
    println!("I'd like to have a toast on {}", meal.toast);
}

fn fn1() -> Option<Result> {
    return None
}
fn fn2() -> Option<IoResult<String>> {
    return None
}