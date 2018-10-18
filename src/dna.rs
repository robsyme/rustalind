use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn count_occurences(input: &str) -> HashMap<char, u32> {
    let mut frequency: HashMap<char, u32> = HashMap::new();
    for char in input.chars() {
        *frequency.entry(char).or_insert(0) += 1;
    }
    frequency
}

fn fmt_nucleotide_occurences(input: &str, bases: &[char]) -> String {
    let freq = count_occurences(input);
    bases.iter()
        .map(|base| freq.get(base).unwrap_or(&0u32))
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn run(arguments: &clap::ArgMatches) {
    let filename = arguments
        .value_of("INPUT")
        .expect("Could not find 'input' argument");

    let mut f = File::open(filename).expect("Could not read file");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).expect("Could not read into buffer");
    let output = fmt_nucleotide_occurences(&buffer, &['A', 'C', 'G', 'T']);

    println!("{}", output);
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn count_basic_occurences() {
        let input = String::from("AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC");

        let mut expected = HashMap::new();
        expected.insert('A' , 20);
        expected.insert('C', 12);
        expected.insert('G', 17);
        expected.insert('T', 21);

        assert_eq!(count_occurences(&input), expected);
    }
}