#![allow(unused)]

use std::collections::HashMap;
use itertools::Itertools;

pub struct Machine {
    variables: usize,
    values: HashMap<Vec<usize>, bool>,
}

impl Machine {
    pub fn new(variables: usize, initial_values_producer: impl Fn(&[usize]) -> bool) -> Self {
        let mut values = HashMap::new();
        for len in 1..=variables {
            for term in (0..variables).permutations(len) {
                let value = initial_values_producer(&term);
                values.insert(term, value);
            }
        }
        Self {
            variables,
            values,
        }
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

#[cfg(test)]
mod tests {
    use crate::Machine;

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
