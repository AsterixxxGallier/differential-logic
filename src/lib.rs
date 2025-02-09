#![allow(unused)]

use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

thread_local! {
    static INDEX_TO_TERM: RefCell<HashMap<usize, Vec<Vec<usize>>>> = RefCell::new(HashMap::new());
    static TERM_TO_INDEX: RefCell<HashMap<usize, HashMap<Vec<usize>, usize>>> = RefCell::new(HashMap::new());
}

fn index_to_term<R>(variables: usize, consumer: impl FnOnce(&Vec<Vec<usize>>) -> R) -> R {
    INDEX_TO_TERM.with(|mut caches| {
        let mut caches = caches.borrow_mut();
        let cache = caches.entry(variables).or_insert_with(|| {
            (1..=variables)
                .flat_map(|len| (0..variables).permutations(len))
                .collect()
        });
        consumer(cache)
    })
}

fn term_to_index<R>(
    variables: usize,
    consumer: impl FnOnce(&HashMap<Vec<usize>, usize>) -> R,
) -> R {
    TERM_TO_INDEX.with(|mut caches| {
        let mut caches = caches.borrow_mut();
        let cache = caches.entry(variables).or_insert_with(|| {
            index_to_term(variables, |index_to_term| {
                index_to_term
                    .iter()
                    .cloned()
                    .enumerate()
                    .map(|(index, term)| (term, index))
                    .collect()
            })
        });
        consumer(cache)
    })
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Machine {
    variables: usize,
    values: Vec<bool>,
}

impl Machine {
    pub fn new(
        variables: usize,
        mut initial_values_producer: impl FnMut(&[usize]) -> bool,
    ) -> Self {
        let mut values = Vec::new();
        index_to_term(variables, |index_to_term| {
            for term in index_to_term {
                let value = initial_values_producer(term.as_slice());
                values.push(value);
            }
        });
        Self { variables, values }
    }

    pub fn all(variables: usize) -> Vec<Self> {
        let mut machines = Vec::new();
        for signature in (0..(1..=variables).map(|k| permutations(variables, k)).sum())
            .map(|_| [false, true].into_iter())
            .multi_cartesian_product()
        {
            let mut term_index = 0;
            machines.push(Self::new(variables, |term| {
                let result = signature[term_index];
                term_index += 1;
                result
            }));
        }
        machines
    }

    pub fn flip(&mut self, variable: usize) {
        term_to_index(self.variables, |term_to_index| {
            index_to_term(self.variables, |index_to_term| {
                self.values[term_to_index[&vec![variable]]] ^= true;
                let terms_to_flip = self
                    .values
                    .iter()
                    .copied()
                    .enumerate()
                    .filter(|&(index, value)| index_to_term[index][0] == variable && value)
                    .map(|(index, _)| index_to_term[index][1..].to_vec())
                    .filter(|term| !term.is_empty())
                    .collect_vec();
                for term in terms_to_flip {
                    self.values[term_to_index[&term]] ^= true;
                }
            });
        });
    }

    pub fn get(&self, variable: usize) -> bool {
        term_to_index(self.variables, |term_to_index| {
            self.values[term_to_index[&vec![variable]]]
        })
    }

    pub fn set(&mut self, variable: usize, value: bool) {
        if self.get(variable) != value {
            self.flip(variable);
        }
    }
}

impl Debug for Machine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        index_to_term(self.variables, |index_to_term| {
            let mut debug_map = f.debug_map();
            for (term, value) in index_to_term.iter().zip(self.values.iter()) {
                debug_map.entry(term, value);
            }
            debug_map.finish()
        })
    }
}

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

fn permutations(n: usize, k: usize) -> usize {
    factorial(n) / factorial(n - k)
}

#[cfg(test)]
mod tests {
    use crate::Machine;
    use hashlink::LinkedHashMap;
    use itertools::Itertools;
    use std::collections::BTreeSet;

    #[test]
    fn print_terms() {
        Machine::new(5, |term| {
            println!("{term:?}");
            true
        });
    }

    #[test]
    fn all() {
        let variables = 3;
        let machines = Machine::all(variables);
        let to_index = machines
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, m)| (m, i))
            .collect::<LinkedHashMap<_, _>>();
        let mut index_map = (0..machines.len()).collect_vec();
        for machine in &machines {
            for variable in 0..variables {
                let mut clone = machine.clone();
                clone.flip(variable);

                let machine_index = to_index[machine];
                let clone_index = to_index[&clone];
                
                let first_index = index_map[machine_index];
                let second_index = index_map[clone_index];

                if machine_index == 16272 {
                    println!("{machine:?}, {variable} => {clone:?} ({first_index}, {second_index})");
                }

                if first_index < second_index {
                    index_map[second_index] = first_index;
                } else if second_index < first_index {
                    index_map[first_index] = second_index;
                }
            }
        }
        let mut reverse_index_map = (0..machines.len()).map(|_| Vec::new()).collect_vec();
        for (machine_index, prime_index) in index_map.iter().copied().enumerate() {
            reverse_index_map[prime_index].push(machine_index);
        }
        let prime_machine_indices = index_map.iter().copied().collect::<BTreeSet<_>>();
        println!("{:?}", prime_machine_indices);
        println!("{:?}", prime_machine_indices.len());
        println!(
            "{:#?}",
            prime_machine_indices
                .iter()
                .rev()
                .min_by_key(|index| reverse_index_map[**index].len())
                .map(|index| (index, reverse_index_map[*index].len(), &machines[*index]))
        );
        // for machine_index in prime_machine_indices {
        //     let machine = &machines[machine_index];
        //     let size = reverse_index_map[machine_index].len();
        //     println!("{:?}", size);
        // }
    }

    #[test]
    fn three() {
        let mut system = Machine::new(3, |term| match term {
            [0] => false,
            [1] => true,
            [2] => true,
            [0, 1] => true,
            [0, 2] => true,
            [1, 0] => true,
            [1, 2] => true,
            [2, 0] => true,
            [2, 1] => false,
            [0, 1, 2] => false,
            [0, 2, 1] => true,
            [1, 0, 2] => false,
            [1, 2, 0] => false,
            [2, 0, 1] => false,
            [2, 1, 0] => false,
            _ => panic!(),
        });
        println!("{:?}", system);
        system.flip(0);
        println!("{:?}", system);
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
