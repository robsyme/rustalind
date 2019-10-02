#[macro_use]
extern crate clap;

mod dna;
mod rna;
mod revc;
mod gc;
mod fib;
mod hamm;
mod iprb;
mod prot;

fn file_exists(path: String) -> Result<(), String> {
    std::fs::metadata(path)
        .map(|_metadata| ())
        .map_err(|e| e.to_string())
}

fn is_valid_integer(input: String) -> Result<(), String> {
    input.parse::<usize>()
        .map(|_i| ())
        .map_err(|e| e.to_string())
}

fn main() {
    let app = clap_app!(rustalind =>
        (version: "0.0.1")
        (author: "Rob Syme <rob.syme@gmail.com>")
        (about: "Learning Rust by implementing solutions to Rosalind problems.")
        (@subcommand dna =>
            (about: "Count nucleotide occurrence")
            (version: "0.0.1")
            (@arg INPUT: +required {file_exists} "Input file to read"))
        (@subcommand rna =>
            (about: "Translate RNA to DNA")
            (version: "0.0.1")
            (@arg INPUT: +required {file_exists} "Input file to read"))
        (@subcommand revc =>
            (about: "Count nucleotide occurrence")
            (version: "0.0.1")
            (@arg INPUT: +required {file_exists} "Input file to read"))
        (@subcommand fib =>
            (about: "Population size simulation")
            (version: "0.0.1")
            (@arg generations: -g --generations <int> +required {is_valid_integer} "Number of rabbit generations to simulate")
            (@arg fecundidty: -f --fecundity <int> +required {is_valid_integer} "Number of rabbit pairs are produced per rabbit pair per generation"))
        (@subcommand gc =>
            (about: "Identify highest GC content sequence from fasta file")
            (version: "0.0.1")
            (@arg INPUT: +required {file_exists} "Input file to read"))
        (@subcommand hamm =>
            (about: "Calculate hamming distance between sequences")
            (version: "0.0.1")
            (@arg INPUT: +required {file_exists} "Input file to read"))
        (@subcommand iprb =>
            (about: "Intro to Mendelian Inheritance")
            (version: "0.0.1")
            (@arg dominant: -k <int> +required {is_valid_integer} "Number of homozygous dominant individuals")
            (@arg heterozygous: -m <int> +required {is_valid_integer} "Number of heterozygous individuals")
            (@arg recessive: -n <int> +required {is_valid_integer} "Number of homozygous recessive individuals"))
        (@subcommand prot =>
            (about: "Translate RNA into protein")
            (version: "0.0.1")
            (@arg INPUT: +required {file_exists} "Input file to read"))
        );

    match app.get_matches().subcommand() {
        ("dna", Some(args)) => dna::run(args),
        ("rna", Some(args)) => rna::run(args),
        ("revc", Some(args)) => revc::run(args),
        ("fib", Some(args)) => fib::run(args),
        ("gc", Some(args)) => gc::run(args),
        ("hamm", Some(args)) => hamm::run(args),
        ("iprb", Some(args)) => iprb::run(args),
        ("prot", Some(args)) => prot::run(args),
        _ => {},
    }
}
