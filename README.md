# Rusatlind
Documenting the process of learning rust by implementing exercises from http://rosalind.info

## Building
```sh
$ cargo build --release
```

## Testing
```
$ cargo test
   Compiling rustalind v0.1.0 (/home/robsyme/src/github.com/robsyme/rustalind)
    Finished dev [unoptimized + debuginfo] target(s) in 1.27s
     Running target/debug/deps/rustalind-e3431cf8a1d7236e

running 8 tests
test fib::tests::test_basic_recurrence ... ok
test hamm::tests::basic_distance ... ok
test revc::tests::test_reverse_complement ... ok
test dna::tests::count_basic_occurences ... ok
test rna::tests::test_translation ... ok
test utils::tests::blank_fasta ... ok
test utils::tests::erroneous_fasta ... ok
test utils::tests::test_gc_counting ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
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
