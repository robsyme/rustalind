fn dominant_pheynotype_probability(k: usize, m: usize, n: usize) -> f64 {
    let population_total = k + m + n;
    let possibility_total = 4 * population_total * (population_total - 1);

    let dom_vs_dom = k * (k - 1);
    let dom_vs_het = 2 * k * m;
    let dom_vs_rec = 2 * k * n;
    let het_vs_het = m * (m - 1);
    let het_vs_rec = 2 * m * n;
    let _rec_vs_rec = n * (n - 1);

    let dom_pheno_total = 4 * dom_vs_dom + 4 * dom_vs_het + 4 * dom_vs_rec + 3 * het_vs_het + 2 * het_vs_rec;
    dom_pheno_total as f64 / possibility_total as f64
}

pub fn run(arguments: &clap::ArgMatches) {
    let k = arguments
        .value_of("dominant")
        .expect("Could not find the 'k' argument")
        .parse::<usize>()
        .expect("Could not parse k argument to integer");
    let m = arguments
        .value_of("heterozygous")
        .expect("Could not find the 'm' argument")
        .parse::<usize>()
        .expect("Could not parse m argument to integer");
    let n = arguments
        .value_of("recessive")
        .expect("Could not find the 'n' argument")
        .parse::<usize>()
        .expect("Could not parse n argument to integer");

    let result = dominant_pheynotype_probability(k, m, n);

    println!("{}", result);
}