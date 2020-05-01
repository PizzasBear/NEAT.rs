use crate::neat::{Net, Species, Conf};

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

    pub fn next_gen(&mut self, conf: &dyn Conf) {
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
                self.species.push(Species::new(net))
            }
        }

        for species in &mut self.species {
            species.choose_random_repr(&self.nets);
            species.fitness_sharing(&self.nets);
        }
    }
}
