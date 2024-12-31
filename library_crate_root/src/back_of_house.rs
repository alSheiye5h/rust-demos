pub fn fix_incorrect_order() {
    cook_order();
    // FILL in the blank in three ways
    //1. using keyword `super`
    //2. using absolute path
    super::front_of_house::serving::serve_order();
}

pub fn cook_order() {}