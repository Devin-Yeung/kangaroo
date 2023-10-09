use crate::dfa::core::{State, DFA};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub struct DFABuilder {
    pub transitions: HashMap<(Rc<State>, char), Rc<State>>,
    pub start: Option<Rc<State>>,
    pub accept: HashSet<Rc<State>>,
}

impl DFABuilder {
    pub fn new() -> DFABuilder {
        DFABuilder {
            transitions: Default::default(),
            start: None,
            accept: Default::default(),
        }
    }

    pub fn transition(&mut self, from: Rc<State>, via: char, to: Rc<State>) -> &mut DFABuilder {
        self.transitions.insert((from, via), to);
        self
    }

    pub fn accept(&mut self, state: Rc<State>) -> &mut DFABuilder {
        self.accept.insert(state);
        self
    }

    pub fn start(&mut self, state: Rc<State>) -> &mut DFABuilder {
        self.start = Some(state);
        self
    }

    pub fn build(self) -> DFA {
        DFA {
            transitions: self.transitions,
            start: self.start.unwrap(),
            accept: self.accept,
        }
    }
}
