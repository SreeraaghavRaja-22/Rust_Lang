// fn main(){
//     let mut x = 5;  
//     println!("The value of x is: {x}");
//     x = 6; 
//     println!("The value of x is: {x}");
// }

mod compound_types;
use compound_types::*;


fn main() {
    let x = 5; 

    let x = x + 1; 

    {
        let x = x * 2; 
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // let mut spaces = "    ";
    let spaces = "      ";
    {
        // spaces = spaces.len(); will not work because spaces cannot change type with mut
        let spaces = spaces.len();
        println!("The number of spaces is {spaces}");
    }
    println!("Spaces: {spaces}!");

    data_types();

    numeric();

    bool();

    char();

    tuple_prac();
    
    array_prac();

    func_prac(67);

    state_exp();

    let six_seven = six_seven();

    println!("Six-Seven: {six_seven}");

    let even_or_odd: i32 = even_odd(67);

    println!("67 is (0 for even and 1 for odd) {even_or_odd}");
}

fn data_types() {
    let x = 2.01; // f64

    let y: f32 = 3.01; 

    println!("x = {x}, y = {y}");
}

fn numeric() {
    let sum = 5 + 10; 

    let difference: f32 = 95.5 - 4.3; 

    let product: u8 = 4 * 63; // should wrap around 

    let quotient = 56.7 / 32.2; 
    let truncated = -5 / 3; 

    let remainder = 43 % 5; 

    println!("Sum = {sum}, Difference = {difference}, Product = {product}, quotient = {quotient}, truncated = {truncated}, remainder = {remainder}");
}

fn bool() {
    let t = true; 

    let f: bool = false; 

    println!("t = {t}, f = {f}");
}

fn char() {
    let c = 'z'; 
    let z: char = 'Z'; // explicit type annotation 
    let money_mouth_face = '🤑';

    println!("c = {c}, z = {z}, money_mouth_face = {money_mouth_face}");
}