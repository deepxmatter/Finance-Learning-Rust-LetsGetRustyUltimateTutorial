notice we have a Cargo.toml...this is where all the project data goes and things like dependencies (cargo.toml == package.json)

typically cargo will create a git repo and such...but I think because i have one outside of the project dir, it didn't

btw make sure you cd into each project

to build this instead of running "rustc main.rs"...we are going to run "cargo build" which will automatically use "main.rs"

now we have a cargo.lock (cargo.lock == package.lock) and a target dir that contains our exe and debug stuff

now to run our program we just type "cargo run"... which is like compiling and running the exe

another useful tool is "cargo check" which checks the program for errors pre-compile (faster than running and compiling)

kk, now were making a guessing game in next section...