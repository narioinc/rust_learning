fn main() {
    // Loops

    //Infinite loop
    //loop {
    //    println!("looping");
    //}

    //Loop with break

    let mut counter = 0;
    loop {
        println!("counter is : {}", counter);

        counter += 1;
        if counter == 5 {
            break;
        }
    }

    //Loop as expression
    let mut counter2 = 0;
    let result = loop{
        println!("counter2 is : {}", counter2);
        counter2 += 1;
        if counter2 == 5 {
            break counter2 * 2;
        }

    };
    println!("result is : {}", result);

    //Loop with labels
    let mut rows = 0;
    'countRow: loop {
        let mut cols = 0;
        loop {
            if cols < 5 {
                print!("*");
            }else{
               break;
            }
            cols += 1;
            
            
        }
        println!();
        rows += 1;
        if rows == 5 {
            break 'countRow
        }
        
    }

    //while loop

    let mut counter3 = 0;
    while counter3 < 5 {
       println!("counter3 is : {}", counter3);
       counter3 += 1;
    }

    // for-in loop
    let a: [i32; 6] = [1, 2, 3, 4, 5, 7];
    for element in a {
        println!("i is : {}", element);
    }

    for index in 1..6 {
        println!("i is : {}", index);
    }
}
