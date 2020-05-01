pub trait Conf {
    fn get_excess_coef(&self) -> f64 { 1.0 }
    fn get_disjoint_coef(&self) -> f64 { 1.0 }
    fn get_weight_diff_coef(&self) -> f64 { 0.4 }
    fn get_compat_threshold(&self) -> f64 { 3.0 }
    fn size_norm(&self, size1: usize, size2: usize) -> f64 { 1.0 }
    fn get_weight_mutation_prob(&self) -> f64 { 0.8 }
    fn get_link_addition_mutation_prob(&self) -> f64 { 0.05 }
    fn get_node_addition_mutation_prob(&self) -> f64 { 0.03 }
    fn link_enabling_in_child_prob(&self) -> f64 { 0.25 }
    fn get_complete_weight_override_prob(&self) -> f64 { 0.1 }
    fn get_weight_init_range(&self) -> f64 { 1.0 }
    fn is_gaussian_weight_mutation(&self) -> bool { false }
    fn get_gaussian_stddev(&self) -> f64 { 0.0 }
    fn get_uniform_weight_mutation_range(&self) -> f64 { 0.25 }
    fn get_crossover_prob(&self) -> f64 {}
}
