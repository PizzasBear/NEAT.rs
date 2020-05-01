use crate::neat::Conf;

/// Smart neural network brain.
pub struct Net {
    pub inputs_count: usize,
    pub outputs_count: usize,
    pub(super) nodes: Vec<Node>,
    pub(super) links: Vec<Link>,
    pub in_species: bool,
    pub fitness: f64,
    pub index: usize,
}

use rand::{thread_rng, prelude::*};
use rand_distr::{Normal, Uniform};

impl Net {
    /// Creates a new neural network.
    pub fn new(index: usize, inputs_count: usize, outputs_count: usize) -> Self {
        let mut out = Self {
            nodes: Vec::with_capacity(inputs_count + outputs_count + 1),
            links: Vec::new(),
            inputs_count: inputs_count,
            outputs_count: outputs_count,
            in_species: false,
            fitness: 0.0,
            index: index,
        };

        for i in 0..(inputs_count + 1) {
            out.nodes[i] = Node {
                index: i,
                in_link_indices: Vec::new(),
            }
        }

        for i in (inputs_count + 1)..(inputs_count + outputs_count + 1) {
            out.nodes[i] = Node {
                index: i,
                in_link_indices: Vec::new(),
            }
        }

        out
    }

    pub fn crossover(&self, net2: &Self, conf: &dyn Conf) -> Self {
        let mut out = self.clone();
        let mut j = 0;
        for (i, link) in self.links.iter().enumerate() {
            while net2.links[j].innov < link.innov { j += 1; }
            let link2 = &net2.links[j];
            let out_link = &mut out.links[i];

            if link.innov == link2.innov {
                if Uniform::from(0.0..1.0).sample(&mut thread_rng()) < 0.5 {
                    out_link.weight = link2.weight;
                }

                if link.enabled != link2.enabled {
                    if Uniform::from(0.0..1.0).sample(&mut thread_rng()) < conf.link_enabling_in_child_prob() {
                        out_link.enabled = true;
                    }
                    else {
                        out_link.enabled = false;
                    }
                }
            }
        }

        out
    }

    /// adds a link betwean the two specified nodes with the specified weight.
    fn add_link(&mut self, innovs: &mut Vec<Innov>, old_innovs_count: usize, weight: f64, from: usize, to: usize) {
        let mut innov = innovs.len();

        for i in old_innovs_count..innov {
            if innovs[i].from == from && innovs[i].to == to {
                innov = i;
                break;
            }
        }

        if innov == innovs.len() {
            innovs.push(Innov {
                from: from,
                to: to,
                number: innov,
            });
        }

        let link = Link {
            innov: innov,
            weight: weight,
            enabled: true,
        };
        self.nodes[to].in_link_indices.push(self.links.len());
        self.links.push(link);
    }

    /// Mutates by adding a link.
    pub fn mutate_link(&mut self, innovs: &mut Vec<Innov>, old_innovs_count: usize, conf: &dyn Conf) {
        let mut rng = thread_rng();
        
        let mut from = Uniform::new(0, self.nodes.len() - self.outputs_count).sample(&mut rng);
        let mut to: usize;
        
        if self.inputs_count <= from {
            from += self.outputs_count;
            to = Uniform::new(self.inputs_count, self.nodes.len()).sample(&mut rng);
        }
        else {
            to = Uniform::new(self.inputs_count, self.nodes.len() - 1).sample(&mut rng);
            if from <= to { to += 1; }
        }
        
        if self.creates_cycles(from, to, innovs) {
            let tmp = to;
            to = from;
            from = tmp;
        }

        self.add_link(innovs, old_innovs_count, Uniform::from(-1.0..1.0).sample(&mut thread_rng()) * conf.get_weight_init_range(), from, to);
    }

    /// Mutates by adding a node.
    pub fn mutate_node(&mut self, innovs: &mut Vec<Innov>, old_innovs_count: usize) {
        let mut rng = thread_rng();
        let uniform = Uniform::new(0, self.links.len());

        let (from, to): (usize, usize);
        let weight: f64;

        {
            let link = &mut self.links[uniform.sample(&mut rng)];
            from = innovs[link.innov].from;
            to = innovs[link.innov].to;
            weight = link.weight;
        }
        
        let new_index = self.nodes.len();
        self.nodes.push(Node {
            in_link_indices: Vec::new(),
            index: new_index,
        });
        self.add_link(innovs, old_innovs_count, 1.0, from, new_index);
        self.add_link(innovs, old_innovs_count, weight, new_index, to);
    }

    /// Mutates the weights.
    pub fn mutate_weights(&mut self, conf: &dyn Conf) {
        for link in &mut self.links {
            link.mutate_weight(conf);
        }
    }

    pub fn mutate(&mut self, innovs: &mut Vec<Innov>, old_innovs_count: usize, conf: &dyn Conf) {
        if Uniform::from(0.0..1.0).sample(&mut thread_rng()) <
            conf.get_link_addition_mutation_prob() {
            self.mutate_link(innovs, old_innovs_count, conf);
        }

        if Uniform::from(0.0..1.0).sample(&mut thread_rng()) <
            conf.get_node_addition_mutation_prob() {
            self.mutate_node(innovs, old_innovs_count);
        }

        if Uniform::from(0.0..1.0).sample(&mut thread_rng()) <
            conf.get_weight_mutation_prob() {
            self.mutate_weights(conf);
        }
    } 

    /// Evaluates the network.
    pub fn eval(&self, inputs: &[f64], innovs: &Vec<Innov>) -> Vec<f64> {
        let mut out = Vec::<f64>::with_capacity(self.outputs_count);
        let mut evaled_nodes = Vec::<(f64, bool)>::with_capacity(self.nodes.len());
        for i in 0..self.outputs_count {
            out[i] = self.nodes[i + self.inputs_count + 1].eval(self, &mut evaled_nodes, inputs, innovs);
        }

        out
    }

    /// Checks if adding a link from `from` to `to` will creates cycles, and therefore makes the network unevaluable.
    pub fn creates_cycles(&self, from: usize, to: usize, innovs: &Vec<Innov>) -> bool {
        let mut visited_nodes = vec![to];
        loop {
            let mut newly_visited_nodes_count = 0;
            for link in &self.links {
                let link_from = innovs[link.innov].from;
                let link_to = innovs[link.innov].to;

                if link.enabled && visited_nodes.contains(&link_from) && !visited_nodes.contains(&link_to) {
                    if link_to == from {
                        return true;
                    }

                    visited_nodes.push(to);
                    newly_visited_nodes_count += 1;
                }
            }

            if newly_visited_nodes_count == 0 {
                return false;
            }
        }
    }
}

impl Clone for Net {
    /// Clones the net.
    fn clone(&self) -> Self {
        Self {
            in_species: self.in_species,
            inputs_count: self.inputs_count,
            outputs_count: self.outputs_count,
            links: self.links.clone(),
            nodes: self.nodes.clone(),
            fitness: self.fitness,
            index: self.index,
        }
    }
}

/// Connects two nodes with weight.
pub(super) struct Link {
    /// The innovation number. The matching innovation contains information about the two connected nodes.
    pub innov: usize,
    /// The weight of the link.
    pub weight: f64,
    /// If this is enabled
    pub enabled: bool,
}

impl Link {
    /// Mutates the weight.
    pub fn mutate_weight(&mut self, conf: &dyn Conf) {
        if Uniform::from(0.0..1.0).sample(&mut thread_rng()) < conf.get_complete_weight_override_prob() {
            self.weight = Uniform::new(-conf.get_weight_init_range(), conf.get_weight_init_range()).sample(&mut thread_rng());
        }
        else if conf.is_gaussian_weight_mutation() {
            self.weight += Normal::new(0.0, conf.get_gaussian_stddev()).unwrap().sample(&mut thread_rng());
        }
        else {
            let mutation_range = conf.get_uniform_weight_mutation_range();
            self.weight += Uniform::new(-mutation_range, mutation_range).sample(&mut thread_rng())
        }
    }
}

impl Clone for Link {
    fn clone(&self) -> Self {
        Self {
            innov: self.innov,
            weight: self.weight,
            enabled: self.enabled,
        }
    }
}

/// A node/neuron in a neural network.
pub(super) struct Node {
    /// The links that connect into the node.
    pub in_link_indices: Vec<usize>,
    /// The index of the node.
    pub index: usize,
}

impl Node {
    /// Finds a link by its innovations number. The method return the links index in `self.in_links`.
    /// 
    /// # Panics
    /// 
    /// Panics if the a link with the specified innovation number wasn't found in `self.in_links`.
    pub fn find_link(&mut self, innov: usize, net: &Net) -> usize {
        let mut right_boundary = self.in_link_indices.len() - 1;
        let mut left_boundary = 0;
        let mut middle;
        while left_boundary <= right_boundary {
            middle = (left_boundary + right_boundary) / 2;

            let middle_innov = net.links[self.in_link_indices[middle]].innov;

            if middle_innov < innov {
                left_boundary = middle + 1
            }
            else if innov < middle_innov {
                right_boundary = middle - 1;
            }
            else {
                return middle;
            }
        }

        panic!("Link not found!");
    }

    pub fn activate(x: f64) -> f64 { 1.0 / (1.0 + (-4.9 * x).exp()) }

    /// evaluate this node
    pub fn eval(&self, net: &Net, evaled_nodes: &mut Vec<(f64, bool)>, inputs: &[f64], innovs: &Vec<Innov>) -> f64 {
        if self.index < net.inputs_count { inputs[self.index] }
        else if self.index == net.inputs_count { 1.0 }
        else {
            let mut sum = 0.0;
            for link_index in &self.in_link_indices {
                let link = &net.links[*link_index];
                if evaled_nodes[self.index].1 {
                    sum += evaled_nodes[innovs[link.innov].from].0 * link.weight;
                }
                else {
                    sum += net.nodes[innovs[link.innov].from].eval(net, evaled_nodes, inputs, innovs) * link.weight;
                }
            }
            
            evaled_nodes[self.index] = (Self::activate(sum), true);

            evaled_nodes[self.index].0
        }
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Self {
            in_link_indices: self.in_link_indices.clone(),
            index: self.index,
        }
    }
}

/// The innovation object. 
pub struct Innov {
    pub from: usize,
    pub to: usize,
    pub number: usize,
}

impl Clone for Innov {
    fn clone(&self) -> Self {
        Self {
            from: self.from,
            to: self.to,
            number: self.number,
        }
    }
}
