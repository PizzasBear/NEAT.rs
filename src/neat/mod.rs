mod conf;
mod neuralnet;
mod population;
mod species;

pub use conf::Conf;
pub use neuralnet::{Innov, Net};
pub use population::Pop;
pub use species::Species;

struct cfg;

impl Conf for cfg {
    fn get_cull_survival_percentage(&self) -> f64 {
        0.8
    }
    fn get_link_addition_mutation_prob(&self) -> f64 {
        0.4
    }
    fn get_node_addition_mutation_prob(&self) -> f64 {
        0.1
    }
    fn get_link_disable_mutation_prob(&self) -> f64 {
        0.4
    }
}

pub fn tst() {
    let mut innovs = Vec::<Innov>::new();
    let conf = cfg {};
    let mut net = Net::new(2, 1, &mut innovs, 0, &conf);
    net.add_link(&mut innovs, 0, 1.0, 0, 1);
}
