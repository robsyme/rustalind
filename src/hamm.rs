use std::io::prelude::*;
use std::fs;
use std::io;

#[derive(Debug)]
struct Pairer<T: Iterator> {
    it: T,
    prev: Option<T::Item>,
}

impl<T: Iterator> Pairer<T> {
    pub fn new(it: T) -> Self {
        Pairer {it, prev: None }
    }
}

impl<T: Iterator> Iterator for Pairer<T> {
    type Item = (T::Item, T::Item);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let first = self.it.next();
        let second = self.it.next();

        match (first, second) {
            (None, _) => None,
            (Some(_), None) => None,
            (Some(a), Some(b)) => Some((a, b)),
        }
    }
}

pub fn hamming_distance(str1: &str, str2: &str) -> usize {
    str1.chars()
        .zip(str2.chars())
        .filter(|a| a.0 != a.1)
        .count()
}

pub fn run(arguments: &clap::ArgMatches) {
    let filename = arguments
        .value_of("INPUT")
        .expect("Couldn't find the 'INPUT' argument");

    let pairs = fs::File::open(filename)
        .map(io::BufReader::new)
        .expect("Couldn't open the hamming distance input file for reading")
        .lines()
        .filter_map(Result::ok);

    for a in Pairer::new(pairs) {
        let (first, second) = a;
        println!("{}", hamming_distance(&first, &second));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_distance() {
        let str1 = "GAGCCTACTAACGGGAT";
        let str2 = "CATCGTAATGACGGCCT";
        assert_eq!(7, hamming_distance(str1, str2));
    }
}