fn main() {

    //struct instance + access fields
    
    let employee = Employee {
        name: String::from("Naresh K"),
        age: 30,
        salary: 50000.0,
        position: String::from("Software Engineer"),
    };

    println!("Employee Name: {}", employee.name);
    println!("Employee Age: {}", employee.age);

    //Mutable struct instance
    let mut employee2 = Employee {
        name: String::from("Naresh Krishnamoorthy"),
        age: 30,
        salary: 50000.0,
        position: String::from("Software Engineer"),
    };
    employee2.age = 25;
   


    //not possible to log employee object like this since employee does not implement the Debug trait
    //println!("employee: {:?}", employee);

    println!("Employee Name: {}", employee2.name);
    println!("Employee Age: {}", employee2.age);

    reset_salary(&mut employee2);
    log_user(&employee2);

    reset_salary2(&mut employee2, 20000.0); 
    log_user(&employee2);

    //use the struct update syntax
    let user2: Employee = Employee {
        name: String::from("Chitra K"),
        ..employee2
    };
    log_user(&user2);
    // This cant be done because user two did a update syntax copy over but only for one of the string fields, the position string field still
    // got invalidated during the update copy over. Partial move error below
    //log_user(&employee2);

    //tuple structs
    struct Rect(i32,i32);
    let rect = Rect(10, 20);
    println!("Rect: {} {}", rect.0, rect.1);

    //A simple program to calculate the area of a rectangle
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let rectangle2 = Rectangle {
        width: 20,
        height: 70,
    };

    let square1: Rectangle = Rectangle::square(10);

    println!("Area of rectangle: {}", rectangle.area());   
    println!("rect 1 can fit rect 2: {}", rectangle.can_hold(&rectangle2));
    println!("area of square  : {}", square1.area()); 

}

#[derive(Debug)]
struct Employee{
    name: String,
    age: u32,
    salary: f64,
    position: String,
}

// mutate struct instance with reference
fn reset_salary(employee: &mut Employee){
    employee.salary = 10000.0;
}

// mutate struct instance with reference and field init shorthand
fn reset_salary2(employee: &mut Employee, salary: f64){
    employee.salary = salary;
}

fn log_user(employee: &Employee){
    println!("*********Employees details*********");
    println!("{:#?}", employee);
    print!("Name: {}", employee.name);

}

//Define a rectangle struct

struct Rectangle {
    width: u32,
    height: u32,

}

//struct implementation +
impl Rectangle{
     //function inside a struct
     fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self{
        Self {
            width: size,
            height: size,
        }
    }
}

//function to calcuate the area of reactanle outside the struct ( taking a struct as arg)

fn area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}