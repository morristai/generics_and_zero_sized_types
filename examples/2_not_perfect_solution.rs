#![allow(unused)]

use std::collections::HashMap;

struct LockedCredManager {
    master_pass: String,
    tokens: HashMap<String, String>,
}

struct UnlockedCredManager {
    master_pass: String,
    tokens: HashMap<String, String>,
}

impl LockedCredManager {
    pub fn new(master_pass: String) -> Self {
        LockedCredManager {
            master_pass,
            tokens: Default::default(),
        }
    }

    pub fn unlock(self, master_pass: String) -> LockedCredManager {
        CredManager {
            master_pass: self.master_pass,
            tokens: self.tokens,
        }
    }

    pub fn encryption(&self) -> String {
        todo!()
    }

    pub fn version(&self) -> String {
        todo!()
    }
}

impl UnlockedCredManager {
    pub fn lock(self) -> LockedCredManager {
        LockedCredManager {
            master_pass: self.master_pass,
            tokens: self.tokens,
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

// Not idea for
// 1. Duplicate fields
// 2. implement common method for both struct
fn main() {
    let mut manager = LockedCredManager::new("password123".to_owned());
    let manager = manager.unlock("password123".to_owned());
    manager.list_tokens();
    manager.lock();
}
