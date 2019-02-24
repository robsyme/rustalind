extern crate rustalind;

use std::fs::File;
use std::io::prelude::*;
use std::convert::TryFrom;
use self::rustalind::seq::codon::Codon;
use self::rustalind::seq::translation::TranslatedCodon;
use self::rustalind::seq::translation::ncbi_translation_tables::STANDARD;

fn translate(input: &String) -> String {
    let collection: Vec<char> = input
        .chars()
        .map(|c| match c {
            'U'|'u'|'T'|'t' => 'T',
            'A'|'a' => 'A',
            'G'|'g' => 'G',
            'C'|'c' => 'C',
            _ => 'N',
        })
        .collect();
    collection[..]
        .chunks_exact(3)
        .map(|c| Codon::try_from(c))
        .filter_map(Result::ok)
        .map(|codon| codon.translate(&STANDARD))
        .filter_map(|translated| match translated {
            TranslatedCodon::Stop => None,
            _ => Some(char::from(translated)),
        })
        .collect::<String>()
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
    fn test_translate() {
        let test_input = String::from("AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA");
        let expected = String::from("MAMAPRTEINSTRING");
        assert_eq!(translate(&test_input), expected);
    }
}