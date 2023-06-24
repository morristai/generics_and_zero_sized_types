#![allow(unused)]

use std::collections::HashMap;

struct CredManager {
    master_pass: String,
    tokens: HashMap<String, String>,
    state: CredManagerState,
}

impl CredManager {
    pub fn new(master_pass: String) -> Self {
        CredManager {
            master_pass,
            tokens: Default::default(),
            state: CredManagerState::Locked,
        }
    }

    pub fn lock(&mut self) {
        self.state = CredManagerState::Locked;
    }

    pub fn unlock(&mut self, master_pass: String) {
        if master_pass == self.master_pass {
            self.state = CredManagerState::Unlocked;
        }
    }

    pub fn list_tokens(&self) -> Option<&HashMap<String, String>> {
        match &self.state {
            CredManagerState::Unlocked => Some(&self.tokens),
            CredManagerState::Locked => None,
        }
    }

    pub fn add_token(&mut self, username: String, token: String) {
        if let CredManagerState::Unlocked = self.state {
            self.tokens.insert(username, token);
        }
    }

    pub fn encryption(&self) -> Option<String> {
        match &self.state {
            CredManagerState::Unlocked => Some(todo!()),
            CredManagerState::Locked => None,
        }
    }

    pub fn version(&self) -> Option<String> {
        match &self.state {
            CredManagerState::Unlocked => Some(todo!()),
            CredManagerState::Locked => None,
        }
    }
}

enum CredManagerState {
    Locked,
    Unlocked,
}

// Risk: Users could run into runtime error or unexpected result
fn main() {
    let mut manager = CredManager::new("password123".to_owned());
    manager.add_token("user1".to_owned(), "token1".to_owned());
    manager.unlock("password123".to_owned());
    manager.add_token("user2".to_owned(), "token2".to_owned());
    manager.lock();
    assert_eq!(manager.list_tokens(), None);
}
