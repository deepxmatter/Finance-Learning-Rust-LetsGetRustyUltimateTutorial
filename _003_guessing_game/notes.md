to get a dependency that allows random nums, we need to go back to Cargo.toml and add it -- you'll see we added 'rand = "0.5.5"', which is a package for random numbers

once you add this in here, you need to run cargo build

now in our imports you can see "rand::Rng" which is the Rng module from the rand library

now we bring in std::cmp::Ordering, which is like dumb af enum compare statments...why we don't just use if/else is beyond me

colored is a DOPE library for coloring outputs on command line as, you can see we used it here