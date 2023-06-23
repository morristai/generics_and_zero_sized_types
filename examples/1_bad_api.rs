#![allow(unused)]

use std::collections::HashMap;

struct CredManager {
    master_pass: String,
    tokens: HashMap<String, String>,
}

impl CredManager {
    pub fn new(master_pass: String) -> Self {
        CredManager {
            master_pass,
            tokens: Default::default(),
        }
    }

    pub fn lock(&mut self) -> CredManager {
        CredManager {
            master_pass: self.master_pass.clone(),
            tokens: self.tokens.clone(),
        }
    }

    pub fn unlock(&mut self, master_pass: String) -> CredManager {
        CredManager {
            master_pass: self.master_pass.clone(),
            tokens: self.tokens.clone(),
        }
    }

    pub fn list_tokens(&self) -> &HashMap<String, String> {
        &self.tokens
    }

    pub fn add_token(&mut self, username: String, token: String) {
        self.tokens.insert(username, token);
    }

    pub fn encryption(&self) -> String {
        todo!()
    }

    pub fn version(&self) -> String {
        todo!()
    }
}

// Risk: Users could run into runtime error or unexpected result
fn main() {
    let mut manager = CredManager::new("password123".to_owned());
    manager.list_tokens();
    manager.unlock("password123".to_owned());
    manager.lock();

    // manager.unlock("password123".to_owned());
    // manager.list_tokens();
    // manager.lock();
}
