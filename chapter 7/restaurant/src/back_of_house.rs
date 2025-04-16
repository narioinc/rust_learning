use crate::front_of_house::hosting;

    //we can do this as well to expose the hosting module inside the back of hosue module

    fn fix_order(){
        println!("Fixing order");

        //this will not work as the hosting module is outside the scope ( front of house module)
        //hosting::add_to_waitlist();

        //this will work as the hosting module is in the same scope
        hosting::add_to_waitlist(); 

        //can call as its sibling
        cook_order();

        //calling using super:: 
        super::deliver_order();

    }   

    fn cook_order(){
        println!("Cooking order");
    } 