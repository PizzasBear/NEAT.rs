use crate::neat::{Net, Species, Conf, Innov};

pub struct Pop {
    pub size: usize,
    pub nets: Vec<Net>,
    pub species: Vec<Species>,
}

impl Pop {
    pub fn new(size: usize, inputs_count: usize, outputs_count: usize) -> Self {
        let mut out = Self {
            size: size,
            nets: Vec::with_capacity(size),
            species: Vec::new(),
        };
        
        for i in 0..size {
            out.nets.push(Net::new(i, inputs_count, outputs_count));
        }

        out
    }

    pub fn next_gen(&mut self, innovs: &mut Vec<Innov>, conf: &dyn Conf) {
        let mut next_nets = Vec::<Net>::with_capacity(self.size);
        
        for species in &mut self.species {
            species.clear();
        }

        for net in &mut self.nets {
            net.in_species = false;
            for species in &mut self.species {
                species.add_member(net, conf);
                if net.in_species { break }
            }

            if !net.in_species {
                self.species.push(Species::new(net));
            }
        }

        let mut species_avarage_fitness_sum = 0.0;
        let mut best_species_index = 0;
        let mut best_species_avarage_fitness = 0.0;
        for (i, species) in self.species.iter_mut().enumerate() {
            species.fitness_sharing(&self.nets);
            species.choose_random_repr(&self.nets);
            species_avarage_fitness_sum += species.avarage_fitness;
            if best_species_avarage_fitness < species.avarage_fitness {
                best_species_index = i;
                best_species_avarage_fitness = species.avarage_fitness;
            }
        }

        let old_innovs_count = innovs.len();

        for species in &self.species {
            let baby_count = (species.avarage_fitness / species_avarage_fitness_sum * (self.size as f64)).floor() as usize;
            for _ in 0..baby_count {
                next_nets.push(species.make_child(innovs, old_innovs_count, &self.nets, conf));
            }
        }

        while next_nets.len() < self.size {
            next_nets.push(self.species[best_species_index].make_child(innovs, old_innovs_count, &self.nets, conf));
        }

        self.nets = next_nets;
    }
}
