mod header;
use header::*;

fn main() {
    {                           // s is not valid here, since it's not declared
        let s = "hello";        // s is valid from this point forward
        println!("S is {s}");   // doing stuff with s
    }                           // this scope is over, the s is no longer valid

    let s = String::from("hello"); // can create a string from a string literal using from function 
    let mut t = String::from("hello"); 
    t.push_str(", world!");         // push_str() appends a literal to a String 
    println!("{t}");                // should print 'hello, world!'

    {
      let s = String::from("hello"); // s is valid from this point forward
      // do stuff with s here    
    }
        // this scope is now over, and s is no longer valid (Rust calls drop here)
    
    variables_mut();
    string_own();
    
   }


