mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list(){}
    }
}

// relative path
use front_of_house::hosting;

pub fn eat_at_restaurant(){
    hosting::add_to_wait_list();
    hosting::add_to_wait_list();
    hosting::add_to_wait_list();
}