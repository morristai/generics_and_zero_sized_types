#![allow(unused)]

use std::collections::HashMap;

struct CredManager {
    state: Box<dyn CredManagerState>,
}

impl CredManager {
    pub fn new(master_pass: String) -> Self {
        CredManager {
            state: Box::new(LockedState::new(master_pass)),
        }
    }

    pub fn lock(&mut self) {
        self.state = self.state.lock();
    }

    pub fn unlock(&mut self, master_pass: String) {
        self.state = self.state.unlock(master_pass);
    }

    pub fn list_tokens(&self) -> &HashMap<String, String> {
        self.state.list_tokens()
    }

    pub fn add_token(&mut self, username: String, token: String) {
        self.state.add_token(username, token);
    }

    pub fn encryption(&self) -> String {
        self.state.encryption()
    }

    pub fn version(&self) -> String {
        self.state.version()
    }
}

trait CredManagerState {
    fn lock(self: Box<Self>) -> Box<dyn CredManagerState>;
    fn unlock(self: Box<Self>, master_pass: String) -> Box<dyn CredManagerState>;
    fn list_tokens(&self) -> &HashMap<String, String>;
    fn add_token(&mut self, username: String, token: String);
    fn encryption(&self) -> String;
    fn version(&self) -> String;
}

struct LockedState {
    master_pass: String,
}

impl LockedState {
    fn new(master_pass: String) -> Self {
        LockedState { master_pass }
    }
}

impl CredManagerState for LockedState {
    fn lock(self: Box<Self>) -> Box<dyn CredManagerState> {
        self
    }

    fn unlock(self: Box<Self>, master_pass: String) -> Box<dyn CredManagerState> {
        if self.master_pass == master_pass {
            Box::new(UnlockedState::new())
        } else {
            // Copilot return type: Box<Box<LockedState>>, which will cause error.
            self
        }
    }

    fn list_tokens(&self) -> &HashMap<String, String> {
        &HashMap::new()
    }

    fn add_token(&mut self, _username: String, _token: String) {
        // do nothing
    }

    fn encryption(&self) -> String {
        todo!()
    }

    fn version(&self) -> String {
        todo!()
    }
}

struct UnlockedState {
    tokens: HashMap<String, String>,
}

impl UnlockedState {
    fn new() -> Self {
        UnlockedState {
            tokens: Default::default(),
        }
    }
}

impl CredManagerState for UnlockedState {
    fn lock(self: Box<Self>) -> Box<dyn CredManagerState> {
        //
        Box::new(LockedState::new(self.master_pass))
    }

    fn unlock(self: Box<Self>, _master_pass: String) -> Box<dyn CredManagerState> {
        self
    }

    fn list_tokens(&self) -> &HashMap<String, String> {
        &self.tokens
    }

    fn add_token(&mut self, username: String, token: String) {
        self.tokens.insert(username, token);
    }

    fn encryption(&self) -> String {
        todo!()
    }

    fn version(&self) -> String {
        todo!()
    }
}

fn main() {
    // let mut manager = CredManager::new("master_pass".to_string());
    // manager.lock();
    // if let Some(tokens) = manager.list_tokens() {
    //     println!("Tokens: {:?}", tokens);
    // } else {
    //     println!("No tokens");
    // }
}
