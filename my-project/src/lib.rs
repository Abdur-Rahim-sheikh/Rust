mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            return Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            };
        }
    }
}

use crate::back_of_house::Breakfast;
// same
// use self::back_of_house::Breakfast;

pub fn eat_at_restaurant() {
    let mut meal = Breakfast::summer("loava");

    meal.toast = String::from("wheat")
}
