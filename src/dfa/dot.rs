use crate::dfa::core::DFA;
use askama::Template;

impl DFA {
    pub fn dot(&self) -> String {
        let transitions = self
            .transitions
            .iter()
            .map(|((from, via), to)| Transition {
                from: &from.name,
                via: *via,
                to: &to.name,
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
    via: char,
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
