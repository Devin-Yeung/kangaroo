use crate::dfa::core::{State, DFA};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

impl DFA {
    pub fn minimization(&self) -> DFA {
        todo!()
    }

    fn grouping(&self) -> Vec<HashSet<Rc<State>>> {
        let mut groups = Vec::new();
        groups.push(self.accept.clone());
        groups.push(
            self.states()
                .iter()
                .filter(|state| !self.accept.contains(*state))
                .map(Rc::clone)
                .collect(),
        );

        let closures = self.closures();

        loop {
            let mut split: HashSet<Rc<State>> = HashSet::new();

            for group in groups.iter_mut() {
                // Do nothing for a set of single item
                if group.len() <= 1 {
                    continue;
                }

                // find the item to split out
                group.iter().for_each(|state| {
                    if closures.get(state).unwrap().is_subset(group) {
                        split.insert(Rc::clone(state));
                    }
                });

                let origin = group.clone();
                // removing the split out item
                group.retain(|state| !closures.get(state).unwrap().is_subset(&origin));
                break;
            }

            if split.len() == 0 {
                break;
            }
            groups.push(split);
        }

        groups
    }

    fn closures(&self) -> HashMap<Rc<State>, HashSet<Rc<State>>> {
        let mut closures = HashMap::new();

        for ((from, _), to) in self.transitions.iter() {
            if !closures.contains_key(from) {
                closures.insert(from.clone(), HashSet::new());
            }

            closures.get_mut(from).unwrap().insert(to.clone());
        }

        closures
    }
}

#[cfg(test)]
mod tests {
    use crate::dfa;

    #[test]
    fn it_works() {
        let dfa = dfa! {
            state { q1, q2, q3, q4, q5 }

            start { q1 }

            transition {
                q1, 'b' -> q2,
                q1, 'a' -> q3,

                q2, 'b' -> q2,
                q2, 'a' -> q4,

                q3, 'b' -> q4,
                q3, 'a' -> q5,

                q4, 'b' -> q4,
                q4, 'a' -> q5,
            }

            accept { q5 }
        };

        println!("{:?}", dfa.grouping());
    }
}
