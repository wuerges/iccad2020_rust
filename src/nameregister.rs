
use std::collections::HashMap;

pub struct NameRegister {
    names : HashMap<usize, String>,
    rev_names : HashMap<String, usize>
}

impl NameRegister {
    pub fn new() -> Self {
        NameRegister {
            names: HashMap::new(),
            rev_names: HashMap::new()
        }
    }
    pub fn register(&mut self, name: String, number: usize) {
        self.names.entry(number).or_insert(name.clone());
        self.rev_names.entry(name).or_insert(number);        
    }

    pub fn resolve_rev(&self, name:String) -> usize {
        self.rev_names[&name]
    }

    pub fn resolve(&self, number:usize) -> String {
        self.names[&number].clone()
    }
}