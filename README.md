# Rusatlind
Documenting the process of learning rust by implementing exercises from http://rosalind.info

## Compilation
```sh
$ cargo build --release
```

## Running
Each exercise is implemented as its own subcommand. 

```sh
$ target/release/rustalind dna --help
```

```
rustalind-dna 0.0.1
Count nucleotide occurence

USAGE:
    rustalind dna <INPUT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <INPUT>    Input file to read
```

I've also included the example data for each successfully completed exercise in the `test-data` folder. To run the first problem (DNA nucleotide counting):

```sh
$ target/release/rustalind dna test-data/dna/dna.txt
20 12 17 21
```

## Caveat emptor
This code should not be relied upon for anything other than to observe the funblings of an idiot learning something new. I don't claim that the code here is idiomatic or performant rust. Enjoy!
