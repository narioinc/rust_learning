mod front_of_house;
mod back_of_house;

//example of use keyword to creeate a shorter path for invoking modules
use crate::front_of_house::hosting;
//use crate::front_of_house::serving;

//this can be called by coming via the crate:: or via super:: from internal child modules
fn deliver_order(){
    println!("Delivering order");
    hosting::add_to_waitlist();
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    //after the use statement
    hosting::add_to_waitlist();
}