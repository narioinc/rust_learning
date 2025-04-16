mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){
            println!("Adding to waitlist");
        }
    }
    mod serving{
        fn take_order(){
            println!("Taking order");
        }
        fn serve_order(){
            println!("Serving order");
        }
        fn take_payment(){
            println!("Taking payment");
        }   
    }

}

//example of use keyword to creeate a shorter path for invoking modules
use crate::front_of_house::hosting;
//use crate::front_of_house::serving;

//this can be called by coming via the crate:: or via super:: from internal child modules
fn deliver_order(){
    println!("Delivering order");
}

mod back_of_house{
    //we can do this as well to expose the hosting module inside the back of hosue module
    //use crate::front_of_house::hosting;

    fn fix_order(){
        println!("Fixing order");

        //this will not work as the hosting module is outside the scope ( front of house module)
        //hosting::add_to_waitlist();

        //this will work as the hosting module is in the same scope
        super::hosting::add_to_waitlist(); 

        //can call as its sibling
        cook_order();

        //calling using super:: 
        super::deliver_order();

    }   

    fn cook_order(){
        println!("Cooking order");
    } 
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}