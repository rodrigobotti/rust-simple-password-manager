use std::{collections::HashMap, marker::PhantomData};

pub struct Locked;
pub struct Unlocked;

pub struct PasswordManager<State = Locked> {
    master_password: String,
    passwords: HashMap<String, String>,
    // zero-sized type as marker
    // by using a zero-sized type we have compile-time checks without wasting memory with a marker
    // ** mind blown **
    state: PhantomData<State>,
}

// associated functions
impl PasswordManager {
    pub fn new(master_password: String) -> Self {
        PasswordManager {
            master_password,
            passwords: Default::default(),
            state: Default::default(),
        }
    }
}

// methods when in Locked state
impl PasswordManager<Locked> {
    pub fn unlock(self) -> PasswordManager<Unlocked> {
        PasswordManager {
            master_password: self.master_password,
            passwords: self.passwords,
            state: PhantomData::<Unlocked>,
        }
    }
}

// methods when in Unlocked state
impl PasswordManager<Unlocked> {
    pub fn lock(self) -> PasswordManager<Locked> {
        PasswordManager {
            master_password: self.master_password,
            passwords: self.passwords,
            state: PhantomData::<Locked>,
        }
    }

    pub fn add_password(&mut self, username: String, password: String) -> &mut Self {
        self.passwords.insert(username, password);
        self
    }

    pub fn list_passwords(&self) -> &HashMap<String, String> {
        &self.passwords
    }
}

// methods not bound by state
impl<State> PasswordManager<State> {
    pub fn encryption(&self) -> String {
        "plaintext".to_owned()
    }

    pub fn version(&self) -> String {
        "1.0.0".to_owned()
    }
}
