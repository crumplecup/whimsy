use std::collections::HashSet;

#[derive(Debug, Default, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Identifier {
    counter: Counter,
    name: Name,
    strategy: Strategy,
}

impl Identifier {
    pub fn number(&mut self) -> usize {
        match self.strategy {
            Strategy::Counter => self.counter.inc(),
            _ => self.counter.inc(),
        }
    }

    pub fn name(&mut self) -> String {
        self.name.new_name()
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    strum_macros::EnumIter,
)]
pub enum Strategy {
    #[default]
    Counter,
    Name,
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct Counter(usize);

impl Counter {
    pub fn value(&self) -> usize {
        self.0
    }

    pub fn inc(&mut self) -> usize {
        self.0 += 1;
        self.value()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Name(HashSet<String>);

impl Name {
    pub fn new_name(&mut self) -> String {
        let mut gen = Self::generator();
        loop {
            let name = gen.next().expect("Could not get name from generator.");
            if !self.0.contains(&name) {
                self.0.insert(name.clone());
                return name;
            }
        }
    }

    pub fn new_names(&mut self, count: usize) -> Vec<String> {
        let mut names = Vec::new();
        while names.len() < count {
            names.push(self.new_name())
        }
        names
    }
}

impl<'a> Name {
    pub fn generator() -> names::Generator<'a> {
        names::Generator::default()
    }
}
