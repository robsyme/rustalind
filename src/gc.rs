extern crate rustalind;

use std::fs::File;
use std::cmp::Ordering::Less;
use self::rustalind::io;

pub fn run(arguments: &clap::ArgMatches) {
    let filename = arguments
        .value_of("INPUT")
        .expect("Could not find the 'input' argument");

    let f = File::open(filename).expect("Could not read file");

    if let Some((name, gc_percent)) = io::FastaReader::new(f)
        .filter_map(Result::ok)
        .map(|record| (record.id().clone(), record.gc_percent()))
        .max_by(|x, y| x.1.partial_cmp(&y.1).unwrap_or(Less)) {
        println!("{}\n{}", name, gc_percent * 100.0);
    }
}
