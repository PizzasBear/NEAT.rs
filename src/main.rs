#![allow(dead_code,unused_imports)]

mod neat;

use neat::{Pop, Conf};

struct NEATConf {
    gens: u32,
}

impl Conf for NEATConf { }

fn main() {
    let conf = NEATConf {
        gens: 150,
    };

    let mut pop = Pop::new(150, 2, 1, &conf);
    for i in 0..conf.gens {
        let mut avarage_fitness = 0.0;
        let mut best_fitness = 0.0;
        for net in &mut pop.nets {
            net.fitness = 2.0 +
                net.eval(&[0.0, 1.0], &pop.innovs)[0] +
                net.eval(&[1.0, 0.0], &pop.innovs)[0] -
                net.eval(&[1.0, 1.0], &pop.innovs)[0] -
                net.eval(&[0.0, 0.0], &pop.innovs)[0];
            best_fitness = if best_fitness < net.fitness { net.fitness } else { best_fitness };
            avarage_fitness += net.fitness;
        }

        println!("Gen {}", i);
        println!("\tavarage fitness {}", avarage_fitness / (pop.size as f64));
        println!("\tbest fitness {}", best_fitness);

        pop.next_gen(&conf);
    }
}

