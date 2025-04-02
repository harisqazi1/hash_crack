use std::fs::{File};
use std::io::{self, BufRead, Write};
use std::path::Path;
use md5;
use indicatif::ProgressBar;

pub fn cracking(hashfile: String, algorithm: String, wordlist: String) {
    // Output
    let mut output_final: Vec<String> = Vec::new();
    // Adding the "header" in the file
    output_final.push("Plaintext::Hash".to_string());
    // Check algorithm to see if code supports it
    let algorithm_string: String = algorithm.clone();
    // Check algorithm here
    if algorithm_string.trim() == "MD5" {
        // Variable to store count for lines
        let mut lines_in_wordlist: u64 = 0;
        // Count the amount of lines in the wordlist
        if let Ok(lines) = read_lines(wordlist.clone()) {
            // Consumes the iterator, returns an (Optional) String
            for _line in lines.map_while(Result::ok) {
                lines_in_wordlist = lines_in_wordlist + 1;
            }
        }
        // Progress bar
        let bar: ProgressBar = ProgressBar::new(lines_in_wordlist);
        // File hosts.txt must exist in the current path
        if let Ok(hash_lines) = read_lines(hashfile.clone()) {
            // Consumes the iterator, returns an (Optional) String
            for hashline in hash_lines.map_while(Result::ok) {
                println!("Attempting to crack: {}", hashline.clone());
                bar.inc(1/lines_in_wordlist);
                // Reading the wordlist file
                // this will run through all the words, while outer loop will do all hashes
                if let Ok(wordlines) = read_lines(wordlist.clone()) {
                    // Consumes the iterator, returns an (Optional) String
                    for wordline in wordlines.map_while(Result::ok) {
                        // runs the current hash line and wordlist line into the function
                        //println!("word: {}", wordline);
                        // after testing, add this vector to the output_final vector
                        //println!("FOUND {}", md_5(hashline.clone(), wordline.clone()));
                        if md_5(hashline.clone(), wordline.clone()) != "" {
                            println!("\n\nHash cracked: {}::{}\n", wordline.clone(), hashline.clone());
                            output_final.push(md_5(hashline.clone(), wordline.clone()))
                        }
                        //output_final.push(md_5(hashline.clone(), wordline.clone()));
                    }
                // finish the bar
                bar.finish();
                } else {
                    panic!("ERROR: WORDLIST NOT FOUND")
                }
            }
        } else {
            panic!("ERROR: HASH FILE NOT FOUND")
        }
    }
    // output to file
    let mut file: File = File::create("cracked.txt").expect("Cannot create file");
    //write to the file
    for strings in output_final {
        writeln!(file, "{strings}").expect("Failed to write to output file");
    }

}

// Output as String since, each value is getting checked here
pub fn md_5(hash: String, word: String) -> String {
    let mut output: String = String::new();
    // check if the hash is 32 characters
    if hash.len() == 32 {
        // create a new digest for each word
        let mut _digest: md5::Digest = md5::compute(word.clone());
        // Uncomment line below to see all the hashes it is currently running
        //println!("Now trying {:x}", _digest.clone());
        // Format hex to be into String format
        let string: String = format!("{:x}", _digest.clone());
        // If hash matches the newly created hash, send it out
        if string == hash {
            output = format!("{}::{}", word, hash);
        }
    } else {
        println!("Hash {} NOT 32 characters", hash.clone());
    }
    // Return the string
    output
}

// copied from https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}