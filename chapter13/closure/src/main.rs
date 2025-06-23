use std::vec;

fn main() {
let list = vec![1, 2, 3, 4, 5];

println!("list: {:?}", list);

let borrow = || { 
println!("borrowed {:?}", list); 
//list.push(6);
};

println!("before list: {:?}", list);

borrow();

println!("after list: {:?}", list);





//mutable borrow closure
// in this example, we will see how to use a mutable borrow closure
// to modify the value of a variable
// However, between the definition of the closure and the call to the closure
// the variable is moved, so we cannot use it in any calls
let mut list2 = vec![1, 2, 3, 4, 5];
println!("list: {:?}", list2);

let mut borrow = || { 
println!("borrowed {:?}", list2); 
list2.push(6);
};

//this call cannot be made as the clousre is holding a mutable borrow
//println!("before list: {:?}", list2);

borrow();

//println!("after list: {:?}", list2);

//borrow();

println!("after list: {:?}", list2);


}