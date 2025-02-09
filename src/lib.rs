#![allow(unused)]

use itertools::Itertools;
use std::collections::HashMap;
use linked_hash_map::LinkedHashMap;

#[derive(Debug)]
pub struct Machine {
    variables: usize,
    values: LinkedHashMap<Vec<usize>, bool>,
}

impl Machine {
    pub fn new(variables: usize, initial_values_producer: impl Fn(&[usize]) -> bool) -> Self {
        let mut values = LinkedHashMap::new();
        for len in 1..=variables {
            for term in (0..variables).permutations(len) {
                let value = initial_values_producer(&term);
                values.insert(term, value);
            }
        }
        Self { variables, values }
    }

    pub fn all(variables: usize) -> Vec<Self> {
        let mut machines = Vec::new();
        for signature in (0..(1..=variables).map(|k| permutations(variables, k)).sum())
            .map(|_| [false, true].into_iter())
            .multi_cartesian_product()
        {
            let mut values = LinkedHashMap::new();
            let mut term_index = 0;
            for len in 1..=variables {
                for term in (0..variables).permutations(len) {
                    let value = signature[term_index];
                    term_index += 1;
                    values.insert(term, value);
                }
            }
            machines.push(Self { variables, values });
        }
        machines
    }

    pub fn flip(&mut self, variable: usize) {
        *self.values.get_mut(&vec![variable]).unwrap() ^= true;
        let terms_to_flip = self
            .values
            .iter()
            .filter(|(term, value)| term[0] == variable && **value)
            .map(|(term, _)| term[1..].to_vec())
            .filter(|term| !term.is_empty())
            .collect::<Vec<_>>();
        for term in terms_to_flip {
            *self.values.get_mut(&term).unwrap() ^= true;
        }
    }

    pub fn get(&self, variable: usize) -> bool {
        self.values[&vec![variable]]
    }

    pub fn set(&mut self, variable: usize, value: bool) {
        if self.get(variable) != value {
            self.flip(variable);
        }
    }
}

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

fn permutations(n: usize, k: usize) -> usize {
    factorial(n) / factorial(n - k)
}

/*fn index_in_signature(variables: usize, term: &[usize]) -> usize {
    if let &[single] = term {
        single
    } else {
        let mut result = (1..term.len()).map(|k| permutations(variables, k)).sum();
        let mut factor = 1;
        let mut factor_factor = variables;
        let mut pulldowns = Vec::new();
        for &variable in term {
            let mut adjusted = variable - pulldowns.iter().filter(|it| **it < variable).count();
            factor_factor -= 1;
            factor *= factor_factor;
            result += adjusted * factor;
            pulldowns.push(variable);
        }
        result
    }
}*/

#[cfg(test)]
mod tests {
    use crate::Machine;
    
    /*use crate::{index_in_signature, Machine};
    use crate::permutations;

    #[test]
    fn test_permutations() {
        assert_eq!(permutations(3, 0), 1);
        assert_eq!(permutations(3, 1), 3);
        assert_eq!(permutations(3, 2), 6);
        assert_eq!(permutations(3, 3), 6);
    }

    #[test]
    fn test_index_in_signature() {
        // assert_eq!(index_in_signature(2, &[0]), 0);
        // assert_eq!(index_in_signature(2, &[1]), 1);
        // assert_eq!(index_in_signature(2, &[0, 1]), 2);
        // assert_eq!(index_in_signature(2, &[1, 0]), 3);

        assert_eq!(index_in_signature(3, &[0]), 0);
        assert_eq!(index_in_signature(3, &[1]), 1);
        assert_eq!(index_in_signature(3, &[2]), 2);
        assert_eq!(index_in_signature(3, &[0, 1]), 3);
        assert_eq!(index_in_signature(3, &[0, 2]), 4);
        assert_eq!(index_in_signature(3, &[1, 0]), 5);
        assert_eq!(index_in_signature(3, &[1, 2]), 6);
        assert_eq!(index_in_signature(3, &[2, 0]), 7);
        assert_eq!(index_in_signature(3, &[2, 1]), 8);

        /*

         0: 0
         1: 1
        01: 2
        10: 3

         */
    }*/

    #[test]
    fn print_terms() {
        Machine::new(5, |term| {
            println!("{term:?}");
            true
        });
    }
    
    #[test]
    fn all() {
        for machine in Machine::all(2) {
            println!("{:?}", machine);
        }
    }

    #[test]
    fn experimental() {
        let mut system = Machine::new(2, |term| match term {
            [0] => false,
            [1] => false,
            [0, 1] => true,
            [1, 0] => false,
            _ => panic!(),
        });
        assert_eq!(system.get(0), false);
        assert_eq!(system.get(1), false);
        system.set(0, true);
        assert_eq!(system.get(0), true);
        assert_eq!(system.get(1), true);
        system.set(0, false);
        assert_eq!(system.get(0), false);
        assert_eq!(system.get(1), false);
        system.set(1, true);
        assert_eq!(system.get(0), false);
        assert_eq!(system.get(1), true);
        system.set(1, false);
        assert_eq!(system.get(0), false);
        assert_eq!(system.get(1), false);
        system.set(1, true);
        assert_eq!(system.get(0), false);
        assert_eq!(system.get(1), true);
        system.set(0, true);
        assert_eq!(system.get(0), true);
        assert_eq!(system.get(1), false);
        system.set(1, false);
        assert_eq!(system.get(0), true);
        assert_eq!(system.get(1), false);
        system.set(0, false);
        assert_eq!(system.get(0), false);
        assert_eq!(system.get(1), true);
    }

    #[test]
    fn equals() {
        let mut system = Machine::new(2, |term| match term {
            [0] => false,
            [1] => false,
            [0, 1] => true,
            [1, 0] => true,
            _ => panic!(),
        });
        assert_eq!(system.get(0), false);
        assert_eq!(system.get(1), false);
        system.set(0, true);
        assert_eq!(system.get(0), true);
        assert_eq!(system.get(1), true);
        system.set(0, false);
        assert_eq!(system.get(0), false);
        assert_eq!(system.get(1), false);
        system.set(1, true);
        assert_eq!(system.get(0), true);
        assert_eq!(system.get(1), true);
        system.set(1, false);
        assert_eq!(system.get(0), false);
        assert_eq!(system.get(1), false);
    }

    #[test]
    fn not_equals() {
        let mut system = Machine::new(2, |term| match term {
            [0] => false,
            [1] => true,
            [0, 1] => true,
            [1, 0] => true,
            _ => panic!(),
        });
        assert_eq!(system.get(0), false);
        assert_eq!(system.get(1), true);
        system.set(0, true);
        assert_eq!(system.get(0), true);
        assert_eq!(system.get(1), false);
        system.set(0, false);
        assert_eq!(system.get(0), false);
        assert_eq!(system.get(1), true);
        system.set(1, true);
        assert_eq!(system.get(0), false);
        assert_eq!(system.get(1), true);
        system.set(1, false);
        assert_eq!(system.get(0), true);
        assert_eq!(system.get(1), false);
    }
}
