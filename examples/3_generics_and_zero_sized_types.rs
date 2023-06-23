#![allow(unused)]

use std::collections::HashMap;
use std::marker::PhantomData;

struct Locked;
struct Unlocked;

// CredManager<Locked> != CredManager<Unlocked>

struct CredManager<State = Locked> {
    master_pass: String,
    tokens: HashMap<String, String>,
    state: PhantomData<State>,
}

impl CredManager<Locked> {
    pub fn unlock(self, master_pass: String) -> CredManager<Unlocked> {
        CredManager {
            master_pass: self.master_pass,
            tokens: self.tokens,
            state: PhantomData,
        }
    }
}

impl CredManager<Unlocked> {
    pub fn lock(self) -> CredManager<Locked> {
        CredManager {
            master_pass: self.master_pass,
            tokens: self.tokens,
            state: PhantomData,
        }
    }

    pub fn list_tokens(&self) -> &HashMap<String, String> {
        &self.tokens
    }

    pub fn add_token(&mut self, username: String, token: String) {
        self.tokens.insert(username, token);
    }
}

impl<State> CredManager<State> {
    pub fn encryption(&self) -> String {
        todo!()
    }

    pub fn version(&self) -> String {
        todo!()
    }
}

impl CredManager {
    pub fn new(master_pass: String) -> Self {
        CredManager {
            master_pass,
            tokens: Default::default(),
            state: PhantomData,
        }
    }
}

fn main() {
    let mut manager = CredManager::new("password123".to_owned());
    let manager = manager.unlock("password123".to_owned());
    manager.list_tokens();
    manager.lock();
}
