use core::panic;
// Reading argument values from env
use std::{env};
use hash_crack::cracking;

fn main() {
    // Collect arguments
    let args: Vec<String> = env::args().collect();
    // hashfile
    let _hash: String = args[1].clone();
    // hashing algorithm
    let _algorithm: String = args[2].clone();
    // wordlist 
    let _wordlist: String = args[3].clone();
    // Format: binary hash hashing_algorithm wordlist
    if args.len() == 4 {
        cracking(_hash, _algorithm, _wordlist);
    } else {
        panic!("\nERROR: The amount of arguments is incorrect.\n");
    }
}
