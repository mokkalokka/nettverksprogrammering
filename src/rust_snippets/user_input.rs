use read_input::prelude::*;
use rand;

pub fn run() {
    let input = input::<u32>()
        .msg("What is your favorite number?")
        .err("That does not look like a positive number. Please try again")
        .get();

    let rand = rand::random::<u32>();

    print!("Your favorite number is: {}, \
    \nMine is {}, and it is greater than yours: {}", input, rand, rand > input)
}


