use std::fs::File;
use std::io::prelude::*;

fn reverse_complement(input: &str) -> String {
    input
        .chars()
        .rev()
        .map(|base| match base {
            'A' => 'T',
            'C' => 'G',
            'G' => 'C',
            'T' => 'A',
            _ => base
        })
        .collect()
}

pub fn run(arguments: &clap::ArgMatches) {
    let filename = arguments
        .value_of("INPUT")
        .expect("Could not find the 'input' argument");

    let mut f = File::open(filename).expect("Could not read file");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).expect("Could not read into buffer");
    let output = reverse_complement(&buffer);

    println!("{}", output);
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_reverse_complement() {
        let input = String::from("AAAACCCGGT");
        assert_eq!(reverse_complement(&input), "ACCGGGTTTT");
    }
}