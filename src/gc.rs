use std::fs::File;
use utils::FastaReader;
use std::cmp::Ordering::Less;

pub fn run(arguments: &clap::ArgMatches) {
    let filename = arguments
        .value_of("INPUT")
        .expect("Could not find the 'input' argument");

    let f = File::open(filename).expect("Could not read file");

    if let Some((name, gc_percent)) = FastaReader::new(f)
        .filter_map(Result::ok)
        .map(|record| (record.id().clone(), record.gc_percent()))
        .max_by(|x, y| x.1.partial_cmp(&y.1).unwrap_or(Less)) {
        println!("{}\n{}", name, gc_percent * 100.0);
    }
}
