use std::io;
use std::io::prelude::*;

extern crate pegtokenizer;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                let tokens = pegtokenizer::tokenize(&line);
                println!("{:?}", tokens);
            }
            Err(error) => {
                println!("{}", error);
                break
            }
        }
    }
}
