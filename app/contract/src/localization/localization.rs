use near_sdk::collections::LookupMap;
use near_sdk::{env, AccountId};
use crate::storage::Storage;

pub struct Localization {
    localized_strings: LookupMap<String, String>,
}

impl Localization {
    pub fn new() -> Self {
        Self {
            localized_strings: LookupMap::new(b"l".to_vec()),
        }
    }

    pub fn set_localized_string(&mut self, key: String, value: String) {
        self.localized_strings.insert(&key, &value);
    }

    pub fn get_localized_string(&self, key: String) -> Option<String> {
        self.localized_strings.get(&key)
    }

    pub fn remove_localized_string(&mut self, key: String) {
        self.localized_strings.remove(&key);
    }
}

pub fn set_localized_string(storage: &mut Storage, key: String, value: String) {
    storage.localization.set_localized_string(key, value);
}

pub fn get_localized_string(storage: &Storage, key: String) -> Option<String> {
    storage.localization.get_localized_string(key)
}

pub fn remove_localized_string(storage: &mut Storage, key: String) {
    storage.localization.remove_localized_string(key);
}