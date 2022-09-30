mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {}

        fn seat_at_table() {}
    }

   pub mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}

       pub mod back_of_house {

           pub enum Appetizer {
               Soup,
               Salad,
           }

            pub struct Breakfast {
                pub toast: String,
                seasonal_fruit: String,
            }

            impl Breakfast {
                pub fn summer(toast:&str) -> Breakfast {
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
        }
    }
}

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_wait_list();

    // relative path
    // todo error message: could not find `front_of_house` in the crate root
    // front_of_house::hosting::add_to_wait_list();

    let mut meal = front_of_house::serving::back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please!", meal.toast);

    // error: seasonal_fruit is not public
    // meal.seasonal_fruit = String::from("blueberries");
}
