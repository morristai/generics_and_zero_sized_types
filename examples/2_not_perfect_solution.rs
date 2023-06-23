#![allow(unused)]

use std::collections::HashMap;

struct LockedPasswordManager {
    master_pass: String,
    passwords: HashMap<String, String>,
}

struct UnlockedPasswordManager {
    master_pass: String,
    passwords: HashMap<String, String>,
}

impl LockedPasswordManager {
    pub fn new(master_pass: String) -> Self {
        LockedPasswordManager {
            master_pass,
            passwords: Default::default(),
        }
    }

    pub fn unlock(self, master_pass: String) -> LockedPasswordManager {
        PasswordManager {
            master_pass: self.master_pass,
            passwords: self.passwords,
        }
    }

    pub fn encryption(&self) -> String {
        todo!()
    }

    pub fn version(&self) -> String {
        todo!()
    }
}

impl UnlockedPasswordManager {
    pub fn lock(self) -> LockedPasswordManager {
        LockedPasswordManager {
            master_pass: self.master_pass,
            passwords: self.passwords,
        }
    }

    pub fn list_passwords(&self) -> &HashMap<String, String> {
        &self.passwords
    }

    pub fn add_password(&mut self, username: String, password: String) {
        self.passwords.insert(username, password);
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
    let mut manager = LockedPasswordManager::new("password123".to_owned());
    let manager = manager.unlock("password123".to_owned());
    manager.list_passwords();
    manager.lock();
}
