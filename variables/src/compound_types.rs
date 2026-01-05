pub fn tuple_prac() {
    let tup: (i32, f64, u8) = (-600, 6.7, 255);

    let (x, y, z) = tup; // must use pattern matching to destructure a tuple
    let first_value = tup.0; let second_value = tup.1; let third_value = tup.2; 

    println!("First val = {x}, Second value = {y}, Third value = {z}");
    println!("First val = {first_value}, Second value = {second_value}, Third value = {third_value}");
}

pub fn array_prac() {

    let months = ["January", "February", "March", "April", "May", "June", "July", "August",
                            "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5]; // type and size must be in declaration 

    let b = [3; 5]; // will declare an array with the value 3 repeated 5 times

    let first = a[0];
    let third = b[3];
    let  bmonth = months[7];

    println!("first = {first}, third = {third}, bmonth = {bmonth}");
}

pub fn func_prac(x: i32) {
    println!("The input value is: {x}");
}

pub fn state_exp() {
    //let x = let y = 6; will not work
    let y = {
        let x = 3; 
        // x + 1; expressions cannot have semicolons or else they won't return a value 
        x + 1
    };
    let x = 43;
    println!("x = {x}, y = {y}");
}

pub fn six_seven() -> i32{
    // return 67;
    67 // just return it using expression since that always returns
}

pub fn even_odd(x: i32) -> i32{
    x % 2
}