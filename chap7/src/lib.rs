
mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {}
        fn sit_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_ordre() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_wait_list();
    // Relative patth
    front_of_house::hosting::add_to_wait_list();
}