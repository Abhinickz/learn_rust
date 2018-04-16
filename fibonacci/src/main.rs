use std::io;

// extern crate cached;
#[macro_use] extern crate cached;
#[macro_use] extern crate lazy_static;

use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

use std::time::Duration;
use std::thread::sleep;

use cached::{Cached, UnboundCache, SizedCache};

fn main() {
    let mut input_stdin = String::new();
    loop {
        println!("Please Enter n (Integer Only): ");
        let mut input_stdin = String::new();
        io::stdin()
            .read_line(&mut input_stdin)
            .expect("failed to read from stdin");

        let input_stdin = input_stdin.trim();
        match input_stdin.parse::<u64>() {
            Ok(integer) => {
                            println!("f(n) = {}", fib_with_memoize(integer));
                            break;
                        }
            Err(..) => {    
                        println!("You entered a Non-Number: {}", input_stdin);
                        continue;
                    }
        };
    }
}
cached!{
    FIB;
    fn fib_with_memoize(n: u64) -> u64 = {
        if n == 0 || n == 1 { return n; }
        fib_with_memoize(n-1) + fib_with_memoize(n-2)
    }
}