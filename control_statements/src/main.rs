fn main() {
    let _y = if true {12} else {5};

    // loop {       // Infinite
    //     println!("HI");
    // }

    // let mut idx = 0;

    // let x = loop {
    //     idx += 1;

    //     if idx == 10 {
    //         break idx * 2;  // returns idx * 2 i.e 20 and breaks the loop
    //     }
    // };

    // println!("x = {}", x);
    // println!();
    // println!();
    
    let mut count = 0;

    'counting_loop :loop {
        let mut remaining_count = 10;
        println!("Count: {}", count);

        loop {
            println!("Remaining Count: {}", remaining_count);
            if remaining_count == 5 {
                break;
            }
            if count == 2 {
                break 'counting_loop;
            }
            remaining_count -= 1;
        }
        count += 1;
    }
    println!("Final Count = {}", count);
}

/*
    if <condition> {}
    else {}

    if <condition> {}
    else if <condition> {}
    else {}
*/

/*
    Types of loops:
        loop {}, while {}, for {}
*/