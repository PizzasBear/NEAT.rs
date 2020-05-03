
#[allow(unused_variables)]
pub trait Conf {
    fn get_excess_coef(&self) -> f64 { 1.0 }
    fn get_disjoint_coef(&self) -> f64 { 1.0 }
    fn get_weight_diff_coef(&self) -> f64 { 0.4 }
    fn get_compat_threshold(&self) -> f64 { 3.0 }
    fn size_norm(&self, size1: usize, size2: usize) -> f64 { 1.0 }

    fn get_weight_mutation_prob(&self) -> f64 { 0.8 }
    fn get_link_addition_mutation_prob(&self) -> f64 { 0.5 }
    fn get_node_addition_mutation_prob(&self) -> f64 { 0.2 }
    fn get_link_disable_mutation_prob(&self) -> f64 { 0.5 }
    fn get_complete_weight_override_prob(&self) -> f64 { 0.1 }

    fn link_enabling_in_child_prob(&self) -> f64 { 0.25 }
    fn get_crossover_prob(&self) -> f64 { 0.75 }

    fn init_weight(&self) -> f64 {
        use rand::{thread_rng, prelude::*};
        use rand_distr::StandardNormal;
        
        StandardNormal{}.sample(&mut thread_rng())
    }
    fn mutate_weight(&self, weight: &mut f64) {
        use rand::{thread_rng, prelude::*};
        use rand_distr::Normal;

        *weight = Normal::new(0.0, 0.5).unwrap().sample(&mut thread_rng());
    }

    fn get_cull_survival_percentage(&self) -> f64 { 0.6 }
    fn get_staleness_threshold(&self) -> u32 { 15 }
}
