use crate::neat::{Net, Pop, Conf, Innov};

pub struct Species {
    pub members: Vec<usize>,
    pub members_shared_fitness: Vec<f64>,
    pub repr: Net,
    pub staleness: u32,
    pub best_fitness: f64,
    pub avarage_fitness: f64,
}

impl Species {
    pub fn new(net: &Net) -> Self {
        Self {
            members: Vec::new(),
            members_shared_fitness: Vec::new(),
            repr: net.clone(),
            staleness: 0,
            best_fitness: 0.0,
            avarage_fitness: 0.0,
        }
    }

    pub fn choose_random_repr(&mut self, nets: &Vec<Net>) {
        use rand::{prelude::*, thread_rng};
        use rand_distr::Uniform;

        let repr_pop_index = self.members[Uniform::new(0, self.members.len()).sample(&mut thread_rng())];
        self.repr = nets[repr_pop_index].clone();
    }

    pub fn add_member(&mut self, net: &mut Net, conf: &dyn Conf) {
        if !net.in_species {
            let mut disjoint = 0i32;
            let mut matching = 0i32;
            let mut weight_diff_sum = 0f64;
            
            let (mut i, mut j) = (0usize, 0usize);
            while i < net.links.len() && j < self.repr.links.len() {
                if net.links[i].innov < self.repr.links[j].innov {
                    disjoint += 1;
                    i += 1;
                }
                else if self.repr.links[j].innov < net.links[i].innov {
                    disjoint += 1;
                    j += 1;
                }
                else {
                    matching += 1;
                    weight_diff_sum += (net.links[i].weight - self.repr.links[j].weight).abs();
                    i += 1; j += 1;
                }
            }

            let excess = i + j - net.links.len() - self.repr.links.len();

            let size_norm = conf.size_norm(net.links.len(), self.repr.links.len());
            let compat = (conf.get_excess_coef() * (excess as f64) + conf.get_disjoint_coef() * (disjoint as f64)) / size_norm +
                conf.get_weight_diff_coef() * weight_diff_sum / (matching as f64);
            if compat < conf.get_compat_threshold() {
                self.members.push(net.index);
                net.in_species = true;
            }
        }
    }

    pub fn fitness_sharing(&mut self, nets: &Vec<Net>) {
        self.staleness += 1;
        self.members_shared_fitness.resize(self.members.len(), 0.0);
        
        for i in 0..self.members.len() {
            let net = nets[self.members[i]];

            if self.best_fitness < net.fitness {
                self.staleness = 0;
                self.best_fitness = net.fitness;
            }
            self.members_shared_fitness[i] = net.fitness / (self.members.len() as f64);
            self.avarage_fitness += self.members_shared_fitness[i];
        }
    }

    pub fn choose_parent(&self, nets: &Vec<Net>) -> usize {
        use rand::{prelude::*, thread_rng};
        use rand_distr::Uniform;

        let to_parent_shared_fitness_sum = Uniform::new(0.0, self.avarage_fitness).sample(&mut thread_rng());
        let mut shared_fitness_sum = 0.0;

        for i in 0..self.members_shared_fitness.len() {
            shared_fitness_sum += self.members_shared_fitness[i];
            if to_parent_shared_fitness_sum < shared_fitness_sum {
                return i;
            }
        }
        panic!("Parent choosing error.");
    }

    pub fn make_child(&self, innovs: &mut Vec<Innov>, old_innovs_count: usize, nets: &Vec<Net>, conf: &dyn Conf) -> Net {
        use rand::{prelude::*, thread_rng};
        use rand_distr::Uniform;

        let mut out: Net;
        if Uniform::from(0.0..1.0).sample(&mut thread_rng()) < conf.get_crossover_prob() {

        }

        out.mutate(innovs, old_innovs_count, conf);
        out
    }

    pub fn clear(&mut self) {
        self.members.clear();
    }
}
