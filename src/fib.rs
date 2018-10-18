struct Rabbits {
    k: usize,
    next_pops: (usize, usize),
}

impl Rabbits {
    fn new(k: usize) -> Rabbits {
        Rabbits { k, next_pops: (0, 1)}
    }
}

impl Iterator for Rabbits {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let prev = self.next_pops.0;
        self.next_pops = (self.next_pops.1, self.next_pops.1 + self.next_pops.0 * self.k);
        Some(prev)
    }
}

fn calculate_rabbit_population(n: usize, k: usize) -> usize {
    Rabbits::new(k).nth(n).expect("Could not calculate population")
}

pub fn run(arguments: &clap::ArgMatches) {
    let n = arguments
        .value_of("generations")
        .expect("Could not find the 'g' argument")
        .parse::<usize>()
        .expect("Could not parse g argument to integer");

    let k = arguments
        .value_of("fecundidty")
        .expect("Could not find the 'f' argument")
        .parse::<usize>()
        .expect("Could not parse f argument to integer");

    println!("{}", calculate_rabbit_population(n, k));
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_recurrence() {
        let calculated = calculate_rabbit_population(5, 3);
        assert_eq!(calculated, 19)
    }
}