#![allow(unused)]

use std::collections::HashMap;

struct CredManager<State> {
    master_pass: String,
    tokens: HashMap<String, String>,
    _state: std::marker::PhantomData<State>,
}

struct Locked;
struct Unlocked;

impl CredManager<Locked> {
    pub fn new(master_pass: String) -> Self {
        CredManager {
            master_pass,
            tokens: Default::default(),
            _state: std::marker::PhantomData,
        }
    }

    pub fn unlock(self, master_pass: String) -> Result<CredManager<Unlocked>, String> {
        if self.master_pass == master_pass {
            Ok(CredManager {
                master_pass,
                tokens: self.tokens,
                _state: std::marker::PhantomData,
            })
        } else {
            Err("Incorrect master password".to_owned())
        }
    }
}

impl CredManager<Unlocked> {
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

    pub fn lock(self) -> CredManager<Locked> {
        CredManager {
            master_pass: self.master_pass,
            tokens: self.tokens,
            _state: std::marker::PhantomData,
        }
    }
}

fn main() {
    let manager = CredManager::<Locked>::new("password123".to_owned());
    let unlocked_manager = manager.unlock("password123".to_owned()).unwrap();
    let mut unlocked_manager = unlocked_manager.lock();
    unlocked_manager.add_token("user1".to_owned(), "token1".to_owned());
    let tokens = unlocked_manager.list_tokens();
    println!("{:?}", tokens);
}
