// One possible solution is to use the state pattern to manage the lock and unlock states of the CredManager struct.
// The CredManager struct can have two states, Locked and Unlocked, and each state can have its own implementation of the methods.
// The CredManager struct can also have a state field that holds the current state.
// Here's an example implementation of the state pattern for CredManager:

// In this implementation, the CredManager struct has two generic types, S and T, (Where is T????)
// which represent the current state and the next state, respectively. The CredManager struct also has two implementations,
// one for the Locked state and one for the Unlocked state. The Locked state has a panic implementation for all methods,
// while the Unlocked state has the actual implementation for the methods.
//
// The CredManager struct also has two methods, new and unlock, which create a new CredManager instance in the Locked state and transition it to the Unlocked state, respectively.
// The unlock method takes a master password as an argument and returns a new CredManager instance in the Unlocked state if the master password is correct.
//
// The CredManager struct also has a lock method, which transitions the CredManager instance from the Unlocked state to the Locked state.

#![allow(unused)]

use std::collections::HashMap;

trait CredManagerState {
    fn list_tokens(&self) -> &HashMap<String, String>;
    fn add_token(&mut self, username: String, token: String);
    fn encryption(&self) -> String;
    fn version(&self) -> String;
}

struct Locked;

impl CredManagerState for Locked {
    fn list_tokens(&self) -> &HashMap<String, String> {
        panic!("CredManager is locked");
    }

    fn add_token(&mut self, _username: String, _token: String) {
        panic!("CredManager is locked");
    }

    fn encryption(&self) -> String {
        panic!("CredManager is locked");
    }

    fn version(&self) -> String {
        panic!("CredManager is locked");
    }
}

struct Unlocked {
    tokens: HashMap<String, String>,
}

impl CredManagerState for Unlocked {
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

struct CredManager<S: CredManagerState> {
    state: S,
}

impl CredManager<Locked> {
    pub fn new(master_pass: String) -> Self {
        CredManager {
            state: Locked,
        }
    }

    pub fn unlock(self, master_pass: String) -> CredManager<Unlocked> {
        if master_pass == self.master_pass {
            CredManager {
                state: Unlocked {
                    tokens: Default::default(),
                },
            }
        } else {
            panic!("Incorrect master password");
        }
    }
}

impl CredManager<Unlocked> {
    pub fn lock(self) -> CredManager<Locked> {
        CredManager {
            state: Locked,
        }
    }
}

fn main() {
    let manager = CredManager::new("password123".to_owned());
    let unlocked_manager = manager.unlock("password123".to_owned());
    unlocked_manager.list_tokens();
    let locked_manager = unlocked_manager.lock();
}
