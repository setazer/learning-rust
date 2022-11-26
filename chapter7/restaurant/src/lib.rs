use crate::front_of_house::hosting;

mod front_of_house;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    let mut meal = back_of_house::Breakfast::summer("oatmeal");
    meal.toast = String::from("wheat");
    println!("I'd like {} toast, please", meal.toast);
    let order1 = back_of_house::Appetizer::Salad;
    //meal.seasonal_fruit = String::from("banana")

}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { toast: String::from(toast), seasonal_fruit: String::from("peaches") }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
        
    }
    fn cook_order() {}
}