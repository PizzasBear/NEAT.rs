use crate::neat::{Net, Species, Conf, Innov};

pub struct Pop {
    pub size: usize,
    pub nets: Vec<Net>,
    pub species: Vec<Species>,
    pub innovs: Vec<Innov>,
}

impl Pop {
    pub fn new(size: usize, inputs_count: usize, outputs_count: usize, conf: &dyn Conf) -> Self {
        let mut out = Self {
            size: size,
            nets: Vec::with_capacity(size),
            species: Vec::new(),
            innovs: Vec::new(),
        };
        
        for _ in 0..size {
            out.nets.push(Net::new(inputs_count, outputs_count, &mut out.innovs, 0, conf));
        }

        out
    }

    pub fn next_gen(&mut self, conf: &dyn Conf) {
        let mut next_nets = Vec::<Net>::with_capacity(self.size);
        
        for species in &mut self.species {
            species.clear();
        }

        for (i, net) in self.nets.iter_mut().enumerate() {
            net.in_species = false;
            for species in &mut self.species {
                species.add_member(net, i, conf);
                if net.in_species { break }
            }

            if !net.in_species {
                self.species.push(Species::new(net));
            }
        }

        let mut species_avarage_fitness_sum = 0.0;
        let mut best_species_index = 0;
        let mut best_species_avarage_fitness = 0.0;
        let mut bad_species = Vec::<usize>::new();


        for (i, species) in self.species.iter_mut().enumerate() {
            if species.members.len() == 0 ||
                conf.get_staleness_threshold() <= species.staleness {
                
                bad_species.push(i);
                continue;
            }
            species.cull(&self.nets, conf);
            species.fitness_sharing(&self.nets);
            species.choose_random_repr(&self.nets);
            species_avarage_fitness_sum += species.avarage_fitness;
            if best_species_avarage_fitness < species.avarage_fitness {
                best_species_index = i;
                best_species_avarage_fitness = species.avarage_fitness;
            }
        }

        bad_species.reverse();
        for i in bad_species {
            self.species.remove(i);
            if i < best_species_index {
                best_species_index -= 1;
            }
        }

        let old_innovs_count = self.innovs.len();

        for species in &self.species {
            let baby_count = (species.avarage_fitness / species_avarage_fitness_sum * (self.size as f64)).floor() as usize;
            for _ in 0..baby_count {
                next_nets.push(species.make_child(&mut self.innovs, old_innovs_count, &self.nets, conf));
            }
        }

        while next_nets.len() < self.size {
            next_nets.push(self.species[best_species_index].make_child(&mut self.innovs, old_innovs_count, &self.nets, conf));
        }

        self.nets = next_nets;
    }
}
