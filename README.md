# hash_crack

A really simple hash cracking program written in Rust. It currently only supports MD5. It also only supports a hash file instead of a single hash. If you would like to extend this program to other hashing algorithms as well, you would have to move some logic from `lib.rs:cracking()` to the `md_5()` function in order to purely segment all the logic for each algorithm into its own function. 

I made this to get more comfortable with Rust while writing my own hash cracker from scratch (or coding as much of it as I could). The speed is actually faster than hashcat on my system (see Speed section below). 

## Installation

Download the code and set up with Cargo:

```bash
git clone https://codeberg.org/Harisfromcyber/hash_crack.git
cd hash_crack
cargo build --release
./target/release/hash_crack hash MD5 words
```

In the previous command, `hash` is your hash file and `words` is your wordlist.

## Speed

For 6 hashes with a wordlist of 104334 (/usr/share/dict/words): 

hash_crack (this program):

```bash
Command: time ../target/release/hash_crack hash MD5 words
________________________________________________________
Executed in  376.15 millis    fish           external
   usr time  373.57 millis    0.00 micros  373.57 millis
   sys time    2.48 millis  475.00 micros    2.00 millis
________________________________________________________
Executed in  374.04 millis    fish           external
   usr time  370.52 millis    0.00 micros  370.52 millis
   sys time    3.53 millis  530.00 micros    3.00 millis
________________________________________________________
Executed in  373.02 millis    fish           external
   usr time  371.83 millis  279.00 micros  371.55 millis
   sys time    1.21 millis  208.00 micros    1.00 millis
```

Hashcat:

```bash
Command: time hashcat -a 0 -m 0 hash words
________________________________________________________
Executed in    2.01 secs      fish           external
   usr time  267.14 millis  273.00 micros  266.87 millis
   sys time  974.06 millis  192.00 micros  973.87 millis
________________________________________________________
Executed in    2.02 secs      fish           external
   usr time  270.97 millis    0.00 micros  270.97 millis
   sys time  980.41 millis  517.00 micros  979.89 millis
________________________________________________________
Executed in    2.02 secs      fish           external
   usr time  273.00 millis  271.00 micros  272.73 millis
   sys time  972.67 millis  195.00 micros  972.48 millis
```

With the averages between the execution times for each program, my code was **81.4345% faster** than hashcat for the MD5 algorithm.