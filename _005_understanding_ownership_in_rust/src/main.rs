fn main() {
    // --------Ownership Rules--------
    // 1. Each value in Rust has a variable that's called its owner
    // 2. There can be only one owner at any given time
    // 3. When the owner goes out of scope, the value will be dropped
    
    // when you use random curly braces, you create a new scope
    {
        // s is not valid here
        let s: &str = "hello";
        // s is now valid
        // do whatever with s
        let new_s:String = String::from("hello"); // this is stored on heap instead of str
    } 
    // the scope is now over and s is no longer valid
    
    {
        let a1: i32 = 7;
        let a2: i32 = a1;
        // here we simply make a copy of a variable on the stack -- because these variables would go on a stack
        
        let s1: String = String::from("yoyo");
        let s2: String = s1; 
        // in this example, you might think we create a copy of s1 just like above, but since String is a dynamic type, we actually don't and nor does the compiler add another pointer to the same memory reference [shallow copy]...instead the s1 gets deleted and the s2 is the only existing owner/variable ... so essentially we do a move instead of a copy
        
        // if you actually wanted to create a heap copy, you would do this...
        let s3: String = s2.clone();
        
        // now what if we call a function
        takes_ownership(s3);
        // now takes ownership is the function that contains the s3 variable and this scope ceases to have a reference to it
        // lets say you didn't want to switch the variable location or pointer on the stack...well then just pass a REFERANCE to the function instead
        takes_ownership_with_ref(&s2);
        // when you do this, the coined term is called BORROWING, we would say takes_ownership_with_ref takes a BORROWED String
    }
    
    {
        // finally lets say you have something you want to mutate with a function, but still don't want that function taking ownership of the variable
        let mut new_s1: String = String::from("lolol");
        
        fn takes_ownership_with_ref_and_muts(some_string: &mut String) {
            some_string.push_str(", it funny");
        }
        
        // now this function call with mutate our string
        takes_ownership_with_ref_and_muts(&mut new_s1);
        
        // note you can only have one mutable reference per scope, as this prevents data races where one pointer is trying to read from a spot in the heap that another pointer is writing to at the same time (which gives us corrupt data)
    }
    
    {
        // last thing to discuss is slices...if you want to return something like, part of a string, you need to use a slice
        let mut s: String = String::from("hello world");
        // this gives us data between index 0 (h) and index 4 (o)
        let hello: &str = &s[0..5];
        // this gives us data between index 6 (w) and index 11 (d i guess seems odd)
        let world: &str = &s[6..11];
        // if you range starts at the beginning or end you can omit that number i.e. we could have done [..5] and [6..]
        
        // so a string slice is going to use &str instead of String string slices and string literals are the same thing
    }
    
    {
        // you can do slices on other collections like arrays too:
        let a: [i32; 5] = [10, 20, 30, 40, 50];
        
        // for the slice we just prepend & to the type and & to the variable we are slicing
        let slice: &[i32] = &a[..2];
    }
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn takes_ownership_with_ref(some_string: &String) {
    println!("{}", some_string);
}
