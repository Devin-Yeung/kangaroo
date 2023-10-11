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

impl Evaluation {
    pub fn is_accept(&self) -> bool {
        matches!(self, Evaluation::Accept(_))
    }

    pub fn is_reject(&self) -> bool {
        matches!(self, Evaluation::Reject(_))
    }
}
