use crate::dfa::builder::DFABuilder;
use crate::dfa::core::{State, DFA};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

impl DFA {
    pub fn minimization(&self) -> DFA {
        let groups = self.grouping();
        DFA::merge(&self, groups)
    }

    fn merge(dfa: &DFA, groups: Vec<HashSet<Rc<State>>>) -> DFA {
        let mut mapping: HashMap<Rc<State>, Rc<State>> = HashMap::new();
        for group in groups {
            let mut label = group
                .iter()
                .map(|state| state.name.as_str())
                .collect::<Vec<_>>();
            label.sort(); // make grouping result deterministic
            let merged = State::new(label.join(""));
            for state in group {
                mapping.insert(state, merged.clone());
            }
        }

        let mut builder = DFABuilder::new();
        // building transition table
        for ((from, via), to) in &dfa.transitions {
            builder.transition(
                mapping.get(from).unwrap().clone(),
                *via,
                mapping.get(to).unwrap().clone(),
            );
        }
        // building accept state
        dfa.accept.iter().for_each(|state| {
            builder.accept(mapping.get(state).unwrap().clone());
        });

        // building start state
        builder.start(mapping.get(&dfa.start).unwrap().clone());

        builder.build()
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

                if split.len() > 0 {
                    let origin = group.clone();
                    // removing the split out item
                    group.retain(|state| !closures.get(state).unwrap().is_subset(&origin));
                    break;
                }
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
    use crate::dfa::core::DFA;

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

        println!("{:#?}", dfa.grouping());
    }

    #[test]
    fn complicate() {
        let dfa = dfa! {
            state { q0, q1, q2, q3, q4, q5 }

            start { q0 }

            transition {
                q0, '0' -> q1,
                q0, '1' -> q3,

                q1, '0' -> q4,
                q1, '1' -> q2,

                q2, '0' -> q4,
                q2, '1' -> q5,

                q3, '0' -> q1,
                q3, '1' -> q3,

                q4, '0' -> q3,
                q4, '1' -> q4,

                q5, '0' -> q4,
                q5, '1' -> q5,
            }

            accept { q0, q3 }
        };

        println!("{:#?}", dfa.grouping());
        println!("{}", DFA::merge(&dfa, dfa.grouping()).dot());
    }
}
