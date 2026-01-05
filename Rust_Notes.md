# Rust Commands

## Cargo 

- Manages dependencies for Rust 
- Commands:
    - cargo new "new directory"
        - makes a new directory with main.rs file in src directory
        - uninitialized git file too
    - cargo build 
        - compiles the rust file automatically
    - cargo run 
        - automatically compiles and runs the binary file 
    - cargo check 
        - checks code to make sure it compiles without making an executable file (binary file)


## Variables and Mutability

### Variables
-  Usually immutable 
    - Option to make variables mutable ("mut" keyword)
    - Common error "cannot assign twice to immutable variable {var_name}"

### Constants 
- values that are bound to a name and are not allowed to change
    - like immutable variables
- cannot use "mut" with constants 
- use "const" keyword 
- must annotate type 
- can be declared in any scope even global scope
- constants must be declared with all uppercase and underscores
- **valid** for the entire time a program runs within the scope in which they are declared


### Shadowing 

- when declaring a new variable with the same name as the old variable 
- compiler only sees the second variable
    - "first variable is shadowed by the second variables"
- use **let** command to shadow variables
    - different from **mut** because we can reassign the type using shadowing and **let**

## Data Types 

### Common Information about Data Types

- **Statically Typed**: must know all the types of the variables at compile time 
- **Different Types**
    - **Scalar Types**: a single value 
        - Integer: signed or unsigned
            - *isize* or *usize* with size the value based on the architecture 
            - Can use _ to separate numbers for readability 
                - 98_222
                - 0xffff_ffff
                - 0o77
                - 0b1111_0000
            - **panic**: program exits with an error 
                - can be caused by **INTEGER OVERFLOW**
                - will not happen when compiling in **release mode** 
                    - *2's complement overflow* happens instead
        - Floating-Point: default is f64 on modern CPUs
        - Numeric Operations: supports numeric operations like any other language
        - Boolean: same as any old lang 
            - 1 byte in size 
            - can be printed out with println!();
        - Character: single quote marks
    - **Compound Types**: group multiple values into one type
        - Tuples: group together a number of values with a variety of types into one compound type 
            - fixed length 
            - static sizing (cannot change)
            - each position of a tuple has a type
            - can use **pattern matching** to **destructure** a tuple 
            - can access a tuple element directly using (.) followed by index we want to access
            - **unit**: tuple without any values
        - Arrays: collection of multiple values all of the same type 
            - arrays must have a fixed length
            - values are **allocated on the stack**
            - Valid Element Access: let var = array_name[index]
            - Invalid Element Access: accessing an element that past the end of array
                - Rust will check to see if index being accessed is smaller than array length at **runtime**
                    - not common for most compilers

## Functions

### Functions Information 

- Conventional Naming Style: **snake_case**
- Parameters: part of a function signature 
- Arguments: concrete values put into function in the place of parameters

### Statements and Expressions

- Statements: instructions that perform some action and do not return a value
    - let x = 5; is a statement because there is no return value 
    - let x = let y = 6; is illegal because of this 
    - let x = (let y = 6); is also illegal since let y = 6; doesn't return a value
- Expressions: evaluate resultant value 
    - function declarations 
    - expressions can be part of statements

### Functions with Return Values

- Must declare return type with -> 
- can return early using **return** keyword
- **can use expression** since that returns too

## Control Flow 

### If-Statements

- Condition must be a bool or else error thrown 
- Multiple branches require else-if
    - if more than one else-if, use a **match**
- Can use an if statement in a let statement

### Repetition with Loops

- 3 types of loops: 
    - **loop**: code just executes forever until explicitly told to stop
    - **while**: doesn't require extra conditional logic inside loop
    - **for**: loop through stuff or a certain number of times
        - machine code is more efficient with for loops because index doesn't need to be compared to array length
        - use **x..y** for **x (inclusive)** and **y (exclusive)** range of numbers to loop through 
        - use **x..=y** for **x (inclusive)** and **y (inclusive)** range of numbers to loop through

- **Loop Labels**
    - can use loop labels to specify which loop a "break" or "continue" applies to 
    - **'loop_label** is the syntax
    - {break, continue} 'loop_label is the syntax


