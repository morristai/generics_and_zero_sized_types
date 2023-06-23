#![allow(unused)]

use std::collections::HashMap;

struct PasswordManager {
    master_pass: String,
    passwords: HashMap<String, String>,
}

impl PasswordManager {
    pub fn new(master_pass: String) -> Self {
        PasswordManager {
            master_pass,
            passwords: Default::default(),
        }
    }

    pub fn lock(&mut self) -> PasswordManager {
        PasswordManager {
            master_pass: self.master_pass.clone(),
            passwords: self.passwords.clone(),
        }
    }

    pub fn unlock(&mut self, master_pass: String) -> PasswordManager {
        PasswordManager {
            master_pass: self.master_pass.clone(),
            passwords: self.passwords.clone(),
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

// Risk: Users could run into runtime error or unexpected result
fn main() {
    let mut manager = PasswordManager::new("password123".to_owned());
    manager.list_passwords();
    manager.unlock("password123".to_owned());
    manager.lock();

    // manager.unlock("password123".to_owned());
    // manager.list_passwords();
    // manager.lock();
}
