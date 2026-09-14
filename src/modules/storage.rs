use std::collections::HashMap;

pub struct Storage {
    database: HashMap<String, String>,
}

impl Storage {
    pub fn new() -> Storage {
        Storage { database: HashMap::new() }
    }

    pub fn insert(&mut self, chave: String, valor: String) {
        self.database.insert(chave, valor);
    }

    pub fn select(&self, chave: &str) -> Option<&String> {
        self.database.get(chave)
    }

    pub fn select_by_value(&self, valor: &str) -> Option<&String> {
        for (chave, v) in self.database.iter() {
            if v == valor {
                return Some(chave);
            }
        }
        None
    }
}