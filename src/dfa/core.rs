use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct State {
    pub name: String,
}

impl State {
    pub fn new<S: AsRef<str>>(name: S) -> Rc<State> {
        Rc::new(State {
            name: name.as_ref().to_string(),
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Evaluation {
    Accept(Rc<State>),
    Reject(Rc<State>),
}

pub struct DFA {
    pub transitions: HashMap<(Rc<State>, char), Rc<State>>,
    pub start: Rc<State>,
    pub accept: HashSet<Rc<State>>,
}

impl DFA {
    pub fn r#move(&self, from: Rc<State>, via: char) -> Rc<State> {
        if let Some(to) = self.transitions.get(&(from.clone(), via)) {
            return to.clone();
        }
        from
    }

    pub fn states(&self) -> HashSet<Rc<State>> {
        let x = self.transitions.values().collect::<HashSet<_>>();

        let y = self
            .transitions
            .keys()
            .map(|(state, _)| state)
            .collect::<HashSet<_>>();

        x.union(&y)
            .map(|state| Rc::clone(state))
            .collect::<HashSet<_>>()
    }
}

pub struct DFACursor<'a> {
    pub dfa: &'a DFA,
    pub current: Rc<State>,
}

impl<'a> DFACursor<'a> {
    pub fn run(mut self, stream: impl Iterator<Item = char>) -> Evaluation {
        for c in stream {
            self.current = self.dfa.r#move(self.current.clone(), c);
        }

        match self.dfa.accept.contains(&self.current.clone()) {
            true => Evaluation::Accept(self.current),
            false => Evaluation::Reject(self.current),
        }
    }

    pub fn r#move(&'a mut self, via: char) -> &'a mut DFACursor {
        self.current = self.dfa.r#move(self.current.clone(), via);
        self
    }
}

impl DFA {
    pub fn cursor(&self) -> DFACursor {
        DFACursor {
            dfa: &self,
            current: self.start.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dfa::builder::DFABuilder;
    use crate::dfa::core::Evaluation;
    use crate::state;
    use std::collections::HashSet;

    #[test]
    fn it_works() {
        state!(q0, q1);
        let builder = DFABuilder::new();
        let dfa = builder
            .transition(q0.clone(), '0', q1.clone())
            .transition(q0.clone(), '1', q0.clone())
            .transition(q1.clone(), '0', q0.clone())
            .transition(q1.clone(), '1', q1.clone())
            .accept(q1.clone())
            .start(q0.clone())
            .build();
        assert_eq!(
            dfa.cursor().run("010".chars()),
            Evaluation::Reject(q0.clone())
        );
        assert_eq!(
            dfa.cursor().run("001".chars()),
            Evaluation::Reject(q0.clone())
        );
        assert_eq!(
            dfa.cursor().run("011".chars()),
            Evaluation::Accept(q1.clone())
        );
        assert_eq!(
            dfa.cursor().run("01".chars()),
            Evaluation::Accept(q1.clone())
        );

        assert_eq!(dfa.states(), HashSet::from([q0.clone(), q1.clone()]))
    }
}
