pub fn if_statements(num: i32, max: i32) -> () {

    let max = 2 * max;
    if num < max {
        println!("{num} is less than {max}");
    } else if num > max{
        println!("{num} is greater than {max}");
    } else {
        println!("{num} is equal to {max}");
    }
}

pub fn if_state_2() {
    let condition = true; 
    // let number = if condition {6} else {"seven"}; won't work because different types 
    let number = if condition {6} else {7}; 

    println!("Number is {number}");
}

pub fn loop_prac(){
    let mut x = 1;
    loop{
        println!("{x}!");
        x += 1; // ooh this works
        if x >= 10 {
            println!("It's the ten duel commandments!");
            // break; // breaks out of loop
            return; // returns from function which is more effective in this case
        }
    }
}

pub fn loop_label() {
    let mut count = 0; 
    'counting_up: loop {
        println!("count = {count}!");
        let mut remaining = 10; 
        loop{
            println!("Remaining = {remaining}");
            if remaining == 6{
                break;
            } 
            if count == 3{
                break 'counting_up;
            }
            remaining -= 1; 
        }
        count += 1; 
    }
    println!("End count = {count}");
}

pub fn while_loop() {

    let mut number = 3; 

    while number != 8 {
        println!("{number}");
        number += 1; 
    }

    println!("Six-Seven Achieved");
}

pub fn for_loop() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

pub fn for_loop_2(){
    for number in (0..100).rev(){ // 100 is not included
        println!("{number}");
    }
    println!("🤑");

    for number in (0..=100).rev(){ // 100 is included
        println!("{number}");
    }
    println!("🐐");
}

pub fn fibonacci_seq(i: i32){
    let mut curr = 0; 
    let mut prev = 1; 
    for num in 0..i{
        if num > 0{
            let next = curr + prev; 
            prev = curr; 
            curr = next; 
            println!("{prev}, {curr}, {next}");
        }
    }
    println!("The {i}th value of the fibonacci sequence is: {curr}");
}