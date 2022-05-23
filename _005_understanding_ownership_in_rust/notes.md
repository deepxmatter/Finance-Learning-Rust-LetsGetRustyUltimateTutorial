rust is kind of an in between of c (manual memory management [aka entirely hands on]) and something like javascript (garbage collector for memory management [aka entirely hands off])

rust has whats called an ownership model:
  - like manual memory management, you have:
    - control over memory
    - faster runtime
    - smaller program size
    
  - like a garbage collector, you have:
    - no errors
    
    
    unfortunately, you won't get the same code write speed as you would with a language that uses garbage collection because you will be fighting with the "borrow checker", which is a safety checker proprietary to rust
    
    the borrow checker allows you to have control over your memory, but double checks everything to make sure you aren't creating memory leaks/bugs
    
    the tradeoff here is that unlike something like c, you won't accidentally create program-breaking bugs, memory leaks or security vulnerabilities, but on the flip-side, you will fight with this borrow checker to make sure you aren't introducing bugs which makes the write time slower
    
    ultimately, many consider this trade off worthwhile as you might be able to write c/c++ a little faster...but if you make a mistake it may take weeks to find it where in rust you spend 15 minutes fixing it on the spot and never encounter an error...feeling safe your code will never break or be exploited
    
    
    
    
    ----------------------
    
    with rust, you have a stack and heap, and inside the stack you have stack frames that are calculated at runtime by each function (aka each function is a frame and the variables for that function [scope] reside in that frame )
    
    the stack cannot change size and cannot be dynamic -- the variables ALSO must be of known size because of this restraint
    
    the heap is able to grow or shrink at runtime and the data can be dynamic in size and very large
    
    things like string literals need to go in the heap
    
    in this tutorial, the instructor uses binary and heap interchangebly but im not sure why
    
    
    also make sure to check the main.rs
    