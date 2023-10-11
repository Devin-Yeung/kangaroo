use crate::common::core::State;
use crate::dfa::core::DFA;
use askama::Template;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

impl DFA {
    pub fn dot(&self) -> String {
        let mut mapping: HashMap<(Rc<State>, Rc<State>), HashSet<char>> = HashMap::default();

        for ((from, via), to) in &self.transitions {
            let key = (from.clone(), to.clone());
            if !mapping.contains_key(&key) {
                mapping.insert(key.clone(), HashSet::new());
            }

            mapping.get_mut(&key).unwrap().insert(*via);
        }

        let transitions = mapping
            .iter()
            .map(|((from, to), via)| {
                let mut via = via.iter().map(|c| c.to_string()).collect::<Vec<_>>();
                via.sort();
                Transition {
                    from: &from.name,
                    via: via.join(","),
                    to: &to.name,
                }
            })
            .collect::<Vec<_>>();

        let accepts = self
            .accept
            .iter()
            .map(|state| state.name.as_str())
            .collect::<Vec<_>>();

        let template = DFATemplate {
            transitions,
            accepts,
            start: self.start.name.as_str(),
        };

        template.render().unwrap()
    }
}

#[derive(Template)]
#[template(path = "dfa.dot", syntax = "default", escape = "none")]
pub struct DFATemplate<'a> {
    transitions: Vec<Transition<'a>>,
    accepts: Vec<&'a str>,
    start: &'a str,
}

struct Transition<'a> {
    from: &'a str,
    via: String,
    to: &'a str,
}

#[cfg(test)]
mod test {
    use crate::dfa;

    #[test]
    pub fn it_works() {
        let dfa = dfa! {
            state { q0, q1 }

            start { q0 }

            transition {
                q0, '0' -> q1,
                q0, '1' -> q0,
                q1, '0' -> q0,
                q1, '1' -> q1,
            }

            accept { q1 }
        };

        println!("{}", dfa.dot());
    }
}
