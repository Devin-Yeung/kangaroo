use crate::common::core::State;
use crate::dfa::builder::DFABuilder;
use crate::dfa::core::DFA;
use log::debug;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

impl DFA {
    pub fn minimization(&self) -> DFA {
        let groups = self.grouping();
        DFA::merge(self, groups)
    }

    fn merge(dfa: &DFA, groups: Vec<HashSet<Rc<State>>>) -> DFA {
        if groups.len() <= 2 {
            return dfa.clone();
        }

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
        debug!("Initial Grouping {:#?}", groups);

        let closures = self.closures();
        loop {
            let mut split: HashSet<Rc<State>> = HashSet::new();
            let mut idx = 0usize;

            for (i, group) in groups.iter().enumerate() {
                // Do nothing for a set of single item
                if group.len() <= 1 {
                    continue;
                }

                let mut info = HashMap::<usize, HashSet<Rc<State>>>::new();

                // find the item to split out
                for state in group.iter() {
                    for (idx, group) in groups.iter().enumerate() {
                        if closures.get(state).unwrap().is_subset(group) {
                            info.entry(idx).or_default();
                            info.get_mut(&idx).unwrap().insert(state.clone());
                        }
                    }
                }

                for (_, set) in info {
                    // splitting out all the item in the group is meaningless
                    if set.len() > 1 && group != &set {
                        split = set;
                        idx = i;
                        break;
                    }
                }

                if split.len() > 1 {
                    break;
                }
            }

            if split.len() > 1 {
                debug!("Split out: {:?}", split);
                // avoid dead loop
                if groups.get(idx).unwrap() == &split {
                    continue;
                }
                // removing the split out item
                groups
                    .get_mut(idx)
                    .unwrap()
                    .retain(|state| !split.contains(state));
                groups.push(split);
            } else {
                break;
            }
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

        assert_eq!(
            dfa.minimization(),
            dfa! {
                state { q1q2, q3q4, q5 }

                start { q1q2 }

                transition {
                    q1q2, 'b' -> q1q2,
                    q1q2, 'a' -> q3q4,
                    q3q4, 'a' -> q5,
                    q3q4, 'b' -> q3q4,
                }

                accept { q5 }
            }
        )
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

        assert_eq!(
            dfa.minimization(),
            dfa! {
                state { q1q2q5, q0q3, q4 }

                start { q0q3 }

                transition {
                    q1q2q5 , '0' -> q4,
                    q4     , '1' -> q4,
                    q0q3   , '1' -> q0q3,
                    q1q2q5 , '1' -> q1q2q5,
                    q0q3   , '0' -> q1q2q5,
                    q4     , '0' -> q0q3,
                }

                accept { q0q3 }
            }
        );
    }

    #[test]
    fn very_complex() {
        let dfa = dfa! {
            state { trap, start, zero, number, neg }

            start { start }

            transitions {
                trap, ['-'|'0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'] -> trap,

                start, ['-'] -> neg,
                start, ['0'] -> zero,
                start, ['1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'] -> number,

                number, ['0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'] -> number,
                number, ['-'] -> trap,

                zero, ['-'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'] -> trap,

                neg, ['-'|'0'] -> trap,
                neg, ['1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'] -> number,
            }

            accept { number, zero }
        };

        assert_eq!(dfa.minimization(), dfa);
    }
}
