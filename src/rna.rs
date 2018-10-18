use std::fs::File;
use std::io::prelude::*;

fn translate(input: &str) -> String {
    input
        .chars()
        .map(|base| match base {
            'T' => 'U',
            't' => 'u',
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
    let output = translate(&buffer);

    println!("{}", output);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation() {
        let input = "GATGGAACTTGACTACGTAAATT";
        assert_eq!(translate(input), String::from("GAUGGAACUUGACUACGUAAAUU"));
    }
}