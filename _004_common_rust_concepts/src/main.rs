// each part of this tutorial is split into regions that you can expand cand condense

fn main() {
    
    // #region variables
    
    // immutable variable -- you can't do x = 6 later 
    let x: i32 = 5;
    
    println!("{}", x);
    
    
    // mutable variable -- you can change it to another u32 (unsigned int 32 bit)
    let mut y:u32 = 6;
    y = 7;
    
    println!("{}", y);
    
    
    // const works like it does in JS, where once you set it it absolutely is static -- typical sytax is SCREAM_SNAKE_CASE
    // also you must annotate your type
    // also the value must be static
    const COOL_NUMBER: u32 = 10;
    
    println!("{}", COOL_NUMBER);
    
    // rust has another cool feature called shadowing, where you can redeclare an immutable var
    // you may be wondering why not just do it the normal way with mut keyword
    // with this way we can keep the var immutable AND change types if needed
    let hi: &str = "yo";
    let hi: u32 = 10;
    
    println!("{}", hi);
    
    // #endregion variables
    
    // #region types
    
    
    
    // scaler data types represent a single value, compound data types represent more than one value
    
    // scalar data types:
    // Ints i.e. u8 or i8, default 32 bit, can be represented as Decimal (98_222), Hex (0xff), Binary (0b1111_0000), ETC -- wraps around if overflow
    // Floats i.e. f32 or f64 -- something like 3.2
    // Bools, i.e. bool true or false only
    // Chars -- written with 'single quotes'
    
    // compound data types:
    
        // Tups or Tuples
        
        // don't think you have to specify the actual data types
        let tup: (&str, i32) = ("hi", 3);
        
        // to get it back out:
        let (channel, sub_count ) = tup;
        println!("{}{}", channel, sub_count);
        // this gets the second value in the tuple
        let other_example = tup.1;
        
        // you can also do arrays the same way but with [] instead of (); arrays have to be of one type and have more options for iterating through them
        // arrays in rust are fixed length -- to get random length you need a vector
        let error_codes: [i32; 3] = [200, 404, 500]; // here we specify the type of data and the length of the array
        
        // to get it out:
        let not_found: i32 = error_codes[1];
        // returns 404
    
    // #endregion types
    
    // #region functions
    
    // functions look like this:
    fn my_function(x: i32, y: i32) -> i32 {
        let sum: i32 = x + y;
        return sum;
    }
    // use snake_case for function names in rust
    // make sure to specify types for params
    // the return type must also be specified after ->
    // lastly fun fact, you can choose to not write return as rust will automatically return the last line of a function if it is returnable
    
    // #endregion functions

    // #region statments & loops
    
    // here is an if/else:
    let number:i32 = 5;
    
    if number < 10 {
        println!("{}", "hi");
    } else if number > 20 {
        println!("{}", "bye");
    } else {
        println!("{}", "number was in between 10 and 20");
    }
    // expressions for if/else MUST be explicitly boolean in rust (i.e. you cant just put falsey/truthy statments like (if number {}))
    
    // inline or ternary if/else:
    let isTrue:bool = if 1 == 1 {true} else {false};
    
    
    
    // here is a general loop
    // to get out of a loop, use break keyword
    let mut counter: i32 = 0;
    
    loop {
        println!("{}", "im loopin");
        
        if counter == 5 {
            break;
        }
        
        counter +=1;
        
    }
    
    
    // here is a while loop
    let mut count_down:i32 = 3;
    
    while count_down != 0 {
        println!("{}!", count_down);
        
        count_down -= 1;
        
        if count_down == 0 {
            println!("liftoff!!!");
            break;
        }
    }
    
    
    
    // here is a for loop
    let an_array: [i32; 5] = [10, 20, 30, 40, 50];
    
    for element in an_array.iter() {
        println!("the value is: {}", element);
    }
    
    for each_number in (1..4) {
        println!("the number is {}", each_number)
    }
    
    // #endregion statments
}
