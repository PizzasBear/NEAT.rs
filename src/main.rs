#![allow(dead_code,unused_imports)]

mod neat;

use neat::{Pop, Conf};

struct NEATConf {
    gens: u32,
}

impl Conf for NEATConf {
    fn get_cull_survival_percentage(&self) -> f64 { 0.8 }
    fn get_link_addition_mutation_prob(&self) -> f64 { 0.4 }
    fn get_node_addition_mutation_prob(&self) -> f64 { 0.1 }
    fn get_link_disable_mutation_prob(&self) -> f64 { 0.4 }
}

fn square(a: f64) -> f64 { a * a }

fn main() {
    let conf = NEATConf {
        gens: 40,
    };

    let mut pop = Pop::new(150, 2, 1, &conf);
    for i in 0..conf.gens {
        let mut fitness_sum = 0.0;
        let mut best_fitness = 0.0;

        let mut links_count_sum = 0;
        let mut hidden_node_count_sum = 0;

        for net in &mut pop.nets {
            links_count_sum += net.get_enabled_links_count();
            hidden_node_count_sum += net.get_hidden_nodes_count();

            let net01 = net.eval(&[0.0, 1.0], &pop.innovs)[0];
            let net10 = net.eval(&[1.0, 0.0], &pop.innovs)[0];
            let net11 = net.eval(&[1.0, 1.0], &pop.innovs)[0];
            let net00 = net.eval(&[0.0, 0.0], &pop.innovs)[0];

            net.fitness = 4.0 - (square(1.0 - net01) + square(1.0 - net10) + square(net11) + square(net00));
            best_fitness = if best_fitness < net.fitness { net.fitness } else { best_fitness };
            fitness_sum += net.fitness;
        }

        //if i % 15 == 14 {
            println!("Gen {}", i);
            println!("\tspecies count: {}", pop.species.len());
            println!("\tavarage hidden nodes count: {}", (hidden_node_count_sum as f64) / (pop.size as f64));
            println!("\tavarage links count: {}", (links_count_sum as f64) / (pop.size as f64));
            println!("\tavarage fitness: {}", fitness_sum / (pop.size as f64));
            println!("\tbest fitness: {}", best_fitness);
        //}
        pop.next_gen(&conf);
    }
}

