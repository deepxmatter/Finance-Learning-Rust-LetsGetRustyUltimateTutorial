
// this is how you do a function -- fn
fn main() {
  // the ! after println is a macro (will cover later)
  println!("lets get rusty!");
}

// to compile this, go to terminal and type "rustc main.rs" -- you will see a main.exe and a main.pdb

// now if you type ./main.exe, you will get "lets get rusty"

// the only problem with this is it's not really a good framework for real project...lets use cargo in the next section (check if you have by typing cargo --version)