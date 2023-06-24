#![allow(unused)]

use std::collections::HashMap;

struct CredManager {
    master_pass: String,
}

impl CredManager {
    pub fn new(master_pass: String) -> Self {
        CredManager { master_pass }
    }

    pub fn lock(self) -> LockedCredManager {
        LockedCredManager {
            master_pass: self.master_pass,
            tokens: Default::default(),
        }
    }
}

struct LockedCredManager {
    master_pass: String,
    tokens: HashMap<String, String>,
}

impl LockedCredManager {
    pub fn unlock(self, master_pass: String) -> Result<UnlockedCredManager, String> {
        if self.master_pass == master_pass {
            Ok(UnlockedCredManager {
                master_pass,
                tokens: self.tokens,
            })
        } else {
            Err("Incorrect master password".to_owned())
        }
    }
}

struct UnlockedCredManager {
    master_pass: String,
    tokens: HashMap<String, String>,
}

impl UnlockedCredManager {
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

    pub fn lock(self) -> LockedCredManager {
        LockedCredManager {
            master_pass: self.master_pass,
            tokens: self.tokens,
        }
    }
}

fn main() {
    let manager = CredManager::new("password123".to_owned());
    let locked_manager = manager.lock();
    let unlocked_manager = locked_manager.unlock("password123".to_owned()).unwrap();
    let mut unlocked_manager = unlocked_manager.lock();
    unlocked_manager.add_token("user1".to_owned(), "token1".to_owned());
    let tokens = unlocked_manager.list_tokens();
    println!("{:?}", tokens);
}
