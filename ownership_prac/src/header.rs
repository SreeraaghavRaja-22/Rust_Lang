pub fn variables_mut(
    let x = 5; 
    let y = x; // both 5 values are pushed onto the stack
)

pub fn string_own(
    let s1 = String::from("Hello");
    let s2 = s1; 

    println!("{s1}, world!");
)

